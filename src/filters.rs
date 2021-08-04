use crate::handlers;
use warp::Filter;

pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    home()
        .or(members())
        .or(featured())
        .or(all_mocs())
        .or(apply())
        .or(collabs())
}

/// GET /home
pub fn home() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("home")
        .and(warp::get())
        .and_then(handlers::home)
}

/// GET /members
pub fn members() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("members")
        .and(warp::get())
        .and_then(handlers::members)
}

/// GET /featured
pub fn featured() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("featured")
        .and(warp::get())
        .and_then(handlers::featured)
}

/// GET /all
pub fn all_mocs() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("all")
        .and(warp::get())
        .and_then(handlers::all_mocs)
}

/// GET /apply
pub fn apply() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("apply")
        .and(warp::get())
        .and_then(handlers::apply)
}

/// GET /collaborations
pub fn collabs() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("collaborations")
        .and(warp::get())
        .and_then(handlers::collabs)
}

// fn json_body() -> impl Filter<Extract = (Todo,), Error = warp::Rejection> + Clone {
//     // When accepting a body, we want a JSON body
//     // (and to reject huge payloads)...
//     warp::body::content_length_limit(1024 * 16).and(warp::body::json())
// }
