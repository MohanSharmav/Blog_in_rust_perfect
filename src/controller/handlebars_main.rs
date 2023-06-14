// use std::fs;
// use glob::glob;
// use handlebars::Handlebars;
//
// pub async fn get_all_handlebars() -> Registry<'reg>
// {
//     let mut handlebars = handlebars::Handlebars::new();
//     let index_template = fs::read_to_string("templates/**/*.hbs")
//         .map_err(actix_web::error::ErrorInternalServerError)?;
//
//     handlebars
//         .register_template_string("all_categories", &index_template)
//         .map_err(actix_web::error::ErrorInternalServerError)?;
//
//     handlebars
// }
//
// fn load_handlebar_templates() -> Result<Handlebars<'static>,actix_web::Error> {
//     let pattern = "templates/**/*.hbs";
//
//     let template_files = glob(pattern)?
//         .filter_map(Result::ok)
//         .map(|file| (file.to_str().unwrap(), file));
//
//     let mut handlebars = Handlebars::new();
//     handlebars.register_template_file(".hbs", template_files)
//         .map_err(actix_web::error::ErrorInternalServerError)?;
//
//
//     Ok(handlebars)
// }