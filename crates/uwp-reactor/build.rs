use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let manifest = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let root = manifest.join("../..");
    let metadata = root.join("reference/windows-rs/metadata/winrt");
    let output = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    println!(
        "cargo:rerun-if-changed={}",
        manifest.join("projection.txt").display()
    );

    let mut rdl_files = fs::read_dir(&metadata)
        .expect("read windows-rs WinRT metadata")
        .map(|entry| entry.expect("read metadata entry").path())
        .filter(|path| {
            path.extension().is_some_and(|extension| extension == "rdl")
                && path
                    .file_name()
                    .is_some_and(|name| name.to_string_lossy().starts_with("Windows.UI.Xaml"))
        })
        .collect::<Vec<_>>();
    rdl_files.sort();

    assert!(
        !rdl_files.is_empty(),
        "Windows.UI.Xaml RDL metadata is missing"
    );
    for path in &rdl_files {
        println!("cargo:rerun-if-changed={}", path.display());
    }

    let winmd = output.join("Windows.UI.Xaml.winmd");
    windows_rdl::Reader::new()
        .inputs(rdl_files)
        .reference_default()
        .output(&winmd)
        .write()
        .expect("compile Windows.UI.Xaml RDL metadata");

    windows_bindgen::builder()
        .input(&winmd)
        .input_default()
        .output(output.join("windows_ui_xaml.rs"))
        .filter_file(manifest.join("projection.txt"))
        .implements(["Windows.UI.Xaml.IApplicationOverrides"])
        .compose("Windows.UI.Xaml.Application")
        .minimal()
        .flat()
        .write();
}
