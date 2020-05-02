use self::Event::*;
use std::cell::RefCell;
use std::rc::Rc;

type ID = String;

#[derive(PartialEq, Debug, Clone)]
pub struct Participant {
    name: ID,
    label: String,
    kind: ParticipantKind,
}

impl Participant {
    pub fn new(name: String, kind: ParticipantKind) -> Participant {
        let label = name.clone();
        Participant::with_label(name, kind, label)
    }

    pub fn with_label(name: String, kind: ParticipantKind, label: String) -> Participant {
        Participant { name, label, kind }
    }

    pub fn get_label(&self) -> &String {
        &self.label
    }

    pub fn get_kind(&self) -> &ParticipantKind {
        &self.kind
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum ParticipantKind {
    Default,
    Actor,
    Database,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Message {
    pub from: ID,
    pub to: ID,
    pub label: String,
    pub style: LineStyle,
}

#[derive(PartialEq, Debug, Clone)]
pub enum LineStyle {
    Plain,
    Dashed,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Case {
    pub row: usize,
    pub label: String,
}

#[derive(PartialEq, Debug, Clone)]
pub struct SimpleGroup {
    start: usize,
    end: usize,
    label: String,
    header: String,
}

#[derive(PartialEq, Debug, Clone)]
pub struct AltGroup {
    group: SimpleGroup,
    cases: Vec<Case>,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Group {
    SimpleGroup(SimpleGroup),
    AltGroup(AltGroup),
}

impl Group {
    fn end(&mut self, end: usize) {
        match self {
            Group::SimpleGroup(g) => g.end(end),
            Group::AltGroup(g) => g.end(end),
        }
    }
}

impl SimpleGroup {
    pub fn new(start: usize, label: String, header: String) -> SimpleGroup {
        SimpleGroup {
            start,
            end: start,
            label,
            header,
        }
    }

    pub fn end(&mut self, end: usize) {
        self.end = end;
    }

    pub fn get_start(&self) -> usize {
        self.start
    }

    pub fn get_end(&self) -> usize {
        self.end
    }

    pub fn get_label(&self) -> &str {
        &self.label
    }
}

impl AltGroup {
    pub fn new(start: usize, header: String) -> AltGroup {
        let group = SimpleGroup {
            start,
            end: 0,
            label: "alt".to_string(),
            header,
        };
        AltGroup {
            group,
            cases: vec![],
        }
    }

    pub fn add_case(&mut self, label: String, row: usize) -> usize {
        let idx = self.cases.len();
        self.cases.push(Case { label, row });
        idx
    }

    pub fn end(&mut self, end: usize) {
        self.group.end(end);
    }

    pub fn get_simple_group(&self) -> &SimpleGroup {
        &self.group
    }

    pub fn get_cases(&self) -> &Vec<Case> {
        &self.cases
    }
}

pub enum Event {
    ParticipantCreated(ID),
    MessageSent(Message),
    GroupStarted(Rc<RefCell<Group>>),
    GroupEnded(Rc<RefCell<Group>>),
    AltElse {
        group: Rc<RefCell<Group>>,
        case_idx: usize,
    },
}

pub struct SequenceDiagram {
    participants: Vec<Participant>,
    timeline: Vec<Vec<Event>>,
}

impl SequenceDiagram {
    pub fn new() -> SequenceDiagram {
        SequenceDiagram {
            participants: vec![],
            timeline: vec![vec![]],
        }
    }

    pub fn get_participants(&self) -> &Vec<Participant> {
        &self.participants
    }

    pub fn get_timeline(&self) -> &Vec<Vec<Event>> {
        &self.timeline
    }

    pub fn find_participant(&self, id: &str) -> Option<(usize, &Participant)> {
        self.participants
            .iter()
            .enumerate()
            .find(|(_idx, p)| p.name.as_str() == id)
    }

    pub fn add_participant(&mut self, participant: Participant) {
        let existing = self.find_participant(&participant.name);
        if existing.is_none() {
            self.timeline[0].push(ParticipantCreated(participant.name.clone()));
            self.participants.push(participant)
        }
    }

    pub fn add_message(&mut self, message: Message) {
        self.add_participant(Participant::new(
            message.from.clone(),
            ParticipantKind::Default,
        ));
        self.add_participant(Participant::new(
            message.to.clone(),
            ParticipantKind::Default,
        ));
        self.timeline.push(vec![MessageSent(message)]);
    }

    pub fn start_group(&mut self, group: Rc<RefCell<Group>>) {
        self.timeline.push(vec![GroupStarted(group)]);
    }

    pub fn end_group(&mut self, group: Rc<RefCell<Group>>) {
        group.borrow_mut().end(self.timeline.len());
        self.timeline.push(vec![GroupEnded(group)]);
    }

    pub fn add_alt_case(&mut self, group: Rc<RefCell<Group>>, case_idx: usize) {
        self.timeline.push(vec![Event::AltElse { group, case_idx }]);
    }
}
