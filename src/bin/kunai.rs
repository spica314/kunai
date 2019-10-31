#[macro_use]
extern crate clap;

use clap::{App, Arg, SubCommand};
use kunai::unify::*;
use kunai::config::*;

fn main() {
    let app = App::new("kunai")
        .version(crate_version!())
        .author(crate_authors!())
        .subcommand(SubCommand::with_name("unify")
            .arg(Arg::with_name("binname")
                .value_name("binname")
                .required(true)
            )
        )
        .subcommand(SubCommand::with_name("crate")
            .subcommand(SubCommand::with_name("add")
                .arg(Arg::with_name("path")
                    .value_name("path")
                    .required(true)
                )
            )
            .subcommand(SubCommand::with_name("remove")
                .arg(Arg::with_name("crate-name")
                    .value_name("crate-name")
                    .required(true)
                )
            )
        );
    let matches = app.clone().get_matches();
    if let Some(unify_matches) = matches.subcommand_matches("unify") {
        let binname = unify_matches.value_of("binname");
        let res = unify(&binname, true);
        println!("{}", res);
    }
    else if let Some(crate_matches) = matches.subcommand_matches("crate") {
        if let Some(crate_add_matches) = crate_matches.subcommand_matches("add") {
            let path = crate_add_matches.value_of("path").unwrap();
            let mut config = read_config();
            config.add_crate(&path);
            write_config(&config);
        }
        else if let Some(crate_remove_matches) = crate_matches.subcommand_matches("remove") {
            let path = crate_remove_matches.value_of("crate-name").unwrap();
            let mut config = read_config();
            config.remove_crate(&path);
            write_config(&config);
        }
        else {
            unimplemented!();
        }
    }
    else {
        let mut app = app;
        app.print_help().ok();
        println!();
    }
}
