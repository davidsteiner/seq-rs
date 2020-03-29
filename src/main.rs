extern crate pest;
#[macro_use]
extern crate pest_derive;

mod diagram;
mod parser;
mod renderer;

fn main() {
    let unparsed_file = std::fs::read_to_string("example.puml").expect("cannot read puml file");
    let diagram = parser::create_diagram(&unparsed_file).expect("cannot parse file");
    renderer::render(&diagram);
}
