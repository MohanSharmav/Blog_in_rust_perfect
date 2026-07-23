use handlebars::Handlebars;

#[test]
fn register_loads_every_known_template() {
    let mut handlebars = Handlebars::new();
    blog_views::register(&mut handlebars).expect("templates should register");

    for name in [
        "admin_category_table",
        "admin_post_table",
        "admin_separate_categories",
        "auth-login-basic",
        "auth-register-basic",
        "category",
        "common",
        "new_category",
        "new_post",
        "single",
        "update_category",
        "update_post",
    ] {
        assert!(
            handlebars.get_template(name).is_some(),
            "expected template `{name}` to be registered"
        );
    }
}

#[test]
fn assets_directory_is_bundled() {
    let css = std::path::Path::new(blog_views::ROOT).join("assets/css/common.css");
    assert!(css.is_file(), "expected {} to exist", css.display());
}
