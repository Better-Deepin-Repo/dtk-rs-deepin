#!/usr/bin/env python3
"""扫描 DTK6 widget 头文件 → 生成 C++ shim、cxx::bridge、Rust wrapper。

用法: tools/gen.py   (幂等，直接覆盖生成文件)
产物:
  dtk-sys/include/dtk_gen_shim.h   shim 声明
  dtk-sys/cpp/dtk_gen_shim.cpp     shim 实现
  dtk-sys/src/gen_ffi.rs               cxx::bridge
  dtk/src/widgets.rs                   safe wrapper
  GEN_REPORT.md                    覆盖报告（含跳过原因）

规则:
  - 只生成所有参数/返回类型都可映射的方法，其余进报告
  - 信号不生成（DtkRelay 按名字运行时连接，wrapper 已 impl Signal0/SignalI32）
  - 构造：有"全部参数带默认值"的 ctor 就生成 new()
"""
import os
import re
import sys

REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
HDR_DIR = "/usr/include/dtk6/DWidget"

# 手写绑定过的类，生成器跳过
HAND_BOUND = {"DApplication", "DMainWindow", "DTitlebar", "DLabel", "DSuggestButton", "DPushButton"}
# 已在手写 bridge 里声明的 Qt opaque 类型（gen bridge 也要用，需重新声明在自己的桥里）
QT_CLASSES = {"QObject", "QWidget", "QLayout", "QVBoxLayout", "QHBoxLayout", "QTableWidget", "QTimer", "QIcon"}
# Qt widget 基类（判断生成 widget_wrapper 还是 object_wrapper）
QT_WIDGET_BASES = {
    "QWidget", "QMainWindow", "QDialog", "QFrame", "QLabel", "QPushButton", "QAbstractButton",
    "QComboBox", "QLineEdit", "QTextEdit", "QAbstractScrollArea", "QScrollArea", "QListView",
    "QTableView", "QTreeView", "QSlider", "QAbstractSlider", "QSpinBox", "QAbstractSpinBox",
    "QProgressBar", "QTabWidget", "QTabBar", "QMenu", "QMenuBar", "QToolBar", "QStatusBar",
    "QSplitter", "QStackedWidget", "QGroupBox", "QCheckBox", "QRadioButton", "QToolButton",
    "QDateTimeEdit", "QDateEdit", "QTimeEdit", "QCalendarWidget", "QDial", "QLCDNumber",
    "QButtonGroup", "QListWidget", "QTableWidget", "QTreeWidget", "QColumnView",
}

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
    r"(static\s+)?([\w:<>&*~ ]+?)\s+(~?\w+)\s*\((.*)\)\s*(const)?\s*(?:override\s*)?(?:=\s*\w+\s*)?;?\s*(?://.*)?$"
)


RUST_KEYWORDS = {"type", "ref", "self", "mod", "fn", "in", "match", "loop", "move", "crate", "super",
                 "where", "impl", "trait", "const", "static", "mut", "pub", "use", "let", "if", "else",
                 "for", "while", "return", "struct", "enum", "unsafe", "extern", "box", "dyn", "as",
                 "async", "await", "break", "continue", "do", "final", "macro", "override", "priv",
                 "typeof", "unsized", "virtual", "yield", "try", "gen"}


def snake(name: str) -> str:
    return re.sub(r"(?<!^)(?=[A-Z])", "_", name).lower()


def split_params(s: str):
    """按逗号切参数，考虑 <> () 嵌套"""
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
    """类型映射上下文：知道全部 DTK 类名"""

    def __init__(self, classes):
        self.classes = classes  # set of DTK class names

    def map_type(self, cpp: str, is_return: bool):
        """返回 (rust_type, cpp_shim_type, kind, target_class|None) 或 None(不支持)。
        kind: prim | str | ptr"""
        t = cpp.strip()
        t = re.sub(r"\s+", " ", t)
        t = re.sub(r"\bconst\s+", "", t).replace("&", "").strip()
        if "<" in t:
            return None
        ptr = t.endswith("*")
        base = t.rstrip("*").strip()
        base = base.replace("DTK_CORE_NAMESPACE::", "").replace("DTK_GUI_NAMESPACE::", "").replace("::", "_")
        if ptr:
            if base in self.classes:
                return (f"*mut {base}", f"{base} *", "ptr", base)
            # Qt 类只放行 QWidget（其余跨 bridge 类型转换太麻烦，进报告）
            if base == "QWidget":
                return ("*mut QWidget", "QWidget *", "qtptr", base)
            return None
        if base in PRIM:
            r = PRIM[base]
            if r == "()" and not is_return:
                return None
            return (r, CPP_OF_RUST[r], "prim", None)
        if base == "QString" or base == "QByteArray":
            return ("String", CPP_OF_RUST["String"], "str", None) if is_return else ("&str", "rust::Str", "str", None)
        return None


