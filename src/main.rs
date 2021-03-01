#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] 
extern crate rocket;
extern crate rocket_raw_response;
extern crate rocket_multipart_form_data;
extern crate image_base64;
extern crate reqwest;
extern crate multipart;

use rocket::Request;
use rocket::response::content;

mod context;
mod main_handler;

#[catch(404)]
fn not_found(req: &Request<'_>) -> content::Html<String> {
    content::Html(format!("<p>Sorry, but '{}' is not a valid path!</p>
    <p>Try visiting /hello/&lt;name&gt;/&lt;age&gt; instead.</p>", req.uri()))
}   

fn rocket() -> rocket::Rocket {
    rocket::ignite()
    .mount("/", routes![
        main_handler::handler::index,
        main_handler::handler::upload
    ])
    .register(catchers![not_found])
}

fn main() {
    rocket().launch();
}
