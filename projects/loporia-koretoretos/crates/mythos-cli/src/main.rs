use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use mythos_core::{InfluenceOutcome, InfluenceReport, Project};

const STARTER_PROJECT: &str = include_str!("../../../templates/loporia-koretoretos.toml");

#[derive(Debug, Parser)]
#[command(
    name = "mythos",
    version,
    about = "Author and validate mythic worldbuilding projects"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Create a starter lore project at the given TOML path.
    Init {
        #[arg(default_value = "mythos.toml")]
        path: PathBuf,
    },
    /// Check IDs, references, chronology, and influence rules.
    Validate {
        #[arg(default_value = "mythos.toml")]
        path: PathBuf,
    },
    /// Print a compact inventory of the project.
    Summary {
        #[arg(default_value = "mythos.toml")]
        path: PathBuf,
    },
    /// Export a readable Markdown setting guide.
    ExportMarkdown { input: PathBuf, output: PathBuf },
    /// Calculate a character's fictional memetic influence state.
    Simulate {
        person_id: String,
        #[arg(default_value = "mythos.toml")]
        path: PathBuf,
    },
}

fn main() -> Result<()> {
    match Cli::parse().command {
        Command::Init { path } => initialize(&path),
        Command::Validate { path } => {
            let project = load_project(&path)?;
            let issues = project.validate();
            if issues.is_empty() {
                println!("Valid: {}", project.project.name);
                Ok(())
            } else {
                for issue in &issues {
                    eprintln!("error: {issue}");
                }
                bail!("{} validation issue(s) found", issues.len());
            }
        }
        Command::Summary { path } => {
            let project = load_project(&path)?;
            ensure_valid(&project)?;
            print_summary(&project);
            Ok(())
        }
        Command::ExportMarkdown { input, output } => {
            let project = load_project(&input)?;
            ensure_valid(&project)?;
            let markdown = export_markdown(&project);
            fs::write(&output, markdown)
                .with_context(|| format!("could not write {}", output.display()))?;
            println!("Wrote {}", output.display());
            Ok(())
        }
        Command::Simulate { person_id, path } => {
            let project = load_project(&path)?;
            ensure_valid(&project)?;
            let report = project.simulate_influence(&person_id)?;
            print_report(&project, &report);
            Ok(())
        }
    }
}

fn initialize(path: &Path) -> Result<()> {
    if path.exists() {
        bail!("refusing to overwrite existing file {}", path.display());
    }
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        fs::create_dir_all(parent)
            .with_context(|| format!("could not create {}", parent.display()))?;
    }
    fs::write(path, STARTER_PROJECT)
        .with_context(|| format!("could not write {}", path.display()))?;
    println!("Created {}", path.display());
    Ok(())
}

fn load_project(path: &Path) -> Result<Project> {
    Project::load(path).with_context(|| format!("could not load {}", path.display()))
}

fn ensure_valid(project: &Project) -> Result<()> {
    let issues = project.validate();
    if !issues.is_empty() {
        for issue in &issues {
            eprintln!("error: {issue}");
        }
        bail!("{} validation issue(s) found", issues.len());
    }
    Ok(())
}

fn print_summary(project: &Project) {
    println!("# {}", project.project.name);
    if !project.project.summary.is_empty() {
        println!("{}", project.project.summary);
    }
    println!(
        "\n{} realm(s), {} spirit(s), {} person/people, {} myth(s), {} event(s), {} relation(s)",
        project.realms.len(),
        project.spirits.len(),
        project.people.len(),
        project.myths.len(),
        project.events.len(),
        project.relations.len()
    );
    for realm in &project.realms {
        let ruler = project
            .spirits
            .iter()
            .find(|spirit| spirit.id == realm.ruler_id)
            .map_or(realm.ruler_id.as_str(), |spirit| spirit.name.as_str());
        println!("\n## {} (ruled by {})", realm.name, ruler);
        println!("{}", realm.doctrine);
        print_bullets("Virtues", &realm.virtues);
        print_bullets("Risks", &realm.risks);
    }
}

fn print_bullets(label: &str, items: &[String]) {
    if !items.is_empty() {
        println!("{label}: {}", items.join(", "));
    }
}

