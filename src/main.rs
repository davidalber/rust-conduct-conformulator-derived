use handlebars::Handlebars;
use std::fs::File;
use std::io::prelude::*;

#[macro_use]
extern crate serde_derive;
extern crate toml;

#[derive(Debug, Deserialize, Serialize)]
struct ModerationPoint {
    text: Option<String>,
    points: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct CodeOfConduct {
    moderation_email: String,
    conduct: Vec<String>,
    moderation: Vec<ModerationPoint>,
}

fn load_coc() -> CodeOfConduct {
    let mut file = File::open("coc.toml").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    toml::from_str(&contents).unwrap()
}

fn make_markdown_coc(coc: &CodeOfConduct, reg: &Handlebars) {
    println!("{}", reg.render("markdown", &coc).unwrap());
}

fn make_web_coc(coc: &CodeOfConduct, reg: &Handlebars) {
    println!("{}", reg.render("web", &coc).unwrap());
}

fn main() {
    let coc: CodeOfConduct = load_coc();

    let mut reg = Handlebars::new();
    reg.register_templates_directory(".hbs", "templates").unwrap();

    //make_markdown_coc(&coc, &reg);
    make_web_coc(&coc, &reg);
}
