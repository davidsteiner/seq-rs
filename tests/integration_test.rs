extern crate seq_rs;
extern crate sxd_document;
extern crate sxd_xpath;

use sxd_document::dom::Document;
use sxd_document::parser;
use sxd_xpath::{evaluate_xpath, Value};

#[test]
fn participants() {
    let diagram_str = "
    @startuml
    actor Actor
    participant \"With Label\" as wl
    database db
    participant P
    Actor -> wl
    Actor -> P
    wl -> P
    P -> db
    @enduml";

    let svg = seq_rs::parse(diagram_str, false).expect("Parsing failed");

    let package = parser::parse(&svg).expect("failed to parse SVG XML");
    let document = package.as_document();

    // Assert that we draw Actor twice
    let value = find_text(&document, "Actor");
    assert_node_count(value, 2);

    // Assert that we draw the label for 'wl' rather than 'wl'
    let value = find_text(&document, "With Label");
    assert_node_count(value, 2);

    let value = find_text(&document, "wl");
    assert_node_count(value, 0);
}

#[test]
fn messages() {
    let diagram_str = "
    @startuml
    a -> b: Simple arrow
    c <- b: Reverse arrow
    a --> c: Dashed arrow
    a -> a: Self arrow
    c -> a
    @enduml";

    let svg = seq_rs::parse(diagram_str, false).expect("Parsing failed");

    let package = parser::parse(&svg).expect("failed to parse SVG XML");
    let document = package.as_document();

    for arrow_label in &[
        "Simple arrow",
        "Reverse arrow",
        "Dashed arrow",
        "Self arrow",
    ] {
        let value = find_text(&document, arrow_label);
        assert_node_count(value, 1);
    }
}

fn assert_node_count(value: Value, count: usize) {
    match value {
        Value::Nodeset(nodeset) => assert_eq!(nodeset.size(), count),
        _ => panic!("expected nodeset, got something else"),
    }
}

fn find_text<'a>(document: &'a Document, text: &str) -> Value<'a> {
    let xpath = format!("//text()[normalize-space() = '{}']", text);
    evaluate_xpath(&document, &xpath).expect("failed to evaluate xpath")
}
