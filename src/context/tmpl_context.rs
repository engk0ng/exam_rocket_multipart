use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "index.stpl")]

pub struct IndexTemplate {
    pub title: String,
}