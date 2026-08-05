#!/usr/bin/env python3
"""Scan DTK6 widget headers -> generate C++ shim, cxx::bridge, Rust wrappers.

Usage: tools/gen.py   (idempotent, overwrites generated files)
Outputs:
  dtk-sys/include/dtk_gen_shim.h   shim declarations
  dtk-sys/cpp/dtk_gen_shim.cpp     shim implementations
  dtk-sys/src/gen_ffi.rs           cxx::bridge
  dtk/src/widgets.rs               safe wrappers
  GEN_REPORT.md                    coverage report (with skip reasons)

Rules:
  - only generate methods whose param/return types all map cleanly; the rest go to the report
  - signals are not generated (DtkRelay connects by name at runtime; wrappers impl Signal0/SignalI32)
  - constructors: generate new() when a ctor exists with all-default args
"""
import os
import re
import subprocess
import sys

REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
HDR_DIR = "/usr/include/dtk6/DWidget"

# classes bound by hand; the generator skips these
HAND_BOUND = {"DApplication", "DMainWindow", "DTitlebar", "DLabel", "DSuggestButton", "DPushButton"}
# Qt opaque types already declared in the hand-written bridge (the gen bridge must redeclare its own)
QT_CLASSES = {"QObject", "QWidget", "QLayout", "QVBoxLayout", "QHBoxLayout", "QTableWidget", "QTimer", "QIcon"}
# Qt widget base classes (decides widget_wrapper vs object_wrapper)
QT_WIDGET_BASES = {
    "QWidget", "QMainWindow", "QDialog", "QFrame", "QLabel", "QPushButton", "QAbstractButton",
    "QComboBox", "QLineEdit", "QTextEdit", "QAbstractScrollArea", "QScrollArea", "QListView",
    "QTableView", "QTreeView", "QSlider", "QAbstractSlider", "QSpinBox", "QAbstractSpinBox",
    "QProgressBar", "QTabWidget", "QTabBar", "QMenu", "QMenuBar", "QToolBar", "QStatusBar",
    "QSplitter", "QStackedWidget", "QGroupBox", "QCheckBox", "QRadioButton", "QToolButton",
    "QDateTimeEdit", "QDateEdit", "QTimeEdit", "QCalendarWidget", "QDial", "QLCDNumber",
    "QButtonGroup", "QListWidget", "QTableWidget", "QTreeWidget", "QColumnView",
}

VALUE_TYPES = {"QColor", "QSize", "QPoint", "QRect", "QFont", "QPixmap", "QIcon", "QPalette"}
# types that are QFlags in Qt (fromInt/toInt conversion)
QT_QFLAGS = {"Qt::Alignment", "Qt::WindowFlags", "Qt::MouseButtons", "Qt::KeyboardModifiers",
             "Qt::Orientations", "Qt::ItemFlags", "Qt::MatchFlags", "Qt::ApplicationStates",
             "Qt::InputMethodHints", "Qt::DockWidgetAreas", "Qt::ToolBarAreas"}

PRIM = {
    "void": "()", "bool": "bool", "int": "i32", "qint32": "i32", "short": "i16",
    "qint64": "i64", "qlonglong": "i64", "long": "i64",
    "quint32": "u32", "uint": "u32", "qulonglong": "u64", "quint64": "u64", "ulong": "u64",
    "qreal": "f64", "double": "f64", "float": "f32", "qint8": "i8", "quint8": "u8",
}
CPP_OF_RUST = {"()": "void", "bool": "bool", "i32": "int32_t", "i16": "int16_t", "i64": "int64_t",
               "u32": "uint32_t", "u64": "uint64_t", "f64": "double", "f32": "float",
               "i8": "int8_t", "u8": "uint8_t", "String": "rust::String"}

CLASS_RE = re.compile(r"^class\s+LIBDTKWIDGETSHARED_EXPORT\s+(\w+)\s*(?::\s*(.+?))?\s*$")
METHOD_RE = re.compile(
    r"^\s*(?:virtual\s+|Q_INVOKABLE\s+|D_DECL_DEPRECATED\s+|explicit\s+)*"
    r"(static\s+)?([\w:<>&*~ ]+?)\s*(~?\w+)\s*\((.*)\)\s*(const)?\s*(?:override\s*)?(?:=\s*\w+\s*)?;?\s*(?://.*)?$"
)


