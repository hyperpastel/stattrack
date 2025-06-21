use clap_complete::{Shell, generate_to};

include!("src/command.rs");

const SHELLS: [Shell; 5] = [
    Shell::Bash,
    Shell::Zsh,
    Shell::Fish,
    Shell::PowerShell,
    Shell::Elvish,
];

fn main() -> Result<(), std::io::Error> {
    let Some(outdir) = std::env::var_os("OUT_DIR") else {
        return Ok(());
    };

    let mut cmd = build_cli();
    let mut path;
    for shell in SHELLS {
        path = generate_to(shell, &mut cmd, "stattrack", &outdir)?;
        println!("cargo:warning={shell} completion generated at {path:?}");
    }

    Ok(())
}
