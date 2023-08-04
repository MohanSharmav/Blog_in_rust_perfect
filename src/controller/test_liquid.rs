// use std::{fs, io};
// // use std::fs::read_to_string;
// use actix_web::http::header::ContentType;
// use actix_web::{web, App, HttpResponse, HttpServer};
// // use liquid::{Parser};
// //
// // use liquid::*;
// // use liquid::ParserBuilder;
// // use liquid::{Parser, Environment, FileSystemLoader};
//
// use std::fs::{DirEntry, read_to_string};
// use std::path::Path;
// use warp::reply::html;
// // use html_parser::Dom;
//
// pub async fn liquid() -> Result<HttpResponse, actix_web::Error> {
//     // let mut env = Environment::new();
//     //
//     // // Create a FileSystemLoader object.
//     // let loader = FileSystemLoader::new("templates");
//     //
//     // // Load the Liquid template.
//     // let template = env.parse_template_file("index.liquid").unwrap();
//     //
//     // // Render the Liquid template.
//     // let html = template.render(&format!("Hello, {}!", "World")).unwrap();
//     //
//     // // Print the rendered HTML.
//     // println!("{}", html);
// //    let x= std::fs::read_to_string("templates/sneat-1.0.0/html/auth-login-basic.html")?;
// //     let template = liquid::ParserBuilder::with_stdlib()
// //         .build().unwrap()
// //         .parse(&*x).unwrap();
// //
// //     let mut globals = liquid::object!({
// //     "num": 4f64
// // });
// //     // visit_dirs("templates/sneat-1.0.0".as_ref(), "templates/sneat-1.0.0").expect("TODO: panic message");
// //
// //     let html = template.render(&globals).unwrap();
// //
//
//     // let path = Path::new("templates/sneat-1.0.0");
//     // let cb = |entry: &DirEntry| {
//     //     println!("{}", entry.path().display());
//     // };
//     // visit_dirs(&path, &cb).unwrap();
//
//     // Ok(HttpResponse::Ok()
//     //     .content_type(ContentType::html())
//     //     .body(html))
//
// }
// pub fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<()> {
//     if dir.is_dir() {
//         for entry in fs::read_dir(dir)? {
//             let entry = entry?;
//             let path = entry.path();
//             if path.is_dir() {
//                 visit_dirs(&path, cb)?;
//             } else {
//                 cb(&entry);
//             }
//         }
//     }
//     Ok(())
// }
