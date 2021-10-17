use std::path::PathBuf;
use std::process::exit;
use anyhow::Result;
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use crate::{red, white};

pub fn mods(index: u32) -> Result<()> {
    let home = std::env::var("HOME")?;
    let modpack_dir = PathBuf::from(&home).join("modpacks");
    if !modpack_dir.exists() {
        red!("You don't have any modpacks installed!");
        exit(1);
    }

    let modpack = crate::cache::modpack::Modpack::read_index(&modpack_dir, index)?;
    if let Some(modpack) = modpack {
        white!("Mods in {}:", &modpack.name);
        let modpack_dir = modpack_dir.join(&modpack.name);
        let mods = crate::cache::addon::ModCache::read(&modpack_dir.join("mods"))?;

        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        for m in mods.mods {
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
            write!(&mut stdout, "- ")?;
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;
            write!(&mut stdout, "{} ", &m.name)?;
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
            write!(&mut stdout, "by ")?;
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Blue)))?;
            write!(&mut stdout, "{} ", &m.authors.join(", "))?;
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(173, 173, 173))))?;
            writeln!(&mut stdout, "({})", &m.filename)?;
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
        }
    } else {
        red!("Modpack does not exist! Use `list` to list available modpacks");
        exit(1);
    }

    Ok(())
}