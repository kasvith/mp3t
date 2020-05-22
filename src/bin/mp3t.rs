use clap::{crate_version, App, AppSettings, Arg};
use mp3t::commands;

fn run_app() {
    let matches = App::new("mp3t")
        .author("Kasun Vithanage <alankasun@gmail.com>")
        .about("View/Edit/Export ID3v1 tags from mp3 files")
        .version(crate_version!())
        .subcommand(
            App::new("edit")
                .about("edits ID3V1 tags of a mp3 file")
                .arg(
                    Arg::with_name("file")
                        .index(1)
                        .help("file to be edited")
                        .required(true),
                ),
        )
        .subcommand(
            App::new("view").about("view ID3V1 tags of a mp3 file").arg(
                Arg::with_name("file")
                    .index(1)
                    .help("file to view tags")
                    .required(true),
            ),
        )
        .subcommand(
            App::new("export")
                .about("export ID3V1 tags of a mp3 file/files")
                .arg(
                    Arg::with_name("path")
                        .short("p")
                        .help("path to file or files")
                        .default_value("."),
                )
                .arg(
                    Arg::with_name("output")
                        .short("o")
                        .default_value("output.json")
                        .help("output path"),
                )
                .arg(
                    Arg::with_name("format")
                        .short("f")
                        .possible_values(&["json", "yaml"])
                        .default_value("json")
                        .help("output format"),
                ),
        )
        .setting(AppSettings::ArgRequiredElseHelp)
        .get_matches();

    match matches.subcommand() {
        ("view", Some(cmd)) => {
            let file = cmd.value_of("file").unwrap();
            match commands::view::view(&file) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        _ => {}
    }
}

fn main() {
    run_app();
}
