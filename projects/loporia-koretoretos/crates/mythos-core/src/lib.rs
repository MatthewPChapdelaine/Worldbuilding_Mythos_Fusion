use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub project: ProjectMetadata,
    #[serde(default)]
    pub rules: InfluenceRules,
    #[serde(default)]
    pub realms: Vec<Realm>,
    #[serde(default)]
    pub spirits: Vec<Spirit>,
    #[serde(default)]
    pub people: Vec<Person>,
    #[serde(default)]
    pub myths: Vec<Myth>,
    #[serde(default)]
    pub events: Vec<Event>,
    #[serde(default)]
    pub relations: Vec<Relation>,
    #[serde(default)]
    pub encounters: Vec<Encounter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectMetadata {
    pub name: String,
    #[serde(default = "default_version")]
    pub version: String,
    #[serde(default)]
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfluenceRules {
    #[serde(default = "default_resonance_threshold")]
    pub resonance_threshold: u32,
    #[serde(default = "default_transformation_threshold")]
    pub transformation_threshold: u32,
    #[serde(default = "default_contest_margin")]
    pub contest_margin: u32,
}

impl Default for InfluenceRules {
    fn default() -> Self {
        Self {
            resonance_threshold: default_resonance_threshold(),
            transformation_threshold: default_transformation_threshold(),
            contest_margin: default_contest_margin(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Realm {
    pub id: String,
    pub name: String,
    pub ruler_id: String,
    pub doctrine: String,
    #[serde(default)]
    pub virtues: Vec<String>,
    #[serde(default)]
    pub risks: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spirit {
    pub id: String,
    pub name: String,
    pub realm_id: String,
    #[serde(default)]
    pub epithets: Vec<String>,
    #[serde(default)]
    pub domains: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub origin: String,
    #[serde(default)]
    pub resilience: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Myth {
    pub id: String,
    pub title: String,
    pub summary: String,
    #[serde(default)]
    pub themes: Vec<String>,
    #[serde(default)]
    pub source_notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: String,
    pub title: String,
    pub year: i32,
    pub summary: String,
    #[serde(default)]
    pub myth_ids: Vec<String>,
    #[serde(default)]
    pub predecessor_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relation {
    pub from_id: String,
    pub to_id: String,
    pub kind: String,
    #[serde(default)]
    pub note: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Encounter {
    pub person_id: String,
    pub spirit_id: String,
    pub intensity: u8,
    pub duration_hours: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InfluenceOutcome {
    Unchanged,
    Resonant {
        realm_id: String,
        score: u32,
    },
    Transformed {
        realm_id: String,
        ruler_id: String,
        score: u32,
    },
    Contested {
        first_realm_id: String,
        first_score: u32,
        second_realm_id: String,
        second_score: u32,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InfluenceReport {
    pub person_id: String,
    pub scores_by_realm: BTreeMap<String, u32>,
    pub outcome: InfluenceOutcome,
}

#[derive(Debug, Error)]
pub enum ProjectLoadError {
    #[error("could not read project file: {0}")]
    Io(#[from] std::io::Error),
    #[error("could not parse project TOML: {0}")]
    Parse(#[from] toml::de::Error),
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum SimulationError {
    #[error("no person has id `{0}`")]
    UnknownPerson(String),
}

impl Project {
    pub fn load(path: impl AsRef<Path>) -> Result<Self, ProjectLoadError> {
        let contents = fs::read_to_string(path)?;
        Ok(toml::from_str(&contents)?)
    }

    pub fn validate(&self) -> Vec<String> {
        let mut issues = Vec::new();
        let mut ids = HashSet::new();

        if self.project.name.trim().is_empty() {
            issues.push("project.name must not be empty".to_owned());
        }
        if self.project.version.trim().is_empty() {
            issues.push("project.version must not be empty".to_owned());
        }
        if self.rules.resonance_threshold == 0 {
            issues.push("rules.resonance_threshold must be greater than zero".to_owned());
        }
        if self.rules.transformation_threshold == 0 {
            issues.push("rules.transformation_threshold must be greater than zero".to_owned());
        }
        if self.rules.resonance_threshold > self.rules.transformation_threshold {
            issues.push(
                "rules.resonance_threshold must not exceed rules.transformation_threshold"
                    .to_owned(),
            );
        }

        let mut all_ids = HashSet::new();
        for id in self
            .realms
            .iter()
            .map(|item| item.id.as_str())
            .chain(self.spirits.iter().map(|item| item.id.as_str()))
            .chain(self.people.iter().map(|item| item.id.as_str()))
            .chain(self.myths.iter().map(|item| item.id.as_str()))
            .chain(self.events.iter().map(|item| item.id.as_str()))
        {
            if !is_valid_id(id) {
                issues.push(format!(
                    "`{id}` is not a valid id; use lowercase kebab-case"
                ));
            }
            if !all_ids.insert(id) {
                issues.push(format!("duplicate id `{id}`"));
            }
            ids.insert(id.to_owned());
        }

        let realms: HashMap<_, _> = self
            .realms
            .iter()
            .map(|item| (item.id.as_str(), item))
            .collect();
        let spirits: HashMap<_, _> = self
            .spirits
            .iter()
            .map(|item| (item.id.as_str(), item))
            .collect();
        let people: HashSet<_> = self.people.iter().map(|item| item.id.as_str()).collect();
        let myths: HashSet<_> = self.myths.iter().map(|item| item.id.as_str()).collect();
        let events: HashMap<_, _> = self
            .events
            .iter()
            .map(|item| (item.id.as_str(), item))
            .collect();

        for realm in &self.realms {
            if realm.name.trim().is_empty() {
                issues.push(format!("realm `{}` has an empty name", realm.id));
            }
            if realm.doctrine.trim().is_empty() {
                issues.push(format!("realm `{}` has an empty doctrine", realm.id));
            }
            match spirits.get(realm.ruler_id.as_str()) {
                Some(ruler) if ruler.realm_id == realm.id => {}
                Some(_) => issues.push(format!(
                    "realm `{}` ruler `{}` belongs to a different realm",
                    realm.id, realm.ruler_id
                )),
                None => issues.push(format!(
                    "realm `{}` references missing ruler `{}`",
                    realm.id, realm.ruler_id
                )),
            }
        }

        for spirit in &self.spirits {
            if spirit.name.trim().is_empty() {
                issues.push(format!("spirit `{}` has an empty name", spirit.id));
            }
            if !realms.contains_key(spirit.realm_id.as_str()) {
                issues.push(format!(
                    "spirit `{}` references missing realm `{}`",
                    spirit.id, spirit.realm_id
                ));
            }
        }

        for person in &self.people {
            if person.name.trim().is_empty() {
                issues.push(format!("person `{}` has an empty name", person.id));
            }
            if person.resilience > 100 {
                issues.push(format!(
                    "person `{}` resilience must be between 0 and 100",
                    person.id
                ));
            }
        }

        for myth in &self.myths {
            if myth.title.trim().is_empty() {
                issues.push(format!("myth `{}` has an empty title", myth.id));
            }
            if myth.summary.trim().is_empty() {
                issues.push(format!("myth `{}` has an empty summary", myth.id));
            }
        }

        for event in &self.events {
            if event.title.trim().is_empty() {
                issues.push(format!("event `{}` has an empty title", event.id));
            }
            if event.summary.trim().is_empty() {
                issues.push(format!("event `{}` has an empty summary", event.id));
            }
            for myth_id in &event.myth_ids {
                if !myths.contains(myth_id.as_str()) {
                    issues.push(format!(
                        "event `{}` references missing myth `{myth_id}`",
                        event.id
                    ));
                }
            }
            for predecessor_id in &event.predecessor_ids {
                match events.get(predecessor_id.as_str()) {
                    Some(predecessor) if predecessor.year > event.year => issues.push(format!(
                        "event `{}` predates its predecessor `{predecessor_id}`",
                        event.id
                    )),
                    Some(_) => {}
                    None => issues.push(format!(
                        "event `{}` references missing predecessor `{predecessor_id}`",
                        event.id
                    )),
                }
            }
        }
        self.validate_event_cycles(&mut issues);

        for relation in &self.relations {
            if !ids.contains(&relation.from_id) {
                issues.push(format!(
                    "relation references missing source `{}`",
                    relation.from_id
                ));
            }
            if !ids.contains(&relation.to_id) {
                issues.push(format!(
                    "relation references missing target `{}`",
                    relation.to_id
                ));
            }
            if relation.kind.trim().is_empty() {
                issues.push("relation kind must not be empty".to_owned());
            }
        }

        for encounter in &self.encounters {
            if !people.contains(encounter.person_id.as_str()) {
                issues.push(format!(
                    "encounter references missing person `{}`",
                    encounter.person_id
                ));
            }
            if !spirits.contains_key(encounter.spirit_id.as_str()) {
                issues.push(format!(
                    "encounter references missing spirit `{}`",
                    encounter.spirit_id
                ));
            }
            if !(1..=100).contains(&encounter.intensity) {
                issues.push(format!(
                    "encounter intensity for `{}` must be between 1 and 100",
                    encounter.person_id
                ));
            }
            if encounter.duration_hours == 0 {
                issues.push(format!(
                    "encounter duration for `{}` must be greater than zero",
                    encounter.person_id
                ));
            }
        }

        issues
    }

    pub fn simulate_influence(&self, person_id: &str) -> Result<InfluenceReport, SimulationError> {
        let person = self
            .people
            .iter()
            .find(|person| person.id == person_id)
            .ok_or_else(|| SimulationError::UnknownPerson(person_id.to_owned()))?;

        let realms: HashMap<_, _> = self
            .realms
            .iter()
            .map(|realm| (realm.id.as_str(), realm))
            .collect();
        let spirits: HashMap<_, _> = self
            .spirits
            .iter()
            .map(|spirit| (spirit.id.as_str(), spirit))
            .collect();
        let mut raw_scores = BTreeMap::<String, u32>::new();

        for encounter in self
            .encounters
            .iter()
            .filter(|encounter| encounter.person_id == person_id)
        {
            if let Some(spirit) = spirits.get(encounter.spirit_id.as_str()) {
                let exposure = u32::from(encounter.intensity)
                    .saturating_mul(u32::from(encounter.duration_hours));
                let total = raw_scores.entry(spirit.realm_id.clone()).or_default();
                *total = total.saturating_add(exposure);
            }
        }

        let resilience_factor = 100 - u32::from(person.resilience.min(100));
        let scores_by_realm: BTreeMap<_, _> = raw_scores
            .into_iter()
            .map(|(realm_id, score)| (realm_id, score.saturating_mul(resilience_factor) / 100))
            .collect();
        let mut ranked: Vec<_> = scores_by_realm
            .iter()
            .filter(|(_, score)| **score > 0)
            .map(|(realm_id, score)| (realm_id.clone(), *score))
            .collect();
        ranked.sort_by(|left, right| right.1.cmp(&left.1).then_with(|| left.0.cmp(&right.0)));

        let outcome = match ranked.as_slice() {
            [] => InfluenceOutcome::Unchanged,
            [(realm_id, score), ..] if *score < self.rules.resonance_threshold => {
                InfluenceOutcome::Unchanged
            }
            [(realm_id, score), ..] if *score < self.rules.transformation_threshold => {
                InfluenceOutcome::Resonant {
                    realm_id: realm_id.clone(),
                    score: *score,
                }
            }
            [(first_id, first_score), (second_id, second_score), ..]
                if *second_score >= self.rules.transformation_threshold
                    && first_score.saturating_sub(*second_score) <= self.rules.contest_margin =>
            {
                InfluenceOutcome::Contested {
                    first_realm_id: first_id.clone(),
                    first_score: *first_score,
                    second_realm_id: second_id.clone(),
                    second_score: *second_score,
                }
            }
            [(realm_id, score), ..] => match realms.get(realm_id.as_str()) {
                Some(realm) => InfluenceOutcome::Transformed {
                    realm_id: realm.id.clone(),
                    ruler_id: realm.ruler_id.clone(),
                    score: *score,
                },
                None => InfluenceOutcome::Unchanged,
            },
        };

        Ok(InfluenceReport {
            person_id: person_id.to_owned(),
            scores_by_realm,
            outcome,
        })
    }

    fn validate_event_cycles(&self, issues: &mut Vec<String>) {
        fn visit<'a>(
            id: &'a str,
            events: &HashMap<&'a str, &'a Event>,
            visiting: &mut HashSet<&'a str>,
            visited: &mut HashSet<&'a str>,
        ) -> bool {
            if visited.contains(id) {
                return false;
            }
            if !visiting.insert(id) {
                return true;
            }
            if let Some(event) = events.get(id) {
                for predecessor in &event.predecessor_ids {
                    if events.contains_key(predecessor.as_str())
                        && visit(predecessor, events, visiting, visited)
                    {
                        return true;
                    }
                }
            }
            visiting.remove(id);
            visited.insert(id);
            false
        }

        let events: HashMap<_, _> = self
            .events
            .iter()
            .map(|event| (event.id.as_str(), event))
            .collect();
        let mut visiting = HashSet::new();
        let mut visited = HashSet::new();
        for event in &self.events {
            if visit(event.id.as_str(), &events, &mut visiting, &mut visited) {
                issues.push("event predecessor relationships contain a cycle".to_owned());
                return;
            }
        }
    }
}

fn is_valid_id(id: &str) -> bool {
    !id.is_empty()
        && !id.starts_with('-')
        && !id.ends_with('-')
        && !id.contains("--")
        && id
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
}

fn default_version() -> String {
    "0.1.0".to_owned()
}

fn default_resonance_threshold() -> u32 {
    40
}

fn default_transformation_threshold() -> u32 {
    100
}

fn default_contest_margin() -> u32 {
    15
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_project() -> Project {
        toml::from_str(
            r#"
[project]
name = "Test Mythos"

[rules]
resonance_threshold = 40
transformation_threshold = 100
contest_margin = 15

[[realms]]
id = "loporia"
name = "Loporia"
ruler_id = "metatron"
doctrine = "Order and record"

[[realms]]
id = "koretoretos"
name = "Koretoretos"
ruler_id = "azazel"
doctrine = "Craft and autonomy"

[[spirits]]
id = "metatron"
name = "Metatron"
realm_id = "loporia"

[[spirits]]
id = "azazel"
name = "Azazel"
realm_id = "koretoretos"

[[people]]
id = "artisan"
name = "The Artisan"

[[encounters]]
person_id = "artisan"
spirit_id = "metatron"
intensity = 10
duration_hours = 10
"#,
        )
        .expect("test project TOML should parse")
    }

    #[test]
    fn valid_project_has_no_validation_issues() {
        assert!(sample_project().validate().is_empty());
    }

    #[test]
    fn detects_missing_references_and_duplicate_ids() {
        let mut project = sample_project();
        project.spirits[1].id = "metatron".to_owned();
        project.encounters[0].spirit_id = "missing-spirit".to_owned();

        let issues = project.validate();
        assert!(issues.iter().any(|issue| issue.contains("duplicate id")));
        assert!(issues.iter().any(|issue| issue.contains("missing-spirit")));
    }

    #[test]
    fn influence_outcomes_reflect_resilience_and_competing_realms() {
        let mut project = sample_project();
        let transformed = project
            .simulate_influence("artisan")
            .expect("person exists");
        assert_eq!(
            transformed.outcome,
            InfluenceOutcome::Transformed {
                realm_id: "loporia".to_owned(),
                ruler_id: "metatron".to_owned(),
                score: 100,
            }
        );

        project.encounters.push(Encounter {
            person_id: "artisan".to_owned(),
            spirit_id: "azazel".to_owned(),
            intensity: 10,
            duration_hours: 10,
        });
        assert!(matches!(
            project
                .simulate_influence("artisan")
                .expect("person exists")
                .outcome,
            InfluenceOutcome::Contested { .. }
        ));

        project.people[0].resilience = 100;
        assert_eq!(
            project
                .simulate_influence("artisan")
                .expect("person exists")
                .outcome,
            InfluenceOutcome::Unchanged
        );
    }

    #[test]
    fn influence_distinguishes_resonance_from_no_signal() {
        let mut project = sample_project();
        project.encounters[0].intensity = 5;
        assert_eq!(
            project
                .simulate_influence("artisan")
                .expect("person exists")
                .outcome,
            InfluenceOutcome::Resonant {
                realm_id: "loporia".to_owned(),
                score: 50,
            }
        );

        project.encounters[0].intensity = 3;
        assert_eq!(
            project
                .simulate_influence("artisan")
                .expect("person exists")
                .outcome,
            InfluenceOutcome::Unchanged
        );
    }

    #[test]
    fn event_predecessor_cycles_are_reported() {
        let mut project = sample_project();
        project.events = vec![
            Event {
                id: "first-event".to_owned(),
                title: "First".to_owned(),
                year: 1,
                summary: String::new(),
                myth_ids: Vec::new(),
                predecessor_ids: vec!["second-event".to_owned()],
            },
            Event {
                id: "second-event".to_owned(),
                title: "Second".to_owned(),
                year: 2,
                summary: String::new(),
                myth_ids: Vec::new(),
                predecessor_ids: vec!["first-event".to_owned()],
            },
        ];

        assert!(project
            .validate()
            .iter()
            .any(|issue| issue.contains("contain a cycle")));
    }

    #[test]
    fn unknown_person_is_a_typed_error() {
        assert_eq!(
            sample_project().simulate_influence("unknown"),
            Err(SimulationError::UnknownPerson("unknown".to_owned()))
        );
    }
}
