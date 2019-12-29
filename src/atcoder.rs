use scraper::{Html, Selector};
use crate::scraping::*;

pub fn get_tests_from_html(html: &str) -> Result<Vec<(String,String)>, ()> {
    let document = Html::parse_document(html);
    let h3_selector = Selector::parse("h3").unwrap();
    let pre_selector = Selector::parse("pre").unwrap();
    let mut inputs = vec![];
    let mut outputs = vec![];
    for elems in continuous_select(&document, &[&h3_selector, &pre_selector]) {
        let h3_text: String = elems[0].text().collect();
        let pre_text: String = elems[1].text().collect();
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
