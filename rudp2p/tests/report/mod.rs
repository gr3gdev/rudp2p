use async_trait::async_trait;
use cucumber::*;
use std::{
    fmt::{Debug, Display},
    io,
    sync::Arc,
    time::{Duration, SystemTime},
};

trait ToMarkdown {
    fn to_md(&self) -> String;
}

#[derive(Clone, Debug)]
pub struct Markdown<Out: io::Write> {
    output: Out,
    features: Vec<Feature>,
    started: Option<SystemTime>,
    logs: Vec<String>,
}

#[derive(Clone, Debug)]
struct Feature {
    uri: Option<String>,
    name: String,
    elements: Vec<Element>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Status {
    Failed,
    Skipped,
    Passed,
    Undefined,
    Ambiguous,
}

#[derive(Clone, Debug)]
struct HookResult {
    result: RunResult,
}

#[derive(Clone, Debug)]
struct RunResult {
    status: Status,
    duration: u128,
    error_message: Option<String>,
}

#[derive(Clone, Debug)]
struct Step {
    line: usize,
    name: String,
    result: RunResult,
}

#[derive(Clone, Debug)]
struct Element {
    after: Vec<HookResult>,
    before: Vec<HookResult>,
    r#type: &'static str,
    line: usize,
    name: String,
    steps: Vec<Step>,
}

impl Feature {
    fn new(feature: &gherkin::Feature) -> Self {
        Self {
            uri: feature
                .path
                .as_ref()
                .and_then(|p| p.to_str())
                .map(str::to_owned),
            name: feature.name.clone(),

            elements: vec![],
        }
    }

    fn example_expansion_err(err: &feature::ExpandExamplesError) -> Self {
        Self {
            uri: err
                .path
                .as_ref()
                .and_then(|p| p.to_str())
                .map(str::to_owned),
            name: String::new(),
            elements: vec![Element {
                after: vec![],
                before: vec![],
                r#type: "scenario",
                line: 0,
                name: String::new(),
                steps: vec![Step {
                    line: err.pos.line,
                    name: "scenario".into(),
                    result: RunResult {
                        status: Status::Failed,
                        duration: 0,
                        error_message: Some(err.to_string()),
                    },
                }],
            }],
        }
    }

    fn parsing_err(err: &gherkin::ParseFileError) -> Self {
        let path = match err {
            gherkin::ParseFileError::Reading { path, .. }
            | gherkin::ParseFileError::Parsing { path, .. } => path,
        }
        .to_str()
        .map(str::to_owned);

        Self {
            uri: path.clone(),
            name: String::new(),
            elements: vec![Element {
                after: vec![],
                before: vec![],
                r#type: "scenario",
                line: 0,
                name: String::new(),
                steps: vec![Step {
                    line: 0,
                    name: "scenario".into(),
                    result: RunResult {
                        status: Status::Failed,
                        duration: 0,
                        error_message: Some(err.to_string()),
                    },
                }],
            }],
        }
    }
}

impl PartialEq<gherkin::Feature> for Feature {
    fn eq(&self, feature: &gherkin::Feature) -> bool {
        self.uri
            .as_ref()
            .and_then(|uri| {
                feature
                    .path
                    .as_ref()
                    .and_then(|p| p.to_str())
                    .map(|path| uri == path)
            })
            .unwrap_or_default()
            && self.name == feature.name
    }
}

impl Element {
    fn new(rule: Option<&gherkin::Rule>, scenario: &gherkin::Scenario, ty: &'static str) -> Self {
        Self {
            after: vec![],
            before: vec![],
            r#type: ty,
            line: scenario.position.line,
            name: format!(
                "{}{}",
                rule.map(|r| format!("{} ", r.name)).unwrap_or_default(),
                scenario.name.clone(),
            ),
            steps: vec![],
        }
    }

