use crate::diagram::{
    AltGroup, Group, LineStyle, Participant, ParticipantKind, SequenceDiagram, SimpleGroup,
};

use pest::error::Error as PestError;
use pest::iterators::Pair;
use pest::Parser;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Parser)]
#[grammar = "planty.pest"]
pub struct PParser;

#[derive(PartialEq, Debug, Clone)]
enum AstNode {
    Participant(Participant),
    Message {
        from: String,
        to: String,
        label: String,
        style: LineStyle,
    },
    GroupStart(String, String),
    AltElse(String),
    GroupEnd,
}

pub fn create_diagram(source: &str) -> Result<SequenceDiagram, Error> {
    let mut diagram = SequenceDiagram::new();
    let mut active_groups: VecDeque<Rc<RefCell<Group>>> = VecDeque::new();
    let ast = parse(source)?;

    for node in ast {
        match node {
            AstNode::Participant(p) => {
                diagram.add_participant(p);
            }
            AstNode::Message {
                from,
                to,
                label,
                style,
            } => diagram.add_message(from, to, label, style),
            AstNode::GroupStart(group_type, header) => {
                let timeline_pos = diagram.get_timeline().len();
                let group = match group_type.as_str() {
                    "group" => {
                        Group::SimpleGroup(SimpleGroup::new(timeline_pos, header, "".to_string()))
                    }
                    "alt" => Group::AltGroup(AltGroup::new(timeline_pos, header)),
                    _ => return Err(Error::new("Unexpected group type".to_string())),
                };
                let rc_group = Rc::new(RefCell::new(group));
                active_groups.push_back(rc_group.clone());
                diagram.start_group(rc_group);
            }
            AstNode::AltElse(label) => match active_groups.back_mut() {
                Some(rc_group) => match *rc_group.borrow_mut() {
                    Group::AltGroup(ref mut group) => {
                        let row = diagram.get_timeline().len();
                        group.add_case(label, row);
                        diagram.add_alt_case();
                    }
                    _ => {
                        return Err(Error::new(
                            "else when active group is not an 'alt' group".to_string(),
                        ))
                    }
                },
                None => return Err(Error::new("else without active alt group".to_string())),
            },
            AstNode::GroupEnd => match active_groups.pop_back() {
                Some(group) => diagram.end_group(group),
                None => return Err(Error::new("Found end without active group".to_string())),
            },
        }
    }

    match active_groups.pop_back() {
        None => Ok(diagram),
        Some(_) => Err(Error::new("Group with no closing end keyword".to_string())),
    }
}

fn parse(source: &str) -> Result<Vec<AstNode>, PestError<Rule>> {
    let mut ast = vec![];

    let pairs = PParser::parse(Rule::program, source)?;
    for pair in pairs {
        if let Rule::stmt = pair.as_rule() {
            let inner = pair.into_inner().next().unwrap();
            ast.push(build_ast_from_stmt(inner));
        }
    }

    Ok(ast)
}

fn build_ast_from_stmt(pair: Pair<Rule>) -> AstNode {
    match pair.as_rule() {
        Rule::participant => AstNode::Participant(parse_participant(pair)),
        Rule::message => parse_message(pair),
        Rule::group_start => parse_group_start(pair),
        Rule::group_end => AstNode::GroupEnd,
        Rule::alt_else => parse_alt_else(pair),
        unknown_expr => panic!("Unexpected expression: {:?}", unknown_expr),
    }
}

fn parse_group_start(pair: Pair<Rule>) -> AstNode {
    let mut pair = pair.into_inner();
    let group_type = pair.next().unwrap().as_str().to_string();
    let header = match pair.next() {
        Some(h) => h.as_str().to_string(),
        None => "".to_string(),
    };
    AstNode::GroupStart(group_type, header)
}

fn parse_alt_else(pair: Pair<Rule>) -> AstNode {
    let mut pair = pair.into_inner();
    let label = pair.next().unwrap().as_str().to_string();
    AstNode::AltElse(label)
}

fn parse_participant(pair: Pair<Rule>) -> Participant {
    let mut pair = pair.into_inner();
    let kind = match pair.next().unwrap().as_str() {
        "participant" => ParticipantKind::Default,
        "actor" => ParticipantKind::Actor,
        "database" => ParticipantKind::Database,
        unknown => panic!("Unexpected participant type: {:?}", unknown),
    };
    let label_pair = pair.next().unwrap();
    let label = match label_pair.as_rule() {
        Rule::ident => label_pair.as_str(),
        Rule::string => {
            // Strip the leading and trailing "
            let str = &label_pair.as_str();
            &str[1..str.len() - 1]
        }
        unknown_expr => panic!(
            "Unexpected expression in participant label: {:?}",
            unknown_expr
        ),
    };

    let name = match pair.next() {
        Some(inner) => inner.into_inner().next().unwrap().as_str(),
        None => label,
    };

    Participant::with_label(String::from(name), kind, String::from(label))
}

fn parse_message(pair: Pair<Rule>) -> AstNode {
    let mut pair = pair.into_inner();
    let left_participant = pair.next().unwrap();
    let arrow = pair.next().unwrap();
    let right_participant = pair.next().unwrap();
    let line_style = match arrow.as_str() {
        "<-" | "->" => LineStyle::Plain,
        "<--" | "-->" => LineStyle::Dashed,
        _ => panic!("unexpected arrow type received"),
    };
    let from;
    let to;
    if arrow.as_str().starts_with('<') {
        from = right_participant.as_str();
        to = left_participant.as_str();
    } else {
        from = left_participant.as_str();
        to = right_participant.as_str();
    };
    let label = match pair.next() {
        Some(l) => l.into_inner().next().unwrap().as_str(),
        None => "",
    };

    AstNode::Message {
        from: String::from(from),
        to: String::from(to),
        label: String::from(label),
        style: line_style,
    }
}

pub enum Error {
    PestError(PestError<Rule>),
    ModelError { message: String },
}

impl From<PestError<Rule>> for Error {
    fn from(err: PestError<Rule>) -> Self {
        Error::PestError(err)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::PestError(err) => write!(f, "{}", err.to_string()),
            Error::ModelError { message } => write!(f, "{}", message),
        }
    }
}

impl Error {
    fn new(message: String) -> Error {
        Error::ModelError { message }
    }
}
