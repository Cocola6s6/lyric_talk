use askama::Template;

#[derive(Template)]
#[template(path = "../template/prompt.txt")]
pub struct PromptTemplate<'a> {
    pub context: &'a str,
}
