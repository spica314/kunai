#[macro_use]
extern crate clap;

use clap::{App, Arg, SubCommand};
use kunai::unify::*;
use std::io::{BufRead, Write};

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
        .subcommand(SubCommand::with_name("atcoder").subcommand(SubCommand::with_name("login")))
        .subcommand(
            SubCommand::with_name("test")
                .arg(
                    Arg::with_name("testcase_dir")
                        .value_name("testcase-dir")
                        .required(true),
                )
                .arg(
                    Arg::with_name("binname")
                        .value_name("binname")
                        .required(true),
                )
                .arg(Arg::with_name("show-stderr").long("show-stderr")),
        );
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
        if let Some(_login_matches) = atcoder_matches.subcommand_matches("login") {
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
    } else if let Some(test_matches) = matches.subcommand_matches("test") {
        let testcase_dir = test_matches.value_of("testcase_dir").unwrap();
        let binname = test_matches.value_of("binname").unwrap();
        let show_stderr = test_matches.is_present("show-stderr");
        let path = std::path::PathBuf::from(&format!(
            "{}/kunai/{}",
            dirs::cache_dir().unwrap().to_str().unwrap(),
            testcase_dir
        ));
        let res = kunai::judge::judge(binname, &path, &std::time::Duration::from_secs(2));
        let mut ok = true;
        use termion::{color, style};
        let green_bold = |s: &str| {
            format!(
                "{}{}{}{}{}",
                style::Bold,
                color::Fg(color::LightGreen),
                s,
                color::Fg(color::Reset),
                style::Reset
            )
        };
        let yellow_bold = |s: &str| {
            format!(
                "{}{}{}{}{}",
                style::Bold,
                color::Fg(color::LightYellow),
                s,
                color::Fg(color::Reset),
                style::Reset
            )
        };
        for (r, testname) in &res {
            use kunai::judge::JudgeResult;
            match r {
                JudgeResult::Accepted => {
                    println!("{}: {}", testname, green_bold("Accepted"));
                }
                JudgeResult::WrongAnswer {
                    stdout,
                    expected,
                    stderr,
                } => {
                    ok = false;
                    println!("{}: {}", testname, yellow_bold("WrongAnswer"));
                    println!("------  stdout  ------");
                    println!("{}", stdout);
                    println!("------ expected ------");
                    println!("{}", expected);
                    if show_stderr {
                        println!("------  stderr  ------");
                        println!("{}", stderr);
                    }
                }
                JudgeResult::TimeLimitExceeded => {
                    ok = false;
                    println!("{}: {}", testname, yellow_bold("TimeLimitExceeded"));
                }
            }
        }
        if ok {
            println!("\n{}", green_bold("Sample Test Succeded"));
        } else {
            println!("\n{}", yellow_bold("Sample Test Failed"));
        }
    } else {
        let mut app = app;
        app.print_help().ok();
        println!();
    }
}
