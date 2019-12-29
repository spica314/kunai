#[macro_use]
extern crate clap;

use clap::{App, Arg, SubCommand};
use kunai::unify::*;

fn main() {
    let app = App::new("kunai")
        .version(crate_version!())
        .author(crate_authors!())
        .subcommand(
            SubCommand::with_name("unify")
                .arg(
                    Arg::with_name("binname")
                        .value_name("binname")
                        .required(true),
                )
                .arg(Arg::with_name("flag_rust2015").long("rust2015"))
                .arg(Arg::with_name("flag_no_eprint").long("no-eprint")),
        );
    let matches = app.clone().get_matches();
    if let Some(unify_matches) = matches.subcommand_matches("unify") {
        let binname = unify_matches.value_of("binname");
        let rust2015_flag = unify_matches.is_present("flag_rust2815");
        let flag_no_eprint = unify_matches.is_present("flag_no_eprint");
        let res = unify(&binname, rust2015_flag, flag_no_eprint);
        println!("{}", res);
    } else {
        let mut app = app;
        app.print_help().ok();
        println!();
    }
}
