#[macro_use]
extern crate clap;

use clap::{App, Arg, SubCommand};
use kunai::config::*;
use kunai::unify::*;

fn main() {
    let app = App::new("kunai")
        .version(crate_version!())
        .author(crate_authors!())
        .subcommand(
            SubCommand::with_name("unify").arg(
                Arg::with_name("binname")
                    .value_name("binname")
                    .required(true),
            ),
        )
        .subcommand(
            SubCommand::with_name("crate")
                .subcommand(
                    SubCommand::with_name("add")
                        .arg(Arg::with_name("path").value_name("path").required(true)),
                )
                .subcommand(
                    SubCommand::with_name("remove").arg(
                        Arg::with_name("crate-name")
                            .value_name("crate-name")
                            .required(true),
                    ),
                )
                .subcommand(SubCommand::with_name("list")),
        );
    let matches = app.clone().get_matches();
    if let Some(unify_matches) = matches.subcommand_matches("unify") {
        let binname = unify_matches.value_of("binname");
        let res = unify(&binname, true);
        println!("{}", res);
    } else {
        let mut app = app;
        app.print_help().ok();
        println!();
    }
}