fn print_report(project: &Project, report: &InfluenceReport) {
    println!("Influence simulation for `{}`", report.person_id);
    for (realm_id, score) in &report.scores_by_realm {
        println!("  {realm_id}: {score}");
    }
    match &report.outcome {
        InfluenceOutcome::Unchanged => println!("Outcome: unchanged"),
        InfluenceOutcome::Resonant { realm_id, score } => {
            println!("Outcome: resonant with {realm_id} (score {score})")
        }
        InfluenceOutcome::Transformed {
            realm_id,
            ruler_id,
            score,
        } => {
            let ruler = project
                .spirits
                .iter()
                .find(|spirit| spirit.id == *ruler_id)
                .map_or(ruler_id.as_str(), |spirit| spirit.name.as_str());
            println!("Outcome: transformed by {realm_id}, ruled by {ruler} (score {score})");
        }
        InfluenceOutcome::Contested {
            first_realm_id,
            first_score,
            second_realm_id,
            second_score,
        } => println!(
            "Outcome: contested between {first_realm_id} ({first_score}) and {second_realm_id} ({second_score})"
        ),
    }
}

fn export_markdown(project: &Project) -> String {
    let mut output = format!(
        "# {}\n\n{}\n",
        project.project.name, project.project.summary
    );
    output.push_str("\n## Realms\n");
    for realm in &project.realms {
        let ruler = project
            .spirits
            .iter()
            .find(|spirit| spirit.id == realm.ruler_id)
            .map_or(realm.ruler_id.as_str(), |spirit| spirit.name.as_str());
        output.push_str(&format!(
            "\n### {} — ruled by {}\n\n{}\n",
            realm.name, ruler, realm.doctrine
        ));
        append_markdown_list(&mut output, "Virtues", &realm.virtues);
        append_markdown_list(&mut output, "Risks", &realm.risks);
    }

    output.push_str("\n## Spirits\n");
    for spirit in &project.spirits {
        let realm = project
            .realms
            .iter()
            .find(|realm| realm.id == spirit.realm_id)
            .map_or(spirit.realm_id.as_str(), |realm| realm.name.as_str());
        output.push_str(&format!("\n### {}\n\nRealm: {}\n", spirit.name, realm));
        append_markdown_list(&mut output, "Epithets", &spirit.epithets);
        append_markdown_list(&mut output, "Domains", &spirit.domains);
    }

    output.push_str("\n## People\n");
    for person in &project.people {
        output.push_str(&format!(
            "\n### {}\n\nOrigin: {}\n\nResilience: {}\n",
            person.name, person.origin, person.resilience
        ));
    }

    output.push_str("\n## Myths\n");
    for myth in &project.myths {
        output.push_str(&format!("\n### {}\n\n{}\n", myth.title, myth.summary));
        append_markdown_list(&mut output, "Themes", &myth.themes);
        append_markdown_list(&mut output, "Source notes", &myth.source_notes);
    }

    output.push_str("\n## Timeline\n");
    let mut events: Vec<_> = project.events.iter().collect();
    events.sort_by_key(|event| (event.year, event.id.as_str()));
    for event in events {
        output.push_str(&format!(
            "\n- **{} — {}:** {}\n",
            event.year, event.title, event.summary
        ));
    }

    output.push_str("\n## Relations\n");
    for relation in &project.relations {
        output.push_str(&format!(
            "\n- **{}** — {} — **{}**",
            relation.from_id, relation.kind, relation.to_id
        ));
        if !relation.note.is_empty() {
            output.push_str(&format!(": {}", relation.note));
        }
        output.push('\n');
    }
    output
}

fn append_markdown_list(output: &mut String, title: &str, items: &[String]) {
    if !items.is_empty() {
        output.push_str(&format!("\n**{title}:** {}\n", items.join(", ")));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn markdown_export_sorts_events_chronologically() {
        let project: Project = toml::from_str(
            r#"
[project]
name = "Chronicle"
summary = "A test."

[[events]]
id = "later"
title = "Later"
year = 20
summary = "Second"

[[events]]
id = "earlier"
title = "Earlier"
year = 10
summary = "First"
"#,
        )
        .expect("project TOML should parse");

        let output = export_markdown(&project);
        assert!(output.find("Earlier").unwrap() < output.find("Later").unwrap());
    }
}
