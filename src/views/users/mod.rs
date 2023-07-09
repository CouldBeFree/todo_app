mod create;

use actix_web::web::{ServiceConfig, post, scope};

pub fn users_views_factory(app: &mut ServiceConfig) {
    app.service(
        scope("v1/users")
        .route("create", post().to(create::create))
    );
}
