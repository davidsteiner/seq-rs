extern crate pest;
#[macro_use]
extern crate pest_derive;

mod diagram;
mod parser;
mod renderer;

pub fn parse(content: &str) -> Result<String, pest::error::Error<parser::Rule>> {
    let diagram = parser::create_diagram(content)?;
    Ok(renderer::render(&diagram))
}

#[test]
fn example_puml() {
    let unparsed_file = std::fs::read_to_string("example.puml").expect("cannot read puml file");
    parse(&unparsed_file).expect("failed to parse diagram");
}
