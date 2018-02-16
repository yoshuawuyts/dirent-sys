extern crate bindgen;

use std::process;

fn main() {
  println!("cargo:rustc-link-lib=bcc");
  // Uncomment below to update binding
  build_bindings();
}

fn build_bindings() {
  let mut bindings = bindgen::Builder::default()
    .header("wrapper.h")
    .clang_arg("-I");

  bindings = bindings
    .derive_debug(true)
    .impl_debug(true)
    .derive_default(true)
    .derive_partialeq(true)
    .impl_partialeq(true)
    .derive_eq(true)
    .derive_partialord(true)
    .derive_ord(true)
    .derive_hash(true)
    .rustfmt_bindings(true);

  let builder = bindings.generate().expect("Should generate bindings");

  builder
    .write_to_file("src/dirent.rs")
    .expect("Couldn't write dirent bindings!");

  let have_working_rustfmt = process::Command::new("rustup")
    .args(&["run", "nightly", "rustfmt", "--version"])
    .stdout(process::Stdio::null())
    .stderr(process::Stdio::null())
    .status()
    .ok()
    .map_or(false, |status| status.success());

  if have_working_rustfmt {
    let output = process::Command::new("rustup")
      .args(&[
        "run",
        "nightly",
        "rustfmt",
        "--config-path",
        concat!(env!("CARGO_MANIFEST_DIR"), "/rustfmt.toml"),
        concat!(env!("CARGO_MANIFEST_DIR"), "/src/dirent.rs"),
      ])
      .output()
      .expect("fail to execute `rustup run nightly rustfmt`");
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert!(output.status.success());
  } else {
    println!("
        The latest `rustfmt` is required to format the generated bindings. Install
            `rustfmt` with:
            $ rustup update nightly
            $ rustup run nightly cargo install -f rustfmt-nightly
            ");
  }
}