    fn duration(&self) -> u128 {
        self.steps.iter().map(|s| s.result.duration).sum()
    }
}

impl<Out: io::Write> Markdown<Out> {
    #[must_use]
    pub fn new<W: Debug + World>(output: Out) -> writer::Normalize<W, Self> {
        Self {
            output,
            features: vec![],
            started: None,
            logs: vec![],
        }
        .normalized()
    }

    pub fn is_failed(&self) -> bool {
        self.features.iter().any(|f| {
            f.elements
                .iter()
                .any(|e| e.steps.iter().any(|s| s.result.status == Status::Failed))
        })
    }

    fn handle_scenario_event<W>(
        &mut self,
        feature: &Arc<gherkin::Feature>,
        rule: Option<&gherkin::Rule>,
        scenario: &gherkin::Scenario,
        ev: event::Scenario<W>,
        meta: event::Metadata,
    ) {
        use event::Scenario;

        match ev {
            Scenario::Started => {}
            Scenario::Hook(ty, ev) => {
                self.handle_hook_event(feature, rule, scenario, ty, ev, meta);
            }
            Scenario::Background(st, ev) => {
                self.handle_step_event(feature, rule, scenario, "background", &st, ev, meta);
            }
            Scenario::Step(st, ev) => {
                self.handle_step_event(feature, rule, scenario, "scenario", &st, ev, meta);
            }
            Scenario::Log(msg) => {
                self.logs.push(msg);
            }
            Scenario::Finished => {}
        }
    }

