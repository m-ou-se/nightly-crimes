extern crate proc_macro;

#[proc_macro]
pub fn do_crimes(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    if !std::env::args_os().any(|arg| arg == "--cfg=yolo_rustc_bootstrap") {
        let version = String::from_utf8(
            std::process::Command::new(std::env::args_os().next().unwrap())
                .arg("--version")
                .output()
                .unwrap()
                .stdout,
        )
        .unwrap();
        let version = version.split(' ').nth(1).unwrap();
        let trick = !version.ends_with("-nightly");

        if trick {
            println!("\x1b[1;32m   Hijacking\x1b[m this rustc process");
            println!("\x1b[1;32m     Abusing\x1b[m proc macros");
            println!("\x1b[1;32m    Enabling\x1b[m the forbidden environment variable");
            println!("\x1b[1;32m    Tricking\x1b[m rustc {}", version);
            println!("\x1b[1;32m  Destroying\x1b[m stability guarantees");
            if let (Ok(c), Ok(v)) = (
                std::env::var("CARGO_PKG_NAME"),
                std::env::var("CARGO_PKG_VERSION"),
            ) {
                println!("\x1b[1;32m Recompiling\x1b[m {} v{}", c, v);
            } else {
                println!("\x1b[1;32m Recompiling\x1b[m your crate");
            }
        }

        let mut args = std::env::args_os();
        let status = std::process::Command::new(args.next().unwrap())
            .arg("--cfg=yolo_rustc_bootstrap")
            .args(args)
            .env("RUSTC_BOOTSTRAP", "1")
            .status()
            .unwrap();

        if trick && status.success() {
            println!("\x1b[1;32m    Finished\x1b[m the dirty work");
            println!("\x1b[1;32m      Hiding\x1b[m all the evidence");
            println!("\x1b[1;32m  Continuing\x1b[m as if nothing happened");
        }

        std::process::exit(status.code().unwrap_or(101));
    }
    Default::default()
}
