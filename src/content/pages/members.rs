use handlebars::Handlebars;
use serde_json::Map;

pub fn get_content() -> String {
    let mut handlebars = Handlebars::new();
    
    // register template from a file and assign a name to it
    handlebars.register_template_file("members", "static/assets/page-content/members-content.hbs").unwrap();

    let mut data = Map::new();
    
    handlebars.render("members", &data).unwrap()
}