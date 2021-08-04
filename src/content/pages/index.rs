use handlebars::Handlebars;
use serde_json::Map;

pub fn get_content() -> String {
    let mut handlebars = Handlebars::new();
    
    // register template from a file and assign a name to it
    handlebars.register_template_file("index", "static/assets/page-content/index-content.hbs").unwrap();

    let mut data = Map::new();
    
    handlebars.render("index", &data).unwrap()
}