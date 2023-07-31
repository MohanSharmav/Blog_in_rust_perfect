// use handlebars::Handlebars;
// use html5ever::tendril::TendrilSink;
// use html5ever::tendril::stream::TendrilSinkExt;
// use html5ever::parse_document;
// use html5ever::rcdom::{Handle, NodeData, RcDom};
// use html5ever::serialize::{SerializeOpts, TraversalScope};
// use markup5ever_rcdom::RcDom as SerRcDom;
// use markup5ever_rcdom::serialize::{serialize, serialize_tree};
// use markup5ever_rcdom::serialize::TraversalScope as SerTraversalScope;
//
// // HTML parsing function
// fn parse_html(input: &str) -> RcDom {
//     let mut dom = parse_document(RcDom::default(), Default::default())
//         .from_utf8()
//         .read_from(&mut input.as_bytes())
//         .unwrap();
//     return dom;
// }
//
// // Handlebars rendering function
// fn render_html(handlebars: &Handlebars, dom: &RcDom) -> String {
//     let mut buf = Vec::<u8>::new();
//     let opts = SerializeOpts {
//         traversal_scope: TraversalScope::IncludeNode,
//         ..Default::default()
//     };
//
//     let ser_dom = SerRcDom::default();
//
//     serialize_tree(&ser_dom.document, opts, &mut buf).unwrap();
//     let output = String::from_utf8(buf).unwrap();
//
//     let mut data = serde_json::Map::new();
//     data.insert("content".to_string(), serde_json::json!(output));
//
//     let rendered = handlebars.render("template", &data).unwrap();
//     return rendered;
// }
//
// fn mains() {
//     // Sample HTML content
//     let html_content = r#"
//         <html>
//             <head>
//                 <title>Hello, Handlebars</title>
//             </head>
//             <body>
//                 <h1>{{title}}</h1>
//                 <p>{{content}}</p>
//             </body>
//         </html>
//     "#;
//
//     // Initialize Handlebars and register a template
//     let mut handlebars = Handlebars::new();
//     handlebars.register_template_string("sample2", html_content).unwrap();
//
//     // Parse the HTML content
//     let dom = parse_html(html_content);
//
//     // Define data to pass to Handlebars
//     let mut data = serde_json::Map::new();
//     data.insert("title".to_string(), serde_json::json!("Welcome to Rust HTML Handlebars"));
//     data.insert("content".to_string(), serde_json::json!("This is an example using Handlebars to render HTML in Rust."));
//
//     // Render HTML using Handlebars
//     let rendered_html = render_html(&handlebars, &dom);
//
//     // Print the rendered HTML
//     println!("{}", rendered_html);
// }
