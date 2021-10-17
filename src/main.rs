#![deny(rust_2018_idioms)]
#![deny(deprecated)]
#![deny(clippy::pedantic)]

use std::path::Path;
use std::process::exit;

use lazy_static::lazy_static;
use reqwest::blocking::Client;

mod cache;
mod install;
mod list;
mod mods;
mod term;
mod run;

lazy_static! {
    pub static ref CLIENT: Client = Client::new();
}

fn main() {
    let matches = app().get_matches();

    match matches.subcommand() {
        ("install", Some(matches)) => {
            let zip = matches.value_of("zip").expect("Missing required value of 'manifest'");
            let skip_optional = matches.is_present("skip-optional");

            let zip_path = Path::new(zip);
            if !zip_path.exists() {
                red!("ZIP file '{}' does not exist.", zip);
                exit(1);
            }

            install::install(&zip_path, skip_optional).expect("Failed to install modpack!");
        },
        ("list", Some(_)) => {
            list::modpacks().expect("Failed to list modpacks");
        },
        ("mods", Some(matches)) => {
            let index = matches.value_of("index").expect("Missing required argument 'index'");

            if let Ok(p) = index.parse() {
                mods::mods(p).expect("Failed to list mods");
            } else {
                red!("Unable to parse index, is it a valid number?");
                exit(1);
            }
        },
        ("run", Some(matches)) => {
            let index = matches.value_of("index").expect("Missing required argument 'index'");

            if let Ok(p) = index.parse() {
                run::run(p).expect("Failed to run Minecraft launcher");
            } else {
                red!("Unable to parse index, is it a valid number?");
                exit(1);
            }
        }
        _ => {}
    }
}

fn app() -> clap::App<'static, 'static> {
    use clap::{Arg, App};

    App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .subcommand(App::new("install")
            .about("Install a new modpack")
            .arg(Arg::with_name("zip")
                .takes_value(true)
                .required(true)
                .value_name("path to modpack ZIP")
                .help("The path to the CurseForge modpack ZIP"))
            .arg(Arg::with_name("skip-optional")
                .takes_value(false)
                .short("o")
                .long("skip-optional")
                .help("Should optional mods be skipped")))
        .subcommand(App::new("list")
            .about("List the modpacks you have installed"))
        .subcommand(App::new("mods")
            .about("List the mods installed in a modpack")
            .arg(Arg::with_name("index")
                .help("The index of the modpack. Use `list` to see what index you should use")
                .takes_value(true)
                .required(true)
                .value_name("modpack index")))
        .subcommand(App::new("run")
            .about("Run a modpack")
            .arg(Arg::with_name("index")
                .help("The index of the modpack. Use `list` to see what index you should use")
                .takes_value(true)
                .required(true)
                .value_name("modpack index")))
}