def parse_header(path, ctx):
    """解析单个头文件 → [(class, bases, [method...], report_skip...)]"""
    classes = []
    cur = None
    section = None  # None | 'pub' | 'other'
    with open(path, encoding="utf-8", errors="replace") as f:
        for line in f:
            line = line.rstrip("\n")
            m = CLASS_RE.match(line)
            if m and cur is None:
                bases = []
                if m.group(2):
                    bases = [b.strip().split("::")[-1] for b in re.findall(r"public\s+([\w:]+)", m.group(2))]
                cur = {"name": m.group(1), "bases": bases, "methods": [], "skipped": []}
                section = None
                continue
            if cur is None:
                continue
            if line.startswith("};"):
                classes.append(cur)
                cur = None
                continue
            s = line.strip()
            if re.match(r"^(public|protected|private)\s*(Q_SLOTS|slots)?:", s):
                section = "pub" if s.startswith("public") else "other"
                continue
            if re.match(r"^(Q_)?[Ss][Ii][Gg][Nn][Aa][Ll][Ss]", s) or s.startswith("Q_SIGNALS"):
                section = "other"
                continue
            if section != "pub" or not s:
                continue
            if any(k in s for k in ("Q_PROPERTY", "Q_DECLARE", "D_DECLARE", "typedef", "using ", "enum ", "struct ",
                                    "friend", "operator", "#", "D_DECL_DEPRECATED", "Q_OBJECT")):
                continue
            if "(" not in s:
                continue
            m = METHOD_RE.match(s)
            if not m:
                cur["skipped"].append((s[:80], "签名解析失败"))
                continue
            is_static, ret, name, params = bool(m.group(1)), m.group(2).strip(), m.group(3), m.group(4)
            if name.startswith("~") or name == cur["name"] and not ret:
                continue  # 析构 / 误匹配
            if name == cur["name"]:
                # 构造函数
                ps = split_params(params)
                all_default = all("=" in p for p in ps)
                cur.setdefault("ctor_new", False)
                if not ps or all_default:
                    cur["ctor_new"] = True
                continue
            cur["methods"].append((is_static, ret, name, split_params(params), s[:80]))
    return classes


def gen_method(ctx, cls, is_static, ret, name, params):
    """映射一个方法 → 生成三处代码。失败返回原因字符串"""
    r = ctx.map_type(ret, is_return=True)
    if r is None:
        return None, f"返回类型不支持: {ret}"
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
        q = ctx.map_type(ptype, is_return=False)
        if q is None:
            return None, f"参数类型不支持: {ptype}"
        prs, pcpp, pkind, pcls = q
        if pkind == "str":
            args.append((f"{pname}: &str", f"rust::Str {pname}", f"from_rust_str({pname})", "str", None))
        elif pkind in ("ptr", "qtptr"):
            args.append((f"{pname}: *mut {pcls}", f"{pcpp} {pname}", pname, pkind, pcls))
        else:
            args.append((f"{pname}: {prs}", f"{pcpp} {pname}", pname, "prim", None))
    return (ret_rs, ret_cpp, ret_kind, ret_cls, is_static, name, args), None


def main():
    headers = sorted(
        os.path.join(HDR_DIR, f) for f in os.listdir(HDR_DIR)
        if f.endswith(".h") and not f.endswith("_p.h") and f != "dwidgetstype.h"
    )
    # 先扫类名建上下文
    all_classes = set()
    for h in headers:
        with open(h, encoding="utf-8", errors="replace") as f:
            for line in f:
                m = CLASS_RE.match(line)
                if m:
                    all_classes.add(m.group(1))
    ctx = Ctx(all_classes - HAND_BOUND)

    classes_out = []  # (name, is_widget, ctor_new, [gen'd methods], skipped)
    for h in headers:
        for c in parse_header(h, ctx):
            if c["name"] in HAND_BOUND:
                continue
            c["header"] = os.path.basename(h)
            is_widget = any(b in QT_WIDGET_BASES or (b in all_classes and b != "DObject") for b in c["bases"])
            if not is_widget and c["bases"]:
                # 基类是另一个 DTK 类时跟随基类（DTK 控件居多）
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
                "ctor_new": c.get("ctor_new", False), "methods": gen_methods, "skipped": skipped,
            })

    emit(classes_out)