RUST_KEYWORDS = {"type", "ref", "self", "mod", "fn", "in", "match", "loop", "move", "crate", "super",
                 "where", "impl", "trait", "const", "static", "mut", "pub", "use", "let", "if", "else",
                 "for", "while", "return", "struct", "enum", "unsafe", "extern", "box", "dyn", "as",
                 "async", "await", "break", "continue", "do", "final", "macro", "override", "priv",
                 "typeof", "unsized", "virtual", "yield", "try", "gen"}


def snake(name: str) -> str:
    return re.sub(r"(?<!^)(?=[A-Z])", "_", name).lower()


def split_params(s: str):
    """split params on commas, respecting <> () nesting"""
    out, depth, cur = [], 0, ""
    for ch in s:
        if ch in "<(":
            depth += 1
        elif ch in ">)":
            depth -= 1
        if ch == "," and depth == 0:
            out.append(cur.strip())
            cur = ""
        else:
            cur += ch
    if cur.strip():
        out.append(cur.strip())
    return out


class Ctx:
    """type-mapping context: knows all DTK class names and enums"""

    def __init__(self, classes):
        self.classes = classes  # set of DTK class names
        self.qenums = {}  # "Scope::Enum" -> True (includes non-exported nested classes)
        self.enums = {}  # unqualified enum name -> scope or "?" (ambiguous, disabled)

    def register_enum(self, scope: str, name: str):
        self.qenums[f"{scope}::{name}"] = True
        prev = self.enums.get(name)
        if prev is None:
            self.enums[name] = scope
        elif prev != scope:
            self.enums[name] = "?"  # ambiguous; unqualified use disabled

    def map_type(self, cpp: str, is_return: bool, scope: str | None = None):
        """returns (rust_type, cpp_shim_type, kind, info) or None (unsupported).
        kind: prim | str | ptr | qtptr | val | enum | qtenum | qflags"""
        t = cpp.strip()
        t = re.sub(r"\s+", " ", t)
        t = re.sub(r"\bconst\s+", "", t).replace("&", "").strip()
        if "<" in t:
            return None
        ptr = t.endswith("*")
        base = t.rstrip("*").strip()
        base = base.replace("DTK_CORE_NAMESPACE::", "").replace("DTK_GUI_NAMESPACE::", "")
        # Qt enums / QFlags
        if base.startswith("Qt::"):
            kind = "qflags" if base in QT_QFLAGS else "qtenum"
            return ("i32", "int32_t", kind, base)
        # DTK enums: own class first -> qualified lookup -> unqualified global
        if scope and f"{scope}::{base}" in self.qenums:
            return ("i32", "int32_t", "enum", f"{scope}::{base}")
        if "::" in base:
            qual = base.replace("DTK_WIDGET_NAMESPACE::", "")
            if qual in self.qenums and qual.split("::")[0] in self.classes:
                return ("i32", "int32_t", "enum", qual)
            return None  # non-exported nested-class enums etc; skip
        if base in self.enums and self.enums[base] != "?":
            sc = self.enums[base]
            if sc in self.classes or sc == "Dtk::Widget":
                return ("i32", "int32_t", "enum", f"{sc}::{base}")
            return None
        if ptr:
            if base in self.classes:
                return (f"*mut {base}", f"{base} *", "ptr", base)
            # only QWidget allowed among Qt classes (cross-bridge casts for others are a pain; report)
            if base == "QWidget":
                return ("*mut QWidget", "QWidget *", "qtptr", base)
            return None
        # value types: heap-allocated opaque pointers
        if base in VALUE_TYPES:
            return (f"*mut {base}", f"{base} *", "val", base)
        if base in PRIM:
            r = PRIM[base]
            if r == "()" and not is_return:
                return None
            return (r, CPP_OF_RUST[r], "prim", None)
        if base == "QString" or base == "QByteArray":
            return ("String", CPP_OF_RUST["String"], "str", None) if is_return else ("&str", "rust::Str", "str", None)
        return None


ENUM_RE = re.compile(r"^\s*enum\s+(?:class\s+)?(\w+)")


