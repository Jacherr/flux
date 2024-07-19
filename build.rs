use std::env;
use std::fs::read_to_string;
use std::process::Command;

fn main() {
    const OUT_DIR: &str = "build";
    const C_FILE_DIR: &str = "vips";

    const C_FILES: &[&str] = &[
        "flux_v_text.c",
        "flux_v_vips_util.c",
        "flux_v_conversion.c",
        "flux_v_edgedetect.c",
    ];

    let _ = std::fs::create_dir(&format!("./{}", OUT_DIR));
    let workspace = env::var("CARGO_MANIFEST_DIR").unwrap();

    println!("cargo:rustc-link-search=native={}/{}", workspace, OUT_DIR);
    println!("cargo:rustc-env=LD_LIBRARY_PATH={}/{}", workspace, OUT_DIR);

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

        let output = match Command::new("git").args(&["rev-parse", "--short", "HEAD"]).output() {
            Ok(o) => String::from_utf8_lossy(&o.stdout).to_string(),
            Err(_) => "Unknown".to_owned(),
        };
        println!("cargo:rustc-env=FLUX_GIT_HASH={output}");

        let c;
        let version = match read_to_string(format!("{workspace}/Cargo.toml")) {
            Ok(s) => {
                c = s.clone();
                c.split("\n")
                    .find(|x| x.trim().starts_with("version ="))
                    .map(|v| v.split(" ").nth(2))
                    .flatten()
                    .map(|v| &v[1..v.len() - 1])
                    .unwrap_or("Unknown")
            },
            Err(_) => "Unknown",
        };
        println!("cargo:rustc-env=FLUX_VERSION={version}");
    }
}
