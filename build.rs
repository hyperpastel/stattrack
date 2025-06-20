use clap_complete::{generate_to, shells::Zsh};

include!("src/command.rs");

fn main() -> Result<(), std::io::Error> {
    let outdir = match std::env::var_os("OUT_DIR") {
        None => return Ok(()),
        Some(outdir) => outdir,
    };

    let mut cmd = build_cli();
    // TODO Support multiple shells
    let path = generate_to(Zsh, &mut cmd, "stattrack", outdir)?;

    println!("cargo:warning=Zsh completion generated at {path:?}");

    Ok(())
}
