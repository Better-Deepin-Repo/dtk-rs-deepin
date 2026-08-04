fn main() {
    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());

    // moc 处理 relay.h（Q_OBJECT）
    let moc = ["/usr/lib/qt6/libexec/moc", "/usr/lib/qt6/bin/moc", "moc"]
        .iter()
        .find(|p| std::process::Command::new(p).arg("-v").output().is_ok_and(|o| o.status.success()))
        .expect("找不到 Qt6 moc");
    let moc_out = out_dir.join("moc_relay.cpp");
    let status = std::process::Command::new(moc)
        .args(["include/relay.h", "-o"])
        .arg(&moc_out)
        .status()
        .unwrap();
    assert!(status.success(), "moc 失败");

    let mut build = cxx_build::bridges(["src/lib.rs", "src/gen_ffi.rs"]);
    build
        .file("cpp/shim.cpp")
        .file("cpp/relay.cpp")
        .file("cpp/dtk_gen_shim.cpp")
        .file(&moc_out)
        .include("include")
        .include("compat") // DTK6 打包缺别名头的兼容目录
        .std("c++17")
        .flag_if_supported("-fPIC"); // Qt6 要求 PIC

    // pkg-config 定位 Qt6 + DTK6，include 路径喂给 cxx_build，链接信息由 pkg-config crate 自动吐出
    for pkg in ["Qt6Widgets", "Qt6Gui", "Qt6Core", "dtk6widget", "dtk6gui", "dtk6core"] {
        let lib = pkg_config::Config::new()
            .probe(pkg)
            .unwrap_or_else(|e| panic!("pkg-config {pkg}: {e}"));
        for inc in &lib.include_paths {
            build.include(inc);
        }
        for (k, v) in &lib.defines {
            build.define(k, v.as_deref());
        }
    }

    build.compile("dtkshim");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/gen_ffi.rs");
    println!("cargo:rerun-if-changed=cpp/shim.cpp");
    println!("cargo:rerun-if-changed=cpp/relay.cpp");
    println!("cargo:rerun-if-changed=cpp/dtk_gen_shim.cpp");
    println!("cargo:rerun-if-changed=include/dtk_shim.h");
    println!("cargo:rerun-if-changed=include/dtk_gen_shim.h");
    println!("cargo:rerun-if-changed=include/relay.h");
}
