use scraper::{Html, Selector};
use std::collections::HashMap;

pub fn continuous_select<'a>(
    document: &'a Html,
    selectors: &[&Selector],
) -> Vec<Vec<scraper::element_ref::ElementRef<'a>>> {
    if selectors.is_empty() {
        return vec![];
    }
    let mut set = HashMap::new();
    for elem in document.select(&selectors[0]) {
        for next in elem.next_siblings() {
            if next.value().is_text() {
                continue;
            }
            set.insert(next.id(), vec![elem]);
            break;
        }
    }
    for selector in &selectors[1..selectors.len() - 1] {
        let mut set2 = HashMap::new();
        for elem in document.select(&selector) {
            if set.contains_key(&elem.id()) {
                for next in elem.next_siblings() {
                    if next.value().is_text() {
                        continue;
                    }
                    let mut ts = set[&elem.id()].clone();
                    ts.push(elem);
                    set2.insert(next.id(), ts);
                    break;
                }
            }
        }
        set = set2;
    }
    let mut res = vec![];
    for elem in document.select(&selectors[selectors.len() - 1]) {
        if set.contains_key(&elem.id()) {
            let mut ts = set[&elem.id()].clone();
            ts.push(elem);
            res.push(ts);
        }
    }
    res
}
