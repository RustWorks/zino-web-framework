use crate::{
    controller::{auth, file, stats, user},
    middleware,
    model::Tag,
};
use ntex::web::{ServiceConfig, get, post, scope};
use zino::{DefaultController, RouterConfigure};
use zino_model::User;

pub fn routes() -> Vec<RouterConfigure> {
    vec![
        auth_router as RouterConfigure,
        file_router as RouterConfigure,
        user_router as RouterConfigure,
        tag_router as RouterConfigure,
    ]
}

pub fn debug_routes() -> Vec<RouterConfigure> {
    vec![
        stats_router as RouterConfigure,
        user_debug_router as RouterConfigure,
        tag_debug_router as RouterConfigure,
    ]
}

fn auth_router(cfg: &mut ServiceConfig) {
    cfg.route("/auth/login", post().to(auth::login));
    cfg.service(
        scope("/auth")
            .route("/refresh", get().to(auth::refresh))
            .route("/logout", post().to(auth::logout))
            .wrap(middleware::UserSessionInitializer),
    );
}

fn file_router(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/file")
            .route("/upload", post().to(file::upload))
            .route("/decrypt", get().to(file::decrypt))
            .wrap(middleware::UserSessionInitializer),
    );
}

fn user_router(cfg: &mut ServiceConfig) {
    cfg.route("/user/new", post().to(user::new))
        .route("/user/{id}/delete", post().to(User::soft_delete))
        .route("/user/{id}/update", post().to(User::update))
        .route("/user/{id}/view", get().to(user::view))
        .route("/user/list", get().to(User::list))
        .route("/user/import", post().to(User::import))
        .route("/user/export", get().to(User::export));
}

fn tag_router(cfg: &mut ServiceConfig) {
    cfg.route("/tag/new", post().to(Tag::new))
        .route("/tag/{id}/delete", post().to(Tag::soft_delete))
        .route("/tag/{id}/update", post().to(Tag::update))
        .route("/tag/{id}/view", get().to(Tag::view))
        .route("/tag/list", get().to(Tag::list))
        .route("/tag/tree", get().to(Tag::tree));
}

fn stats_router(cfg: &mut ServiceConfig) {
    cfg.route("/stats", get().to(stats::index));
}

fn user_debug_router(cfg: &mut ServiceConfig) {
    cfg.route("/user/schema", get().to(User::schema))
        .route("/user/definition", get().to(User::definition))
        .route("/user/mock", get().to(User::mock));
}

fn tag_debug_router(cfg: &mut ServiceConfig) {
    cfg.route("/tag/schema", get().to(Tag::schema))
        .route("/tag/definition", get().to(Tag::definition))
        .route("/tag/mock", get().to(Tag::mock));
}
