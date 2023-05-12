use std::fs::File;
use std::io::Write;

use serde_json::Value;

use rudp2plib::utils::read_file;

#[derive(PartialEq, Eq)]
enum Status {
    Passed,
    Failed,
    Skipped,
    Undefined,
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
    name: String,
    status: Status,
    children: Vec<Element>,
}

fn parse_element(value: &Value) -> Option<Element> {
    let keyword = Keyword::value_of(value["keyword"].as_str().unwrap());
    let mut children = Vec::new();
    match keyword {
        Keyword::Feature => children.append(&mut get_scenarios(value)),
        Keyword::Scenario => children.append(&mut get_steps(value)),
        Keyword::Background => return None,
        _ => (),
    };
    let status = get_status(&children, value);
    Some(Element {
        name: value["name"].as_str().unwrap().to_string(),
        status,
        children,
    })
}

fn get_scenarios(f: &Value) -> Vec<Element> {
    let mut scenarios = Vec::new();
    if let Some(elements) = f["elements"].as_array() {
        for e in elements {
            if let Some(elt) = parse_element(e) {
                scenarios.push(elt);
            }
        }
    }
    scenarios
}

fn get_steps(f: &Value) -> Vec<Element> {
    let mut steps = Vec::new();
    if let Some(elements) = f["steps"].as_array() {
        for e in elements {
            if let Some(elt) = parse_element(e) {
                steps.push(elt);
            }
        }
    }
    steps
}

fn get_status(children: &Vec<Element>, f: &Value) -> Status {
    if let Some(status) = f["result"]["status"].as_str() {
        Status::value_of(status)
    } else {
        let mut status = Status::Undefined;
        if !children.is_empty() {
            if children.iter().find(|p| p.status == Status::Failed).is_some() {
                status = Status::Failed;
            } else if children.iter().find(|p| p.status == Status::Skipped).is_some() {
                status = Status::Skipped;
            } else {
                status = Status::Passed;
            }
        }
        status
    }
}

fn status_checkbox_md(status: Status) -> &'static str {
    if status == Status::Passed { "[X]" } else { "[ ]" }
}

pub(crate) fn report_cucumber(report: &str) {
    let report = read_file(report);
    let json: Value = serde_json::from_slice(report.as_slice()).unwrap();
    let mut features = Vec::new();
    if let Some(array) = json.as_array() {
        for f in array {
            if let Some(element) = parse_element(f) {
                features.push(element);
            }
        }
    }
    let mut readme = File::create("target/replacer.txt").unwrap();
    let mut buf = Vec::new();
    buf.append(&mut "{FEATURES}:".as_bytes().to_vec());
    buf.append(&mut "## Features\\n".as_bytes().to_vec());
    for feature in features {
        buf.append(&mut format!("\\n- {} {}\\n", status_checkbox_md(feature.status), feature.name).as_bytes().to_vec());
        for scenario in feature.children {
            buf.append(&mut format!("    - {} {}\\n", status_checkbox_md(scenario.status), scenario.name).as_bytes().to_vec());
        }
    }
    readme.write(buf.as_slice()).expect("Unable to write into README.md");
}
