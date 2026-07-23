use crate::adapters::http::{auth, categories_admin, posts_admin, posts_guest};
use actix_files::Files;
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").to(posts_guest::redirect_user))
        .service(web::resource("/posts").to(posts_guest::index_redirect))
        .service(web::resource("./templates/").to(posts_guest::redirect_user))
        .service(web::resource("/check").to(auth::check_user))
        .service(web::resource("/admin/posts/page/{page_number}").to(posts_admin::admin_index))
        .service(
            web::resource("/admin/categories/new")
                .route(web::get().to(categories_admin::new_category))
                .route(web::post().to(categories_admin::create_category)),
        )
        .service(
            web::resource("/admin/category/{title}/edit")
                .route(web::get().to(categories_admin::edit_category))
                .route(web::post().to(categories_admin::update_category)),
        )
        .service(
            web::resource("/admin/categories/page/{page_number}")
                .route(web::get().to(categories_admin::get_all_categories)),
        )
        .service(web::resource("/admin/posts/new").to(posts_admin::get_new_post))
        .service(web::resource("/admin/posts").route(web::post().to(posts_admin::new_post)))
        .service(
            web::resource("/admin/posts/{post_id}").route(web::get().to(posts_admin::show_post)),
        )
        .service(
            web::resource("/admin/posts/{post_id}/edit")
                .route(web::get().to(posts_admin::edit_post))
                .route(web::post().to(posts_admin::update_post)),
        )
        .service(
            web::resource("/admin/post/{post_id}/delete")
                .route(web::get().to(posts_admin::destroy_post)),
        )
        .service(
            web::resource("/admin/categories/{category_id}/page/{page_number}")
                .to(posts_admin::get_categories_posts),
        )
        .service(
            web::resource("/admin/category/{name}/delete")
                .route(web::get().to(categories_admin::destroy_category)),
        )
        .service(
            web::resource("/login")
                .route(web::get().to(auth::get_login))
                .route(web::post().to(auth::login)),
        )
        .service(web::resource("/logout").to(auth::logout))
        .service(
            web::resource("/register")
                .route(web::get().to(auth::get_register))
                .route(web::post().to(auth::register)),
        )
        .service(web::resource("/posts/{post_id}").route(web::get().to(posts_guest::show_posts)))
        .service(
            web::resource("/posts/category/{category_id}/page/{page_number}")
                .to(posts_guest::get_category_posts),
        )
        .service(
            web::resource("/posts/page/{page_number}").route(web::get().to(posts_guest::index)),
        )
        .service(Files::new("/", blog_views::root()).show_files_listing());
}