    fn handle_hook_event<W>(
        &mut self,
        feature: &gherkin::Feature,
        rule: Option<&gherkin::Rule>,
        scenario: &gherkin::Scenario,
        hook_ty: event::HookType,
        event: event::Hook<W>,
        meta: event::Metadata,
    ) {
        use event::{Hook, HookType};

        let mut duration = || {
            let started = self
                .started
                .take()
                .unwrap_or_else(|| panic!("No `Started` event for `{hook_ty} Hook`"));
            meta.at
                .duration_since(started)
                .unwrap_or_else(|e| {
                    panic!(
                        "Failed to compute duration between {:?} and  {started:?}: {e}",
                        meta.at,
                    );
                })
                .as_nanos()
        };

        let res = match event {
            Hook::Started => {
                self.started = Some(meta.at);
                return;
            }
            Hook::Passed => HookResult {
                result: RunResult {
                    status: Status::Passed,
                    duration: duration(),
                    error_message: None,
                },
            },
            Hook::Failed(_, info) => {
                let default_error = String::from("Unable to read errors");
                HookResult {
                    result: RunResult {
                        status: Status::Failed,
                        duration: duration(),
                        error_message: Some(
                            info.downcast_ref::<String>()
                                .unwrap_or_else(|| &default_error)
                                .clone(),
                        ),
                    },
                }
            }
        };

        let el = self.mut_or_insert_element(feature, rule, scenario, "scenario");
        match hook_ty {
            HookType::Before => el.before.push(res),
            HookType::After => el.after.push(res),
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn handle_step_event<W>(
        &mut self,
        feature: &gherkin::Feature,
        rule: Option<&gherkin::Rule>,
        scenario: &gherkin::Scenario,
        ty: &'static str,
        step: &gherkin::Step,
        event: event::Step<W>,
        meta: event::Metadata,
    ) {
        let mut duration = || {
            let started = self
                .started
                .take()
                .unwrap_or_else(|| panic!("No `Started` event for `Step` '{}'", step.value));
            meta.at
                .duration_since(started)
                .unwrap_or_else(|e| {
                    panic!(
                        "Failed to compute duration between {:?} and \
                         {started:?}: {e}",
                        meta.at,
                    );
                })
                .as_nanos()
        };

        let result = match event {
            event::Step::Started => {
                self.started = Some(meta.at);
                _ = self.mut_or_insert_element(feature, rule, scenario, ty);
                return;
            }
            event::Step::Passed(..) => RunResult {
                status: Status::Passed,
                duration: duration(),
                error_message: None,
            },
            event::Step::Failed(_, loc, _, err) => {
                let status = match &err {
                    event::StepError::NotFound => Status::Undefined,
                    event::StepError::AmbiguousMatch(..) => Status::Ambiguous,
                    event::StepError::Panic(..) => Status::Failed,
                };
                RunResult {
                    status,
                    duration: duration(),
                    error_message: Some(format!(
                        "{}{err}",
                        loc.map(|l| format!("Matched: {}:{}:{}\n", l.path, l.line, l.column,))
                            .unwrap_or_default(),
                    )),
                }
            }
            event::Step::Skipped => RunResult {
                status: Status::Skipped,
                duration: duration(),
                error_message: None,
            },
        };

        let step = Step {
            line: step.position.line,
            name: step.value.clone(),
            result,
        };
        let el = self.mut_or_insert_element(feature, rule, scenario, ty);
        el.steps.push(step);
    }

    fn mut_or_insert_element(
        &mut self,
        feature: &gherkin::Feature,
        rule: Option<&gherkin::Rule>,
        scenario: &gherkin::Scenario,
        ty: &'static str,
    ) -> &mut Element {
        let f_pos = self
            .features
            .iter()
            .position(|f| f == feature)
            .unwrap_or_else(|| {
                self.features.push(Feature::new(feature));
                self.features.len() - 1
            });
        let f = self
            .features
            .get_mut(f_pos)
            .unwrap_or_else(|| unreachable!());

        let el_pos = f
            .elements
            .iter()
            .position(|el| {
                el.name
                    == format!(
                        "{}{}",
                        rule.map(|r| format!("{} ", r.name)).unwrap_or_default(),
                        scenario.name,
                    )
                    && el.line == scenario.position.line
                    && el.r#type == ty
            })
            .unwrap_or_else(|| {
                f.elements.push(Element::new(rule, scenario, ty));
                f.elements.len() - 1
            });
        f.elements.get_mut(el_pos).unwrap_or_else(|| unreachable!())
    }
}

#[async_trait(?Send)]
impl<W: World + Debug, Out: io::Write> Writer<W> for Markdown<Out> {
    type Cli = cli::Empty;

    async fn handle_event(
        &mut self,
        event: parser::Result<Event<event::Cucumber<W>>>,
        _: &Self::Cli,
    ) {
        use event::{Cucumber, Rule};

        match event.map(event::Event::split) {
            Err(parser::Error::Parsing(e)) => {
                let feature = Feature::parsing_err(&e);
                self.features.push(feature);
            }
            Err(parser::Error::ExampleExpansion(e)) => {
                let feature = Feature::example_expansion_err(&e);
                self.features.push(feature);
            }
            Ok((Cucumber::Feature(f, event::Feature::Scenario(sc, ev)), meta)) => {
                self.handle_scenario_event(&f, None, &sc, ev.event, meta);
            }
            Ok((Cucumber::Feature(f, event::Feature::Rule(r, Rule::Scenario(sc, ev))), meta)) => {
                self.handle_scenario_event(&f, Some(&r), &sc, ev.event, meta);
            }
            Ok((Cucumber::Finished, _)) => {
                let mut report = String::from("# Report");
                for feature in self.features.clone() {
                    report.push_str(&feature.to_md());
                }
                report.push_str("\n---\n");
                report.push_str(&details_logs(&self.logs));
                self.output
                    .write(report.as_bytes())
                    .expect("Unable to write Markdown report");
            }
            _ => {}
        }
    }
}

fn details(summary: &str, details: &str) -> String {
    format!("\n\n<details>\n<summary>{summary}</summary>\n\n{details}\n</details>\n\n")
}

fn details_logs(logs: &Vec<String>) -> String {
    details("Logs", &format!("```\n{}\n```", logs.join("")))
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Status::Failed => "![Failed](https://img.shields.io/badge/Failed-red)",
            Status::Skipped => "![Skipped](https://img.shields.io/badge/Skipped-yellow)",
            Status::Passed => "![Passed](https://img.shields.io/badge/Passed-green)",
            Status::Undefined => "![Undefined](https://img.shields.io/badge/Undefined-grey)",
            Status::Ambiguous => "![Ambiguous](https://img.shields.io/badge/Ambiguous-purple)",
        };
        f.write_str(str)
    }
}

impl Display for RunResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(error) = &self.error_message {
            f.write_fmt(format_args!(
                "{} {}\n\n```\n{}\n```",
                self.status,
                convert_duration(self.duration),
                error
            ))
        } else {
            f.write_fmt(format_args!(
                "{} {}",
                self.status,
                convert_duration(self.duration)
            ))
        }
    }
}

