use std::{
    env,
    io::Write,
    process::{Command, Stdio},
    sync::atomic::{AtomicUsize, Ordering},
};

const PROBE_HAS_FUTURE: &str = r#"
    #![allow(unused)]
    #![no_std]
    use ::core::future::Future;
"#;
const PROBE_HAS_STD: &str = r#"
    #![allow(unused)]
    #![no_std]
    extern crate std;
"#;
const PROBE_HAS_CORE: &str = r#"
    #![allow(unused)]
    #![no_std]
    extern crate core as _core;
"#;

fn main() {
    if probe(PROBE_HAS_FUTURE).unwrap_or(false) {
        println!("cargo:rustc-cfg=std_future_trait")
    } else if probe(PROBE_HAS_STD).unwrap_or(false) {
        println!("cargo:rustc-cfg=std_crate")
    } else if !probe(PROBE_HAS_CORE).unwrap_or(false) {
        // Treat as "the build script did not run" because `probe` is not
        // working properly in the current environment.
        println!("cargo:warning={}: unable to determine rustc version", env!("CARGO_PKG_NAME"));
        return;
    }

    // Mark as build script has been run successfully.
    // If this is not set and trait is already stable on the latest stable compiler,
    // always prioritize re-export from core.
    // Note: This means that it is likely that older compilers will not be
    //       supported in build systems where build-script cannot be run.
    println!("cargo:rustc-cfg=has_build_script");
}

// https://github.com/cuviper/autocfg/blob/d2c60343b63239dd514622df39172f90463db886/src/lib.rs#L229-L263
fn probe(code: &str) -> Option<bool> {
    static ID: AtomicUsize = AtomicUsize::new(0);

    let rustc = env::var_os("RUSTC")?;
    let out_dir = env::var_os("OUT_DIR")?;
    let target = env::var_os("TARGET");

    let id = ID.fetch_add(1, Ordering::Relaxed);
    let mut cmd = Command::new(rustc);
    cmd.stderr(Stdio::null())
        .arg("--edition=2018")
        .arg("--crate-name")
        .arg(format!("futures_compat_experiment_build{}", id))
        .arg("--crate-type=lib")
        .arg("--out-dir")
        .arg(out_dir)
        .arg("--emit=llvm-ir");

    if let Some(target) = target {
        cmd.arg("--target").arg(target);
    }

    // TODO: handle rustflags

    cmd.arg("-").stdin(Stdio::piped());
    let mut child = cmd.spawn().ok()?;
    let mut stdin = child.stdin.take().expect("rustc stdin");

    stdin.write_all(code.as_bytes()).ok()?;
    drop(stdin);

    let status = child.wait().ok()?;
    Some(status.success())
}
