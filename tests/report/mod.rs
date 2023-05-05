use std::fmt::{Debug, Display, Formatter};

use serde_json::Value;

use rudp2plib::utils::read_file;

#[derive(PartialEq, Eq)]
enum Status {
    Passed,
    Failed,
    Skipped,
    Undefined,
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Passed => write!(f, "Passed"),
            Status::Failed => write!(f, "Failed"),
            Status::Skipped => write!(f, "Skipped"),
            Status::Undefined => write!(f, "Undefined"),
        }
    }
}

impl Status {
    fn value_of(value: &str) -> Status {
        match value {
            "Passed" => Status::Passed,
            "Failed" => Status::Failed,
            "Skipped" => Status::Skipped,
            _ => panic!("Status undefined : {}", value),
        }
    }
}

enum Keyword {
    Background,
    Feature,
    Scenario,
    Given,
    When,
    Then,
    And,
}

impl Display for Keyword {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Keyword::Background => write!(f, "Background"),
            Keyword::Feature => write!(f, "Feature"),
            Keyword::Scenario => write!(f, "Scenario"),
            Keyword::Given => write!(f, "Step [Given]"),
            Keyword::When => write!(f, "Step [When]"),
            Keyword::Then => write!(f, "Step [Then]"),
            Keyword::And => write!(f, "Step [And]"),
        }
    }
}

impl Keyword {
    fn value_of(name: &str) -> Keyword {
        match name {
            "Background" => Keyword::Background,
            "Feature" => Keyword::Feature,
            "Scenario" => Keyword::Scenario,
            "Given " => Keyword::Given,
            "When " => Keyword::When,
            "Then " => Keyword::Then,
            "And " => Keyword::And,
            _ => panic!("Keyword undefined : {}", name)
        }
    }
}

struct Element {
    keyword: Keyword,
    name: String,
    status: Status,
    children: Vec<Element>,
}

impl Debug for Element {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, " {} : {} [{}] > {:?}", self.keyword, self.name, self.status, self.children)
    }
}

fn parse_element(value: &Value) -> Element {
    let keyword = Keyword::value_of(value["keyword"].as_str().unwrap());
    let mut children = Vec::new();
    let mut status = Status::Undefined;
    match keyword {
        Keyword::Feature => children.append(&mut get_scenarios(value)),
        Keyword::Scenario => children.append(&mut get_steps(value)),
        _ => status = get_status(&children, value),
    };
    Element {
        keyword,
        name: value["name"].as_str().unwrap().to_string(),
        status,
        children,
    }
}

fn get_scenarios(f: &Value) -> Vec<Element> {
    let mut scenarios = Vec::new();
    if let Some(elements) = f["elements"].as_array() {
        for e in elements {
            scenarios.push(parse_element(e));
        }
    }
    scenarios
}

fn get_steps(f: &Value) -> Vec<Element> {
    let mut steps = Vec::new();
    if let Some(elements) = f["steps"].as_array() {
        for e in elements {
            steps.push(parse_element(e));
        }
    }
    steps
}

fn get_status(children: &Vec<Element>, f: &Value) -> Status {
    if let Some(status) = f["result"]["status"].as_str() {
        Status::value_of(status)
    } else {
        let mut status = Status::Passed;
        if children.iter().find(|p| p.status == Status::Failed).is_some() {
            status = Status::Failed;
        } else if children.iter().find(|p| p.status == Status::Skipped).is_some() {
            status = Status::Skipped;
        }
        status
    }
}

pub(crate) fn report_cucumber(report: &str) {
    let report = read_file(report);
    let json: Value = serde_json::from_slice(report.as_slice()).unwrap();
    let mut features = Vec::new();
    if let Some(array) = json.as_array() {
        for f in array {
            let element = parse_element(f);
            features.push(element);
        }
    }
    println!("{:?}", features)
}