def emit(classes):
    shim_h, shim_cpp, bridge, wrapper, report = [], [], [], [], []
    shim_h.append("// 自动生成 by tools/gen.py，勿手改\n#pragma once\n#include \"dtk_shim.h\"\n")
    shim_cpp.append('// 自动生成 by tools/gen.py，勿手改\n#include "dtk_gen_shim.h"\n\nnamespace dtkrs {\n')
    bridge.append("// 自动生成 by tools/gen.py，勿手改\n#[cxx::bridge(namespace = \"dtkrs\")]\npub mod genffi {\n    extern \"C++\" {\n        include!(\"dtk_gen_shim.h\");\n        type QWidget;\n")
    wrapper.append("// 自动生成 by tools/gen.py，勿手改\n#![allow(clippy::all, non_snake_case, unused_imports)]\nuse crate::{Signal0, SignalI32, QWidget};\nuse dtk_sys::ffi;\nuse dtk_sys::gen_ffi::genffi;\nuse std::marker::PhantomData;\n")
    report.append("# DTK6 widget binding 覆盖报告\n")

    total_ok, total_skip = 0, 0
    used_headers = sorted({c["header"] for c in classes})
    for h in used_headers:
        shim_h.append(f"#include <{h}>\n")
    shim_h.append("\nnamespace dtkrs {\n")
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
                base_fn += "_"  # 与 wrapper 侧同规则，防 Rust 关键字
            n = used_names.get(base_fn, 0)
            used_names[base_fn] = n + 1
            fn = base_fn if n == 0 else f"{base_fn}_{n + 1}"
            # shim 签名
            self_arg = [] if is_static else [f"{name} *self"]
            cpp_sig = ", ".join(self_arg + [a[1] for a in args])
            call_args = ", ".join(a[2] for a in args)
            call = f"{name}::{meth}({call_args})" if is_static else f"self->{meth}({call_args})"
            if ret_kind == "str":
                call = f"to_rust_string({call})"
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
        # widget_wrapper 宏已提供这些方法，跳过防重复定义
        MACRO_METHODS = {"show", "resize", "set_enabled", "set_window_title", "as_widget", "from_raw", "as_qobject", "new", "default"}
        for (ret_rs, _, ret_kind, ret_cls, is_static, meth, args) in c["methods"]:
            mname = snake(meth)
            if mname in RUST_KEYWORDS:
                mname += "_"
            n = used.get(mname, 0)
            used[mname] = n + 1
            mname = mname if n == 0 else f"{mname}_{n + 1}"
            if mname in MACRO_METHODS:
                continue  # 宏已提供
            fn_name = f"gen_{sname}_{mname}"  # 与 shim/bridge 命名同序同规则            # wrapper 参数: *mut X → &X
            wr_args, call_args = [], []
            for rs, _cpp, _call, pkind, pcls in args:
                pname, ptype = rs.split(": ", 1)
                if pkind == "ptr":
                    wr_args.append(f"{pname}: &{pcls}")
                    call_args.append(f"{pname}.ptr")
                elif pkind == "qtptr":
                    wr_args.append(f"{pname}: &{pcls}")
                    call_args.append(f"{pname}.ptr as _")
                else:
                    wr_args.append(rs)
                    call_args.append(pname)
            self_piece = [] if is_static else ["&self"]
            sig = ", ".join(self_piece + wr_args)
            self_call = [] if is_static else ["self.ptr"]
            calls = ", ".join(self_call + call_args)
            # 返回：指针包成 wrapper，值类型原样
            if ret_kind == "ptr":
                ret_decl = f" -> {ret_cls}"
                expr = f"{ret_cls}::from_raw(unsafe {{ genffi::{fn_name}({calls}) }})"
            elif ret_kind == "qtptr":
                ret_decl = " -> QWidget"
                expr = f"QWidget::from_raw(unsafe {{ genffi::{fn_name}({calls}) }} as _)"
            else:
                ret_decl = "" if ret_rs == "()" else f" -> {ret_rs}"
                expr = f"unsafe {{ genffi::{fn_name}({calls}) }}"
            wrapper.append(f"    pub fn {mname}({sig}){ret_decl} {{\n        {expr}\n    }}\n")
        wrapper.append("}\n\n")
        if c["ctor_new"]:
            wrapper.append(f"impl Default for {name} {{\n    fn default() -> Self {{ Self::new() }}\n}}\n\n")
        total_skip_n = len(c["skipped"])
        total_skip += total_skip_n
        report.append(f"\n## {name} — {len(c['methods'])} 方法已生成, {total_skip_n} 跳过\n")
        for raw, why in c["skipped"]:
            report.append(f"- `{raw}` ← {why}\n")

    report.insert(1, f"\n类: {len(classes)}, 已生成方法: {total_ok}, 跳过: {total_skip}\n")

    write(os.path.join(REPO, "dtk-sys/include/dtk_gen_shim.h"), "".join(shim_h))
    write(os.path.join(REPO, "dtk-sys/cpp/dtk_gen_shim.cpp"), "".join(shim_cpp))
    write(os.path.join(REPO, "dtk-sys/src/gen_ffi.rs"), "".join(bridge))
    write(os.path.join(REPO, "dtk/src/widgets.rs"), "".join(wrapper))
    write(os.path.join(REPO, "GEN_REPORT.md"), "".join(report))
    print(f"类 {len(classes)}, 方法生成 {total_ok}, 跳过 {total_skip}")


def write(path, content):
    with open(path, "w", encoding="utf-8") as f:
        f.write(content)


if __name__ == "__main__":
    main()
