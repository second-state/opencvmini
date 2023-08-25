use os_info::Type;
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn program_exists(program: &str) -> bool {
    if let Ok(paths) = env::var("PATH") {
        for path in paths.split(":") {
            let p_str = format!("{}/{}", path, program);
            if fs::metadata(p_str).is_ok() {
                return true;
            }
        }
    }
    false
}

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated.rs");

    let witc_in_path = program_exists("witc");

    let exe = if !witc_in_path {
        let info = os_info::get();
        let exe = match info.os_type() {
            Type::Ubuntu => "witc-v0.4-ubuntu",
            Type::Macos => "witc-v0.4-macos",
            Type::Windows => "witc-v0.4-windows.exe",
            _ => panic!("Unsupported OS type"),
        };

        println!(
            "Downloading {}",
            format!(
                "https://github.com/second-state/witc/releases/download/v0.4/{}",
                exe
            )
        );

        match info.os_type() {
            Type::Ubuntu | Type::Macos => {
                Command::new("wget")
                    .arg(format!(
                        "https://github.com/second-state/witc/releases/download/v0.4/{}",
                        exe
                    ))
                    .output()
                    .expect("Failed to get executable");
            }
            Type::Windows => {
                let cmd = format!(
                    "Invoke-WebRequest -URI {} -OutFile {}",
                    format!(
                        "https://github.com/second-state/witc/releases/download/v0.4/{}",
                        exe
                    ),
                    exe
                );
                powershell_script::run(&cmd).expect("Failed to get executable");
            }
            _ => panic!("Unsupported OS type"),
        };

        Command::new("chmod")
            .arg("+x")
            .arg(exe)
            .output()
            .expect("Failed to change mode for downloaded executable");

        format!("./{}", exe)
    } else {
        "witc".to_string()
    };

    // witc plugin wasmedge_opencvmini.wit > src/generated.rs
    let output = Command::new(&exe)
        .arg("plugin")
        .arg("wasmedge_opencvmini.wit")
        .output()
        .expect("Failed to execute command");

    println!("status: {}", output.status);

    fs::write(&dest_path, String::from_utf8_lossy(&output.stdout).as_ref()).unwrap();

    if !witc_in_path {
        let _ = Command::new("rm")
            .arg(&exe)
            .output()
            .expect("Failed to get script");
    }

    println!("cargo:rerun-if-changed=build.rs");
}
