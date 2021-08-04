/// These are our API handlers, the ends of each filter chain.
/// Notice how thanks to using `Filter::and`, we can define a function
/// with the exact arguments we'd expect from each filter in the chain.
/// No tuples are needed, it's auto flattened for the functions.

use std::convert::Infallible;
use warp::http::StatusCode;

use serde_json::{Map, value::Value as Json};
use handlebars::Handlebars;

use crate::content::render::{
    CurrentPage,
    template::get_page_template_data
};

pub async fn home() -> Result<impl warp::Reply, Infallible> {
    let body: String = load_content(CurrentPage::Index);

    Ok(warp::reply::with_status(warp::reply::html(body), StatusCode::OK))
}

pub async fn members() -> Result<impl warp::Reply, Infallible> {
    let body: String = load_content(CurrentPage::Members);

    Ok(warp::reply::with_status(warp::reply::html(body), StatusCode::OK))
}

pub async fn featured() -> Result<impl warp::Reply, Infallible> {
    let body: String = load_content(CurrentPage::Featured);

    Ok(warp::reply::with_status(warp::reply::html(body), StatusCode::OK))
}

pub async fn all_mocs() -> Result<impl warp::Reply, Infallible> {
    let body: String = load_content(CurrentPage::AllMocs);

    Ok(warp::reply::with_status(warp::reply::html(body), StatusCode::OK))
}

pub async fn apply() -> Result<impl warp::Reply, Infallible> {
    let body: String = load_content(CurrentPage::Apply);

    Ok(warp::reply::with_status(warp::reply::html(body), StatusCode::OK))
}

pub async fn collabs() -> Result<impl warp::Reply, Infallible> {
    let body: String = load_content(CurrentPage::Collabs);

    Ok(warp::reply::with_status(warp::reply::html(body), StatusCode::OK))
}

fn load_content(current_page: CurrentPage) -> String {
    let mut handlebars = Handlebars::new();
    handlebars.register_template_file("page-template", "static/assets/templates/page-template.hbs").unwrap();

    // register some custom helpers
    // handlebars.register_helper("format_b64_image", Box::new(format_base64_image));

    let data: Map<String, Json> = get_page_template_data(&current_page);
    let body: String = handlebars.render("page-template", &data).unwrap();

    body
}
