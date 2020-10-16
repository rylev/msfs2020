use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rustc-link-lib=SimConnect");
    println!(r#"cargo:rustc-link-search=C:\MSFS SDK\SimConnect SDK\lib"#);

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .clang_args(&["-x", "c++"])
        .whitelist_function("SimConnect_Open")
        .whitelist_function("SimConnect_Close")
        .whitelist_function("SimConnect_MapClientEventToSimEvent")
        .whitelist_function("SimConnect_AddClientEventToNotificationGroup")
        .whitelist_function("SimConnect_SetNotificationGroupPriority")
        .whitelist_function("SimConnect_CallDispatch")
        .whitelist_type("SIMCONNECT_GROUP_PRIORITY_HIGHEST")
        .whitelist_type("SIMCONNECT_RECV")
        .whitelist_type("SIMCONNECT_RECV_ID*")
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