fn convert_duration(duration: u128) -> String {
    let duration = Duration::from_nanos(duration as u64);
    format!(
        "![Duration](https://img.shields.io/badge/{}s-{}ms-blue)",
        duration.as_secs(),
        duration.subsec_millis()
    )
}

impl ToMarkdown for Step {
    fn to_md(&self) -> String {
        format!("  - {} (line {}) {}", self.name, self.line, self.result,)
    }
}

impl ToMarkdown for Element {
    fn to_md(&self) -> String {
        let report_steps = self
            .steps
            .iter()
            .map(Step::to_md)
            .collect::<Vec<String>>()
            .join("\n");
        let nb_steps_passed = format!(
            "![Passed](https://img.shields.io/badge/{}-Passed-green)",
            self.steps
                .iter()
                .filter(|s| s.result.status == Status::Passed)
                .count()
        );
        let nb_steps_skipped = format!(
            "![Skipped](https://img.shields.io/badge/{}-Skipped-yellow)",
            self.steps
                .iter()
                .filter(|s| s.result.status == Status::Skipped)
                .count()
        );
        let nb_steps_failed = format!(
            "![Failed](https://img.shields.io/badge/{}-Failed-red)",
            self.steps
                .iter()
                .filter(|s| s.result.status == Status::Failed)
                .count()
        );
        let mut before = String::new();
        if !self.before.is_empty() {
            before = details(
                "Hoos before",
                &self
                    .before
                    .iter()
                    .map(|h| format!("- {}", h.result))
                    .collect::<Vec<String>>()
                    .join("\n"),
            );
        }
        let mut after = String::new();
        if !self.after.is_empty() {
            after = details(
                "Hook after",
                &self
                    .after
                    .iter()
                    .map(|h| format!("- {}", h.result))
                    .collect::<Vec<String>>()
                    .join("\n"),
            );
        }
        format!(
            "- {} {} {} {} {}{}{}{}",
            self.name,
            nb_steps_passed,
            nb_steps_skipped,
            nb_steps_failed,
            convert_duration(self.duration()),
            before,
            details("Steps", &report_steps),
            after,
        )
    }
}

impl ToMarkdown for Feature {
    fn to_md(&self) -> String {
        let report_elements = self
            .elements
            .iter()
            .map(Element::to_md)
            .collect::<Vec<String>>()
            .join("\n");
        let feature_passed = self
            .elements
            .iter()
            .all(|e| e.steps.iter().all(|s| s.result.status == Status::Passed));
        let feature_failed = self
            .elements
            .iter()
            .any(|e| e.steps.iter().any(|s| s.result.status == Status::Failed));
        let status = if feature_passed {
            "![Passed](https://img.shields.io/badge/Passed-green)"
        } else if feature_failed {
            "![Failed](https://img.shields.io/badge/Failed-red)"
        } else {
            "![Undefined](https://img.shields.io/badge/Undefined-grey)"
        };
        format!(
            "\n\n## Feature : {} {}\n\n{}",
            self.name, status, report_elements
        )
    }
}
