use std::{
    env,
    path::{Path, PathBuf},
};

#[cfg(not(any(feature = "static", feature = "dynamic")))]
compile_error!(r#"Either feature "static" or "dynamic" must be enabled."#);

#[cfg(all(feature = "static", feature = "dynamic"))]
compile_error!(r#"Feature "static" and "dynamic" cannot coexist."#);

fn main() {
    let root_dir = env::var("OUT_DIR").expect("OUT_DIR not found");
    let root = PathBuf::from(&root_dir);

    let includes: Vec<_> = include_paths()
        .map(|path| format!("-I{}", path.display()))
        .collect();
    eprintln!("{:?}", includes);

    set_link();

    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_args(includes)
        .allowlist_function("Mix_.*")
        .allowlist_function("SDL_RWFromFile")
        .allowlist_type("MIX_.*")
        .allowlist_var("MIX_.*")
        .generate_comments(false)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("bindgen builder was invalid");

    bindings
        .write_to_file(root.join("bind.rs"))
        .expect("`src` directory not found");
}

fn include_paths() -> impl Iterator<Item = PathBuf> {
    let vendor_include = if cfg!(feature = "vendor") {
        let root_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not found"));

        // setup vendored
        let repo_path = root_dir.join("SDL_mixer");
        let lib_dir = repo_path.join("build");
        let include_dir = repo_path.join("include");
        if !repo_path.is_dir() {
            build_vendor(repo_path, &root_dir);
        }
        println!("cargo:rustc-link-search={}", lib_dir.display());
        eprintln!("vendored SDL: {}", root_dir.display());
        vec![include_dir]
    } else {
        vec![]
    };
    pkg_config::Config::new()
        .atleast_version("2.0.4")
        .probe("sdl2_mixer")
        .into_iter()
        .flat_map(|sdl2_mixer| sdl2_mixer.include_paths)
        .chain(
            std::env::var("SDL2_MIXER_PATH")
                .map(PathBuf::from)
                .into_iter(),
        )
        .chain(vendor_include.into_iter())
}

fn build_vendor(repo_path: PathBuf, root_dir: &Path) {
    use git2::Repository;
    use std::process::Command;

    let _ = Repository::clone("https://github.com/libsdl-org/SDL_mixer", &repo_path);
    let build_path = repo_path.join("build");
    std::fs::create_dir(&build_path).expect("failed to mkdir build");
    assert!(
        Command::new(repo_path.join("configure"))
            .current_dir(&build_path)
            .args([format!("--prefix={}", root_dir.display())])
            .status()
            .expect("failed to configure SDL")
            .success(),
        "cmake failed"
    );
    assert!(
        Command::new("make")
            .current_dir(&build_path)
            .status()
            .expect("failed to build SDL")
            .success(),
        "build failed"
    );
}

fn set_link() {
    #[cfg(feature = "static")]
    println!("cargo:rustc-link-lib=static=SDL2_mixer");
    #[cfg(feature = "dynamic")]
    println!("cargo:rustc-link-lib=dylib=SDL2_mixer");
}
