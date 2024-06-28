use std::process::Command;

fn main() {
    const OUT_DIR: &str = "build";
    const C_FILE_DIR: &str = "vips";

    const C_FILES: &[&str] = &["v_text.c", "v_vips_util.c", "v_conversion.c"];

    let _ = std::fs::create_dir(&format!("./{}", OUT_DIR));

    println!("cargo:rustc-link-search=native=subproc/{}", OUT_DIR);
    println!("cargo:rustc-env=LD_LIBRARY_PATH={}", OUT_DIR);

    for file in C_FILES {
        Command::new("bash")
            .arg("-c")
            .arg(&format!(
                "gcc -fPIC -Wall -O2 -shared ./{0}/{1} -g -o ./{2}/lib{3}.so `pkg-config vips --cflags --libs`",
                C_FILE_DIR,
                file,
                OUT_DIR,
                &file[..file.len() - 2]
            ))
            .output()
            .unwrap();

        println!("cargo:rerun-if-changed=natives/{}", file);
        println!("cargo:rustc-link-lib={}", &file[..file.len() - 2]);
    }
}
