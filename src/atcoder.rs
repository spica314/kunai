use scraper::{Html, Selector};
use crate::scraping::*;
use std::path::{Path,PathBuf};
use std::io::Write;
use reqwest::{self, ClientBuilder};
use reqwest::header::{HeaderMap,HeaderValue, COOKIE};

pub fn get_tests_from_html(html: &str) -> Result<Vec<(String,String)>, ()> {
    let document = Html::parse_document(html);
    let h3_selector = Selector::parse("h3").unwrap();
    let pre_selector = Selector::parse("pre").unwrap();
    let mut inputs = vec![];
    let mut outputs = vec![];
    for elems in continuous_select(&document, &[&h3_selector, &pre_selector]) {
        let h3_text: String = elems[0].text().collect();
        let mut pre_text: String = elems[1].text().collect();
        if pre_text.chars().last().unwrap() != '\n' {
            pre_text.push('\n');
        }
        if h3_text.contains("入力") {
            inputs.push(pre_text);
        }
        else if h3_text.contains("出力") {
            outputs.push(pre_text);
        }
    }
    if inputs.len() != outputs.len() {
        eprintln!("inputs = {:?}", inputs);
        eprintln!("outputs = {:?}", outputs);
        return Err(());
    }
    let mut res = vec![];
    for i in 0..inputs.len() {
        res.push((inputs[i].clone(), outputs[i].clone()));
    }
    Ok(res)
}

pub fn get_csrf_token_from_html(html: &str) -> Result<String, ()> {
    let document = Html::parse_document(html);
    let csrf_selector = Selector::parse(r#"input[name="csrf_token"]"#).unwrap();
    for elem in document.select(&csrf_selector) {
        if let Some(token) = elem.value().attr("value") {
            return Ok(token.to_string());
        }
    }
    Err(())
}

pub fn path_of_session_cookie_file() -> PathBuf {
    let mut pathbuf = dirs::cache_dir().unwrap();
    pathbuf.push("kunai");
    std::fs::create_dir_all(&pathbuf).unwrap();
    pathbuf.push("atcoder.cookie");
    pathbuf
}

pub fn load_session_cookie() -> Vec<String> {
    let path = path_of_session_cookie_file();
    let s = std::fs::read_to_string(&path).unwrap();
    let mut res = vec![];
    for line in s.lines() {
        if line.contains("=") {
            res.push(line.to_string());
        }
    }
    res
}

pub fn store_session_cookie(cookies: &Vec<String>) {
    let path = path_of_session_cookie_file();
    let mut file = std::fs::File::create(&path).unwrap();
    for cookie in cookies {
        file.write_all(cookie.as_bytes()).unwrap();
        file.write_all("\n".as_bytes()).unwrap();
    }
}

pub fn write_to_file<P: AsRef<Path>>(path: P, s: &str) -> Result<(),()> {
    let mut file = std::fs::File::create(&path).unwrap();
    file.write_all(s.as_bytes()).unwrap();
    Ok(())
}

pub fn get_page(url: &str) -> Result<String, ()> {
    let cookies = load_session_cookie();
    let client = ClientBuilder::new()
        .cookie_store(true)
        .build().unwrap();
    let mut cookie_headers = HeaderMap::new();
    for cookie in cookies {
        cookie_headers.insert(
            COOKIE,
            HeaderValue::from_str(&cookie).unwrap()
        );
    }
    let mut response = client.get(url).headers(cookie_headers).send().unwrap();
    let text = response.text().unwrap();
    let mut cookies = vec![];
    for cookie in response.cookies() {
        cookies.push(format!("{}={}", cookie.name(), cookie.value()).to_string());
    }
    store_session_cookie(&cookies);
    Ok(text)
}

pub fn login(username: &str, password: &str) -> Result<(), ()> {
    let client = ClientBuilder::new()
        .cookie_store(true)
        .build().unwrap();
    let mut login_page_response = client.get("https://atcoder.jp/login").send().unwrap();
    let login_page_text = login_page_response.text().unwrap();
    let mut cookies = vec![];
    for cookie in login_page_response.cookies() {
        cookies.push(format!("{}={}", cookie.name(), cookie.value()).to_string());
    }
    let mut cookie_headers = HeaderMap::new();
    for cookie in cookies {
        cookie_headers.insert(
            COOKIE,
            HeaderValue::from_str(&cookie).unwrap()
        );
    }
    let csrf_token = get_csrf_token_from_html(&login_page_text).unwrap();
    let params = [
        ("username", username),
        ("password", password),
        ("csrf_token", &csrf_token),
    ];
    let res = client.post("https://atcoder.jp/login?continue=https%3A%2F%2Fatcoder.jp%2Fhome").headers(cookie_headers).form(&params).send().unwrap();
    let mut cookies = vec![];
    for cookie in res.cookies() {
        cookies.push(format!("{}={}", cookie.name(), cookie.value()).to_string());
    }
    store_session_cookie(&cookies);
    Ok(())
}

#[derive(Debug)]
pub struct ProblemInfo {
    contest_name: String,
    problem_name: String,
    tests: Vec<(String,String)>,    
}

impl ProblemInfo {
    pub fn get(problem_url: &str) -> ProblemInfo {
        let url = url::Url::parse(problem_url).unwrap();
        let segs: Vec<String> = url.path_segments().unwrap().map(|s|s.to_string()).collect();
        let contest_name = segs[segs.len()-3].clone();
        let problem_name = segs[segs.len()-1].clone();
        let problem_text = get_page(problem_url).unwrap();
        let tests = get_tests_from_html(&problem_text).unwrap();
        ProblemInfo {
            contest_name,
            problem_name,
            tests,
        }
    }
    pub fn save_tests(&self) -> Result<(),()> {
        let mut pathbuf = dirs::cache_dir().unwrap();
        pathbuf.push("kunai");
        pathbuf.push(&self.contest_name);
        pathbuf.push(&self.problem_name);
        std::fs::create_dir_all(&pathbuf).unwrap();
        for (i, (test_in, test_out)) in self.tests.iter().enumerate() {
            pathbuf.push(&format!("sample_{}.in", i + 1));
            write_to_file(&pathbuf, test_in).unwrap();
            pathbuf.pop();
            pathbuf.push(&format!("sample_{}.out", i + 1));
            write_to_file(&pathbuf, test_out).unwrap();
            pathbuf.pop();
        }
        Ok(())
    }
}
