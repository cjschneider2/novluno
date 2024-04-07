use std::path::PathBuf;

fn main() {
    let target = std::env::var("TARGET").unwrap();
    println!("target: {}", target);
    if target.contains("pc-windows") {
        let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let _out_path = PathBuf::from(out_dir);
        let mut lib_dir = manifest_dir.clone();
        let mut dll_dir = manifest_dir.clone();
        // println!("target is windows");
        // println!("Manifest dir: {:?}", &manifest_dir);

        lib_dir.push("lib");
        dll_dir.push("lib");

        if target.contains("msvc") {
            lib_dir.push("msvc");
            dll_dir.push("msvc");
        } else {
            lib_dir.push("gnu-mingw");
            dll_dir.push("gnu-mingw");
        }

        if target.contains("x86_64") {
            lib_dir.push("64");
            dll_dir.push("64");
        } else {
            lib_dir.push("32");
            dll_dir.push("32");
        }

        println!("cargo:rustc-link-serach=all={}", lib_dir.display());
        //println!("cargo:rustc-link-lib=dynamic=SDL2");
        // for entry in std::fs::read_dir(dll_dir).expect("Can't read DLL dir") {
        //     let entry_path = entry.expect("invalid fs entry").path();
        //     let file_name_result = entry_path.file_name();
        //     let mut new_file_path = out_path.clone();
        //     if let Some(file_name) = file_name_result {
        //         let file_name = file_name.to_str().unwrap();
        //         if file_name.ends_with(".dll") || file_name.ends_with(".lib") {
        //             new_file_path.push(file_name);
        //             std::fs::copy(&entry_path, new_file_path.as_path()).expect("Can't copy from DLL dir");
        //         }
        //     }
        // }
    }
}
