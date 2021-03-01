use crate::context::tmpl_context::IndexTemplate;
use rocket::response::content;
use sailfish::TemplateOnce;
use anyhow::Result;

use rocket::Data;
use rocket::http::ContentType;

use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};

use rocket_multipart_form_data::{
    mime, 
    MultipartFormDataOptions, 
    MultipartFormData, 
    MultipartFormDataField,
    MultipartFormDataError
};

use rocket_raw_response::RawResponse;
use reqwest::blocking::multipart;

use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

#[get("/")]
pub fn index() -> content::Html<String> {
    let ctx = IndexTemplate {
        title: String::from("Image Uploader")
    };

    content::Html(ctx.render_once().unwrap())
}

#[post("/upload", data = "<data>")]
pub fn upload(content_type: &ContentType, data: Data) -> Result<RawResponse, &'static str> {
    let options = MultipartFormDataOptions::with_multipart_form_data_fields(vec![
        MultipartFormDataField::raw("image")
            .size_limit(32 * 1024 * 1024)
            .content_type_by_string(Some(mime::IMAGE_STAR))
            .unwrap(),
        MultipartFormDataField::text("caption"),
    ]);

    let mut multipart_form_data = match MultipartFormData::parse(content_type, data, options) {
        Ok(multipart_form_data) => multipart_form_data,
        Err(err) => {
            match err {
                MultipartFormDataError::DataTooLargeError(_) => {
                    return Err("The file is too large.");
                }
                MultipartFormDataError::DataTypeError(_) => {
                    return Err("The file is not an image.");
                }
                _ => panic!("{:?}", err),
            }
        }
    };

    let image = multipart_form_data.raw.remove("image");
    let caption = multipart_form_data.texts.remove("caption");

    let cpt = match caption {
        Some(mut r) => {
            let text_field = r.remove(0);
            let _text = text_field.text;
            _text
        }
        None => "".to_string()
    };

    match image {
        Some(mut image) => {
            let raw = image.remove(0);

            let content_type = raw.content_type;
            let file_name = raw.file_name.unwrap_or("Image".to_string());
            let data = raw.raw;

            let mut pos = 0;
            let mut buffer = File::create(file_name.clone()).unwrap();
            while pos < data.len() {
                let bytes_written = buffer.write(&data[pos..]).unwrap();
                pos += bytes_written;
            }

            let pth = Path::new(&file_name);
            let mut headers = HeaderMap::new();
            headers.insert(CONTENT_TYPE, HeaderValue::from_static("multipart/form-data"));
            let token = "1569073353:AAGtuCfh0C_21zANq_Q0vNd8UHGFJD0UYIU";
            let command = format!("/sendPhoto?chat_id={}", "122604792");
            let url_post = format!("https://api.telegram.org/bot{}{}", token, command);
            let form = multipart::Form::new()
            .text("caption", cpt)
            .file("photo", pth.to_str().unwrap()).unwrap();
            let req = reqwest::blocking::Client::new();
            let res = req.post(url_post.as_str())
            .headers(headers)
            .multipart(form)
            .send();
            
            let r = res.unwrap();
            println!("Res: {}", r.text().unwrap());
            Ok(RawResponse::from_vec(data, Some(file_name), content_type))
        }
        None => Err("Please input a file."),
    }
}
