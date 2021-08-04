use std::fs;

pub fn get_content() -> String {
    fs::read_to_string("static/assets/page-content/apply-content.hbs").unwrap()
}