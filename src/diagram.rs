use crate::rendering::layout::{GridSize, ReservedWidth};
use crate::rendering::renderer::Renderer;
use std::cell::RefCell;
use std::rc::Rc;

type ID = String;

pub trait TimelineEvent {
    fn draw(
        &self,
        diagram: &SequenceDiagram,
        renderer: &mut dyn Renderer,
        grid: &GridSize,
        row: usize,
    );
    fn reserved_width(&self) -> Option<ReservedWidth>;
    fn height(&self) -> u32;
    fn col_range(&self) -> Option<(usize, usize)>;
}

#[derive(PartialEq, Debug, Clone)]
pub struct Participant {
    pub name: ID,
    label: String,
    kind: ParticipantKind,
    idx: usize,
}

impl Participant {
    pub fn new(name: String, kind: ParticipantKind) -> Participant {
        let label = name.clone();
        Participant::with_label(name, kind, label)
    }

    pub fn with_label(name: String, kind: ParticipantKind, label: String) -> Participant {
        Participant {
            name,
            label,
            kind,
            idx: 0,
        }
    }

    pub fn get_label(&self) -> &String {
        &self.label
    }

    pub fn get_kind(&self) -> &ParticipantKind {
        &self.kind
    }

    pub fn get_idx(&self) -> usize {
        self.idx
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
    pub from: Rc<RefCell<Participant>>,
    pub to: Rc<RefCell<Participant>>,
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

    pub fn get_header(&self) -> &str {
        &self.header
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

pub struct ParticipantCreated {
    pub(crate) participant: Rc<RefCell<Participant>>,
}

pub struct MessageSent {
    pub(crate) message: Message,
}

pub struct GroupStarted {
    pub(crate) group: Rc<RefCell<Group>>,
}

pub struct GroupEnded;

pub struct AltElse;

pub struct SequenceDiagram {
    participants: Vec<Rc<RefCell<Participant>>>,
    timeline: Vec<Vec<Box<dyn TimelineEvent>>>,
}

impl SequenceDiagram {
    pub fn new() -> SequenceDiagram {
        SequenceDiagram {
            participants: vec![],
            timeline: vec![vec![]],
        }
    }

    pub fn get_participants(&self) -> &Vec<Rc<RefCell<Participant>>> {
        &self.participants
    }

    pub fn get_timeline(&self) -> &Vec<Vec<Box<dyn TimelineEvent>>> {
        &self.timeline
    }

    fn find_participant_by_name(&self, id: &str) -> Option<Rc<RefCell<Participant>>> {
        self.participants
            .iter()
            .find(|&p| p.borrow().name.as_str() == id)
            .cloned()
    }

    pub fn add_participant(&mut self, mut participant: Participant) -> Rc<RefCell<Participant>> {
        participant.idx = self.participants.len();
        let rc_participant = Rc::new(RefCell::new(participant));
        self.timeline[0].push(Box::new(ParticipantCreated {
            participant: rc_participant.clone(),
        }));
        self.participants.push(rc_participant.clone());
        rc_participant
    }

    pub fn add_message(&mut self, from: String, to: String, label: String, style: LineStyle) {
        let mut get_participant = |name: &String| {
            self.find_participant_by_name(&name).unwrap_or_else(|| {
                let p = Participant::new(name.clone(), ParticipantKind::Default);
                self.add_participant(p)
            })
        };
        let from_participant = get_participant(&from);
        let to_participant = get_participant(&to);

        let message = Message {
            from: from_participant,
            to: to_participant,
            label,
            style,
        };
        self.timeline.push(vec![Box::new(MessageSent { message })]);
    }

    pub fn start_group(&mut self, group: Rc<RefCell<Group>>) {
        self.timeline.push(vec![Box::new(GroupStarted { group })]);
    }

    pub fn end_group(&mut self, group: Rc<RefCell<Group>>) {
        group.borrow_mut().end(self.timeline.len());
        self.timeline.push(vec![Box::new(GroupEnded)]);
    }

    pub fn add_alt_case(&mut self) {
        self.timeline.push(vec![Box::new(AltElse)]);
    }
}