def parse_header(path, ctx):
    """parse one header -> [(class, bases, [method...], report_skip...)]"""
    classes = []
    cur = None
    nested = None  # non-exported nested class name (methods not generated, enums only registered)
    section = None  # None | 'pub' | 'other'
    with open(path, encoding="utf-8", errors="replace") as f:
        for line in f:
            line = line.rstrip("\n")
            m = CLASS_RE.match(line)
            if m and cur is None and nested is None:
                bases = []
                if m.group(2):
                    bases = [b.strip().split("::")[-1] for b in re.findall(r"public\s+([\w:]+)", m.group(2))]
                cur = {"name": m.group(1), "bases": bases, "methods": [], "skipped": []}
                section = None
                continue
            # non-exported (nested/file-scope) class definition: track scope, do not generate methods
            nm = re.match(r"^\s*(?:class|struct)\s+(\w+)\b[^;{]*$", line)
            if nm and not m and nested is None:
                nested = nm.group(1)
                continue
            if nested is not None:
                if re.match(r"^\s*enum\s+(?:class\s+)?(\w+)", line):
                    ctx.register_enum(nested, re.match(r"^\s*enum\s+(?:class\s+)?(\w+)", line).group(1))
                if line.startswith("};"):
                    nested = None
                continue
            if cur is None:
                em = ENUM_RE.match(line)
                if em:
                    ctx.register_enum("Dtk::Widget", em.group(1))
                continue
            if line.startswith("};"):
                classes.append(cur)
                cur = None
                continue
            s = line.strip()
            em = ENUM_RE.match(s)
            if em and "(" not in s:
                ctx.register_enum(cur["name"], em.group(1))
                continue
            if re.match(r"^(public|protected|private)\s*(Q_SLOTS|slots)?:", s):
                section = "pub" if s.startswith("public") else "other"
                continue
            if re.match(r"^(Q_)?[Ss][Ii][Gg][Nn][Aa][Ll][Ss]", s) or s.startswith("Q_SIGNALS"):
                section = "other"
                continue
            if section != "pub" or not s:
                continue
            if s.startswith(("{", "}", "~")):
                continue  # inline function body / destructor
            if any(k in s for k in ("Q_PROPERTY", "Q_DECLARE", "D_DECLARE", "typedef", "using ", "enum ", "struct ",
                                    "friend", "operator", "#", "D_DECL_DEPRECATED", "Q_OBJECT", "Q_ENUM", "Q_FLAG")):
                continue
            if "(" not in s:
                continue
            if re.search(r"=\s*0\s*;", s):
                cur["abstract"] = True
            s = re.sub(r"\s*Q_DECL_\w+", "", s)  # strip noexcept/override macros
            # constructor: no return type, name == class name
            cm = re.match(rf"^\s*(?:explicit\s+)?{cur['name']}\s*\((.*)\)\s*;?\s*$", s)
            if cm:
                ps = split_params(cm.group(1))
                if not ps or all("=" in p for p in ps):
                    cur["ctor_new"] = True
                continue
            m = METHOD_RE.match(s)
            if not m:
                cur["skipped"].append((s[:80], "signature parse failed"))
                continue
            is_static, ret, name, params = bool(m.group(1)), m.group(2).strip(), m.group(3), m.group(4)
            if name.startswith("~") or name == cur["name"] and not ret:
                continue  # destructor / false match
            if name == cur["name"]:
                # constructor
                ps = split_params(params)
                all_default = all("=" in p for p in ps)
                cur.setdefault("ctor_new", False)
                if not ps or all_default:
                    cur["ctor_new"] = True
                continue
            cur["methods"].append((is_static, ret, name, split_params(params), s[:80]))
    return classes


