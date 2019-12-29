#[macro_use]
extern crate clap;

use clap::{App, Arg, SubCommand};
use kunai::unify::*;
use std::io::{BufRead, Read, Write};

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
        )
        .subcommand(
            SubCommand::with_name("download")
                .arg(Arg::with_name("url").value_name("url").required(true)),
        )
        .subcommand(SubCommand::with_name("atcoder").subcommand(SubCommand::with_name("login")));
    let matches = app.clone().get_matches();
    if let Some(unify_matches) = matches.subcommand_matches("unify") {
        let binname = unify_matches.value_of("binname");
        let rust2015_flag = unify_matches.is_present("flag_rust2815");
        let flag_no_eprint = unify_matches.is_present("flag_no_eprint");
        let res = unify(&binname, rust2015_flag, flag_no_eprint);
        println!("{}", res);
    } else if let Some(download_matches) = matches.subcommand_matches("download") {
        let url = download_matches.value_of("url").unwrap();
        let problem = kunai::atcoder::ProblemInfo::get(&url);
        for (i, (test_in, test_out)) in problem.tests.iter().enumerate() {
            println!("----- sample_{}.in  -----", i + 1);
            print!("{}", test_in);
            println!("----- sample_{}.out -----", i + 1);
            print!("{}", test_out);
            println!();
        }
        problem.save_tests().unwrap();
        println!("Saved!");
    } else if let Some(atcoder_matches) = matches.subcommand_matches("atcoder") {
        if let Some(login_matches) = atcoder_matches.subcommand_matches("login") {
            let stdin = std::io::stdin();
            let mut iter = stdin.lock().lines();
            print!("username: ");
            std::io::stdout().flush().unwrap();
            let username = iter.next().unwrap().unwrap();
            print!("password: ");
            std::io::stdout().flush().unwrap();
            let password = iter.next().unwrap().unwrap();
            kunai::atcoder::login(&username, &password).unwrap();
            println!("Login Succeeded!")
        } else {
            println!("{}", atcoder_matches.usage());
        }
    } else {
        let mut app = app;
        app.print_help().ok();
        println!();
    }
}
