mod dashboard;
mod logout;
mod newsletters;
mod password;

use actix_web::web;
use dashboard::admin_dashboard;
use logout::log_out;
use newsletters::*;
use password::*;

pub fn config_admin_scope(cfg: &mut web::ServiceConfig) {
    cfg.route("/dashboard", web::get().to(admin_dashboard))
        .route("/password", web::get().to(change_password_form))
        .route("/password", web::post().to(change_password))
        .route("/logout", web::post().to(log_out))
        .route("/newsletters", web::get().to(newsletters_form))
        .route("/newsletters", web::post().to(publish_newsletter));
}
