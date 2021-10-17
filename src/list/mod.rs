use std::path::PathBuf;
use anyhow::Result;
use std::fs;
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use crate::white;

pub fn modpacks() -> Result<()> {
    let home = std::env::var("HOME")?;
    let modpack_dir = PathBuf::from(&home).join("modpacks");

    let mut i = 1;
    white!("Your modpacks:");
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    for f in fs::read_dir(&modpack_dir)? {
        let path = f?.path();
        if !path.is_dir() {
            continue;
        }

        let mut modpack = crate::cache::modpack::Modpack::read(&path)?;
        modpack.update_index(i)?;
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))?;
        write!(&mut stdout, "{}. ", i)?;
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;
        write!(&mut stdout, "{} ", &modpack.name)?;
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
        write!(&mut stdout, "version ")?;
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Blue)))?;
        write!(&mut stdout, "{} ", &modpack.version)?;
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
        write!(&mut stdout, "by ")?;
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Blue)))?;
        writeln!(&mut stdout, "{}", &modpack.author)?;
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;

        i+=1;
    }

    Ok(())
}