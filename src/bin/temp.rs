use std::io::Read;

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer).unwrap();
    let res = kunai::atcoder::get_tests_from_html(&buffer);
    eprintln!("res = {:?}", res);
}