def gen_method(ctx, cls, is_static, ret, name, params):
    """map one method -> emit code in three places. Returns failure reason string on error"""
    r = ctx.map_type(ret, is_return=True, scope=cls)
    if r is None:
        return None, f"unsupported return type: {ret}"
    ret_rs, ret_cpp, ret_kind, ret_cls = r
    args = []  # (rust_sig_piece, cpp_sig_piece, call_piece)
    for i, p in enumerate(params):
        p_no_default = p.split("=")[0].strip()
        parts = p_no_default.rsplit(" ", 1)
        if len(parts) == 2:
            ptype, pname = parts
        else:
            ptype, pname = parts[0], f"arg{i}"
        pname = pname.replace("&", "").replace("*", "").strip() or f"arg{i}"
        if pname in RUST_KEYWORDS:
            pname += "_"
        q = ctx.map_type(ptype, is_return=False, scope=cls)
        if q is None:
            return None, f"unsupported param type: {ptype}"
        prs, pcpp, pkind, pcls = q
        if pkind == "str":
            args.append((f"{pname}: &str", f"rust::Str {pname}", f"from_rust_str({pname})", "str", None))
        elif pkind in ("ptr", "qtptr"):
            args.append((f"{pname}: *mut {pcls}", f"{pcpp} {pname}", pname, pkind, pcls))
        elif pkind == "val":
            args.append((f"{pname}: *mut {pcls}", f"{pcpp} {pname}", f"*{pname}", "val", pcls))
        elif pkind == "qflags":
            args.append((f"{pname}: i32", f"int32_t {pname}", f"{q[3]}::fromInt({pname})", "qflags", None))
        elif pkind in ("enum", "qtenum"):
            args.append((f"{pname}: i32", f"int32_t {pname}", f"static_cast<{q[3]}>({pname})", pkind, None))
        else:
            args.append((f"{pname}: {prs}", f"{pcpp} {pname}", pname, "prim", None))
    return (ret_rs, ret_cpp, ret_kind, ret_cls, is_static, name, args), None


def main():
    headers = sorted(
        os.path.join(HDR_DIR, f) for f in os.listdir(HDR_DIR)
        if f.endswith(".h") and not f.endswith("_p.h") and f != "dwidgetstype.h"
    )
    # first pass: collect class names to build the context
    all_classes = set()
    for h in headers:
        with open(h, encoding="utf-8", errors="replace") as f:
            for line in f:
                m = CLASS_RE.match(line)
                if m:
                    all_classes.add(m.group(1))
    ctx = Ctx(all_classes - HAND_BOUND)

    # two passes: parse everything (collecting cross-class enum refs), then generate
    parsed = []
    for h in headers:
        for c in parse_header(h, ctx):
            if c["name"] in HAND_BOUND:
                continue
            c["header"] = os.path.basename(h)
            parsed.append(c)

    classes_out = []
    for c in parsed:
            is_widget = any(b in QT_WIDGET_BASES or (b in all_classes and b != "DObject") for b in c["bases"])
            if not is_widget and c["bases"]:
                # if a base is another DTK class, follow it (most DTK classes are widgets)
                is_widget = any(b.startswith(("Q", "D")) and b != "DObject" for b in c["bases"])
            gen_methods = []
            skipped = c["skipped"]
            for is_static, ret, name, params, raw in c["methods"]:
                g, why = gen_method(ctx, c["name"], is_static, ret, name, params)
                if g is None:
                    skipped.append((raw, why))
                else:
                    gen_methods.append(g)
            classes_out.append({
                "name": c["name"], "is_widget": is_widget, "header": c["header"],
                "ctor_new": c.get("ctor_new", False) and not c.get("abstract", False), "methods": gen_methods, "skipped": skipped,
            })

    emit(classes_out)


