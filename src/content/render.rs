use crate::content::pages::*;

use serde_json::{Map, value::Value as Json};
use handlebars::{Handlebars, to_json};
use std::fs;

pub enum CurrentPage {
    Index,
    Members,
    Featured,
    Apply,
    AllMocs,
    Collabs
}

impl CurrentPage {
    pub fn get_content(&self) -> String {
        match &self {
            CurrentPage::Index => index::get_content(),
            CurrentPage::Members => members::get_content(),
            CurrentPage::Featured => featured::get_content(),
            CurrentPage::Apply => apply::get_content(),
            CurrentPage::AllMocs => String::from(""),
            CurrentPage::Collabs => String::from("")
        }
    }
}

pub mod template {
    use super::*;

    pub fn get_page_template_data(current_page: &CurrentPage) -> Map<String, Json> {
        let mut data: Map<String, Json> = Map::new();
        data.insert("page_links".to_string(), to_json(&render_page_links_source(&current_page)));
        data.insert("social_links".to_string(), to_json(&render_social_links_source()));
        data.insert("hover_css".to_string(), to_json(&render_hover_css_source()));
        data.insert("content".to_string(), to_json(current_page.get_content()));

        data
    }

    fn render_page_links_source(current_page: &CurrentPage) -> String {
        let mut handlebars = Handlebars::new();
    
        // register template from a file and assign a name to it
        handlebars.register_template_file("page-links", "static/assets/templates/page-links.hbs").unwrap();
    
        let mut data = Map::new();
    
        let keys = ["index", "members", "gallery", "featured", "all_mocs", "collabs", "apply"];
        for key in keys.iter() {
            data.insert(key.to_string(), to_json(""));
        }
    
        match current_page {
            CurrentPage::Index    => data.insert("index".to_string(), to_json("active")),
            CurrentPage::Apply    => data.insert("apply".to_string(), to_json("active")),
            CurrentPage::Members  => data.insert("members".to_string(), to_json("active")),
            CurrentPage::AllMocs  => {
                data.insert("gallery".to_string(), to_json("active"));
                data.insert("all_mocs".to_string(), to_json("active"))
            },
            CurrentPage::Featured => {
                data.insert("gallery".to_string(), to_json("active"));
                data.insert("featured".to_string(), to_json("active"))
            },
            CurrentPage::Collabs  => {
                data.insert("gallery".to_string(), to_json("active"));
                data.insert("collabs".to_string(), to_json("active"))
            }
        };
    
        handlebars.render("page-links", &data).unwrap()
    }
    
    fn render_social_links_source() -> String {
        fs::read_to_string("static/assets/templates/social-links.hbs").unwrap()
    }
    
    fn render_hover_css_source() -> String {
        fs::read_to_string("static/assets/templates/hover-css.hbs").unwrap()
    }
}

