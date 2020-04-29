extern crate pest;
#[macro_use]
extern crate pest_derive;

mod diagram;
mod parser;
mod rendering;

pub fn parse(content: &str) -> Result<String, parser::Error> {
    let diagram = parser::create_diagram(content)?;
    Ok(rendering::render(&diagram))
}

#[test]
fn example_puml() {
    let unparsed_file = std::fs::read_to_string("example.puml").expect("cannot read puml file");
    println!("{}", parse(&unparsed_file).expect("Parsing puml failed"));
}
