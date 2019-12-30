use std::path::PathBuf;
use std::time::{Instant, Duration};
use std::process::{Command, Stdio};
use std::io::{Read,Write};
use glob::glob;

pub fn check_answer(answer: &str, expected: &str) -> Result<(),()> {
    let a_s: Vec<&str> = answer.split_whitespace().collect();
    let e_s: Vec<&str> = expected.split_whitespace().collect();
    if a_s.len() != e_s.len() {
        return Err(());
    }
    for (a,e) in a_s.iter().zip(e_s.iter()) {
        if a.chars().any(|c| c.is_alphabetic()) || e.chars().any(|c| c.is_alphabetic()) ||
           a.len() > 1 && a.chars().nth(0) == Some('0') || e.len() > 1 && e.chars().nth(0) == Some('0') {
            // string
            if a != e {
                return Err(());
            }
        }
        else if a.chars().any(|c| c == '.') || e.chars().any(|c| c == '.') {
            // float
            let a_v = a.parse::<f64>().unwrap();
            let e_v = e.parse::<f64>().unwrap();
            if (a_v - e_v) / e_v > 1e-9 || a_v - e_v > 1e-9 {
                return Err(());
            }
        }
        else {
            // num
            let a_v = a.parse::<i64>().unwrap();
            let e_v = e.parse::<i64>().unwrap();
            if a_v != e_v {
                return Err(());
            }
        }
    }
    Ok(())
}

#[test]
fn test_check_answer_1() {
    let answer   = "1 2 ABC 3.449999999993847";
    let expected = "1 2 ABC 3.45";
    assert!(check_answer(answer, expected).is_ok());
}

#[test]
fn test_check_answer_2() {
    let answer   = "1 2";
    let expected = "1 3";
    assert!(check_answer(answer, expected).is_err());
}

#[test]
fn test_check_answer_3() {
    let answer   = "1 2 ABC";
    let expected = "1 2";
    assert!(check_answer(answer, expected).is_err());
}

#[test]
fn test_check_answer_4() {
    let answer   = "1 2";
    let expected = "1 2 ABC";
    assert!(check_answer(answer, expected).is_err());
}

#[test]
fn test_check_answer_5() {
    let answer   = "1 2 ABC";
    let expected = "1 2 ACD";
    assert!(check_answer(answer, expected).is_err());
}

#[derive(Debug)]
pub enum JudgeResult {
    Accepted,
    WrongAnswer {
        stdout: String,
        expected: String,
        stderr: String,
    },
    TimeLimitExceeded,
}

pub fn judge1(binname: &str, test_in: &str, test_out: &str, timelimit: &Duration) -> JudgeResult {
    let interval = std::time::Duration::from_millis(100);
    let mut child = Command::new(std::env::var("CARGO").unwrap())
        .args(&["run", "-q", "--bin", binname])
        .env("RUST_BACKTRACE", "1")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin
        .as_mut()
        .ok_or("")
        .unwrap()
        .write_all(test_in.as_bytes())
        .unwrap();
    let ins = Instant::now();
    loop {
        std::thread::sleep(interval);
        if ins.elapsed() > *timelimit {
            return JudgeResult::TimeLimitExceeded;
        }
        match child.try_wait() {
            Ok(Some(status)) => {
                let mut buf = vec![];
                child.stdout.as_mut().unwrap().read_to_end(&mut buf).unwrap();
                let out = std::str::from_utf8(&buf).unwrap();
                let mut buf = vec![];
                child.stderr.as_mut().unwrap().read_to_end(&mut buf).unwrap();
                let err = std::str::from_utf8(&buf).unwrap();
                if check_answer(out, test_out).is_ok() {
                    return JudgeResult::Accepted;
                }
                else {
                    return JudgeResult::WrongAnswer {
                        stdout: out.to_string(),
                        expected: test_out.to_string(),
                        stderr: err.to_string(),
                    };
                }
            }
            Ok(None) => {
                continue;
            }
            Err(e) => {
                panic!("{:?}", e);
            }
        }
    }    
}

pub fn judge(binname: &str, testdir: &PathBuf, timelimit: &Duration) -> Vec<JudgeResult> {
    let mut res = vec![];
    for entry in glob(&format!("{}/*.in", testdir.to_str().unwrap())).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let mut path = path.clone();
                let test_in = std::fs::read_to_string(&path).unwrap();
                path.set_extension("out");
                let test_out = std::fs::read_to_string(&path).unwrap();
                let r = judge1(binname, &test_in, &test_out, timelimit);
                res.push(r);
            },
            Err(e) => {
                panic!("{:?}", e);
            },
        }
    }
    res
}
