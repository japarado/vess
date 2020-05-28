use actix_web::web;

#[allow(unused_imports)]
use crate::controllers;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/posts")
            .service(controllers::post_controller::index)
            .service(controllers::post_controller::show)
            .service(controllers::post_controller::store)
            .service(controllers::post_controller::destroy)
            .service(controllers::post_controller::update),
    )
    .service(
        web::scope("/auth")
            .service(controllers::auth_controller::login)
            .service(controllers::auth_controller::register)
            .service(controllers::auth_controller::logout),
    )
    .service(
        web::scope("/comments")
            .service(controllers::comment_controller::show)
            .service(controllers::comment_controller::destroy),
    );
    // cfg.service(
    //     web::scope("/auth")
    //         .service(controllers::auth_controller::login)
    //         .service(controllers::auth_controller::register)
    //         .service(controllers::auth_controller::logout)
    //         .service(controllers::auth_controller::me),
    // )
    // .service(
    //     web::scope("/posts")
    //         .service(controllers::post_controller::index)
    //         .service(controllers::post_controller::mine)
    //         .service(controllers::post_controller::store)
    //         .service(controllers::post_controller::show)
    //         .service(controllers::post_controller::update)
    //         .service(controllers::post_controller::destroy),
    // )
    // .service(
    //     web::scope("/comments")
    //         .service(controllers::comment_controller::store)
    //         .service(controllers::comment_controller::destroy),
    // )
    // .service(
    //     web::scope("/tags")
    //         .service(controllers::tag_controller::index)
    //         .service(controllers::tag_controller::store)
    //         .service(controllers::tag_controller::destroy),
    // )
    // .service(
    //     web::scope("/post-tags")
    //         .service(controllers::post_tag_controller::index)
    //         .service(controllers::post_tag_controller::show),
    // );
}