def emit(classes):
    shim_h, shim_cpp, bridge, wrapper, report = [], [], [], [], []
    shim_h.append("// auto-generated by tools/gen.py, do not edit\n#pragma once\n#include \"dtk_shim.h\"\n")
    shim_cpp.append('// auto-generated by tools/gen.py, do not edit\n#include "dtk_gen_shim.h"\n\nnamespace dtkrs {\n')
    bridge.append("// auto-generated by tools/gen.py, do not edit\n#[cxx::bridge(namespace = \"dtkrs\")]\npub mod genffi {\n    extern \"C++\" {\n        include!(\"dtk_gen_shim.h\");\n        type QWidget;\n")
    for vt in sorted(VALUE_TYPES):
        bridge.append(f"        type {vt};\n")
    wrapper.append("// auto-generated by tools/gen.py, do not edit\n#![allow(clippy::all, non_snake_case, unused_imports)]\nuse crate::{Signal0, SignalI32, QWidget};\nuse crate::{QColor, QFont, QIcon, QPalette, QPixmap, QPoint, QRect, QSize};\nuse dtk_sys::ffi;\nuse dtk_sys::gen_ffi::genffi;\nuse std::marker::PhantomData;\n")
    report.append("# DTK6 widget binding coverage report\n")

    total_ok, total_skip = 0, 0
    used_headers = sorted({c["header"] for c in classes})
    for h in used_headers:
        shim_h.append(f"#include <{h}>\n")
    shim_h.append("\nnamespace dtkrs {\n")
    for vt in sorted(VALUE_TYPES):
        shim_h.append(f"using ::{vt};\n")
    for c in classes:
        name = c["name"]
        shim_h.append(f"using {name} = Dtk::Widget::{name};\n")
        bridge.append(f"        type {name};\n")

    bridge.append("\n")
    for c in classes:
        name, sname = c["name"], snake(c["name"])
        used_names = {}
        # ctor
        if c["ctor_new"]:
            fn = f"gen_{sname}_new"
            shim_h.append(f"{name} *{fn}();\n")
            shim_cpp.append(f"{name} *{fn}() {{ return new {name}; }}\n")
            bridge.append(f"        unsafe fn {fn}() -> *mut {name};\n")
        for (ret_rs, ret_cpp, ret_kind, ret_cls, is_static, meth, args) in c["methods"]:
            base_fn = f"gen_{sname}_{snake(meth)}"
            if snake(meth) in RUST_KEYWORDS:
                base_fn += "_"  # same rule as the wrapper side, avoids Rust keywords
            n = used_names.get(base_fn, 0)
            used_names[base_fn] = n + 1
            fn = base_fn if n == 0 else f"{base_fn}_{n + 1}"
            # shim signature
            self_arg = [] if is_static else [f"{name} *self"]
            cpp_sig = ", ".join(self_arg + [a[1] for a in args])
            call_args = ", ".join(a[2] for a in args)
            call = f"{name}::{meth}({call_args})" if is_static else f"self->{meth}({call_args})"
            if ret_kind == "str":
                call = f"to_rust_string({call})"
            elif ret_kind == "val":
                call = f"new {ret_cls}({call})"
            elif ret_kind in ("enum", "qtenum"):
                call = f"static_cast<int32_t>({call})"
            elif ret_kind == "qflags":
                call = f"({call}).toInt()"
            body = f"return {call};" if ret_rs != "()" else f"{call};"
            shim_h.append(f"{ret_cpp} {fn}({cpp_sig});\n")
            shim_cpp.append(f"{ret_cpp} {fn}({cpp_sig}) {{ {body} }}\n")
            # bridge
            rust_self = [] if is_static else [f"self_: *mut {name}"]
            rust_args = ", ".join(rust_self + [a[0] for a in args])
            ret_decl = "" if ret_rs == "()" else f" -> {ret_rs}"
            bridge.append(f"        unsafe fn {fn}({rust_args}){ret_decl};\n")
            total_ok += 1

    shim_h.append("\n} // namespace dtkrs\n")
    shim_cpp.append("\n} // namespace dtkrs\n")
    bridge.append("    }\n}\n")

    # ---- Rust wrapper ----
    for c in classes:
        name, sname = c["name"], snake(c["name"])
        mac = "widget_wrapper" if c["is_widget"] else "object_wrapper"
        wrapper.append(f"{mac}!({name}, genffi::{name});\nimpl {name} {{\n")
        if c["ctor_new"]:
            wrapper.append(f"    pub fn new() -> Self {{\n        Self::from_raw(unsafe {{ genffi::gen_{sname}_new() }})\n    }}\n")
        used = {}
        # widget_wrapper macro already provides these; skip to avoid duplicate definitions
        MACRO_METHODS = {"show", "resize", "set_enabled", "set_window_title", "as_widget", "from_raw", "as_qobject", "new", "default"}
        for (ret_rs, _, ret_kind, ret_cls, is_static, meth, args) in c["methods"]:
            mname = snake(meth)
            if mname in RUST_KEYWORDS:
                mname += "_"
            n = used.get(mname, 0)
            used[mname] = n + 1
            mname = mname if n == 0 else f"{mname}_{n + 1}"
            if mname in MACRO_METHODS:
                continue  # already provided by the macro
            fn_name = f"gen_{sname}_{mname}"  # same naming order/rule as shim/bridge
            # wrapper params: *mut X -> &X
            wr_args, call_args = [], []
            for rs, _cpp, _call, pkind, pcls in args:
                pname, ptype = rs.split(": ", 1)
                if pkind == "ptr":
                    wr_args.append(f"{pname}: &{pcls}")
                    call_args.append(f"{pname}.ptr")
                elif pkind == "qtptr":
                    wr_args.append(f"{pname}: &{pcls}")
                    # AUDIT(2026-02): casting between ffi::QWidget and genffi::QWidget is sound.
                    # cxx opaque C++ types are zero-sized Rust structs that are never
                    # dereferenced in Rust; both name the same C++ type ::QWidget via
                    # `using ::QWidget;` in namespace dtkrs (dtk_shim.h / dtk_gen_shim.h).
                    # A raw pointer cast moves no bits and assumes no layout.
                    call_args.append(f"{pname}.ptr as _")
                elif pkind == "val":
                    wr_args.append(f"{pname}: &{pcls}")
                    call_args.append(f"{pname}.ptr as _")  # same audit as qtptr above
                else:
                    wr_args.append(rs)
                    call_args.append(pname)
            self_piece = [] if is_static else ["&self"]
            sig = ", ".join(self_piece + wr_args)
            self_call = [] if is_static else ["self.ptr"]
            calls = ", ".join(self_call + call_args)
            # returns: wrap pointers in wrappers, pass values through
            if ret_kind == "ptr":
                ret_decl = f" -> {ret_cls}"
                expr = f"{ret_cls}::from_raw(unsafe {{ genffi::{fn_name}({calls}) }})"
            elif ret_kind == "qtptr":
                ret_decl = " -> QWidget"
                expr = f"QWidget::from_raw(unsafe {{ genffi::{fn_name}({calls}) }} as _)"  # see qtptr audit comment above
            elif ret_kind == "val":
                ret_decl = f" -> {ret_cls}"
                expr = f"{ret_cls}::from_raw(unsafe {{ genffi::{fn_name}({calls}) }} as _)"  # see qtptr audit comment above
            else:
                ret_decl = "" if ret_rs == "()" else f" -> {ret_rs}"
                expr = f"unsafe {{ genffi::{fn_name}({calls}) }}"
            wrapper.append(f"    pub fn {mname}({sig}){ret_decl} {{\n        {expr}\n    }}\n")
        wrapper.append("}\n\n")
        if c["ctor_new"]:
            wrapper.append(f"impl Default for {name} {{\n    fn default() -> Self {{ Self::new() }}\n}}\n\n")
        total_skip_n = len(c["skipped"])
        total_skip += total_skip_n
        report.append(f"\n## {name} — {len(c['methods'])} methods generated, {total_skip_n} skipped\n")
        for raw, why in c["skipped"]:
            report.append(f"- `{raw}` ← {why}\n")

    report.insert(1, f"\nclasses: {len(classes)}, methods generated: {total_ok}, skipped: {total_skip}\n")

    write(os.path.join(REPO, "dtk-sys/include/dtk_gen_shim.h"), "".join(shim_h))
    write(os.path.join(REPO, "dtk-sys/cpp/dtk_gen_shim.cpp"), "".join(shim_cpp))
    write(os.path.join(REPO, "dtk-sys/src/gen_ffi.rs"), "".join(bridge))
    write(os.path.join(REPO, "dtk/src/widgets.rs"), "".join(wrapper))
    write(os.path.join(REPO, "GEN_REPORT.md"), "".join(report))
    print(f"classes {len(classes)}, methods generated {total_ok}, skipped {total_skip}")
    run_cargo_fmt()


def write(path, content):
    with open(path, "w", encoding="utf-8") as f:
        f.write(content)


def run_cargo_fmt():
    """Format the generated Rust after writing files."""
    subprocess.run(["cargo", "fmt"], cwd=REPO, check=True)
    print("cargo fmt done")


if __name__ == "__main__":
    main()
