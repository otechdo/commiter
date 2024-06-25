#![allow(clippy::multiple_crate_versions)]

use std::env::consts::OS;
use std::process::Command;

use inquire::{Confirm, Select, Text};

const TYPES: [&str; 69] = [
    "Star",
    "Comet",
    "Nebula",
    "Pulsar",
    "Quasar",
    "Asteroid Belt",
    "Solar Flare",
    "Dwarf Planet",
    "Terraform",
    "Black Hole",
    "Wormhole",
    "Big Bang",
    "Launch",
    "Lightspeed",
    "Mission Control",
    "Spacewalk",
    "Moon Landing",
    "First Contact",
    "Interstellar Communication",
    "Solar Eclipse",
    "Supernova",
    "Meteor Shower",
    "Solar Wind",
    "Lunar Eclipse",
    "Cosmic Dawn",
    "Solar Storm",
    "Lunar Transit",
    "Perihelion",
    "Aphelion",
    "White Dwarf",
    "Red Giant",
    "Neutron Star",
    "Binary Star",
    "Brown Dwarf",
    "Quark Star",
    "Rogue Planet",
    "Stellar Nursery",
    "Planetary Nebula",
    "Globular Cluster",
    "Void",
    "Gravity",
    "Dark Matter",
    "Time Dilation",
    "Spacetime",
    "Gravitational Lensing",
    "Cosmic String",
    "Quantum Fluctuation",
    "Hawking Radiation",
    "Quantum Entanglement",
    "Gravitational Redshift",
    "Space Probe",
    "Station",
    "Rocket Launch",
    "Spacewalk",
    "Space Elevator",
    "Warp Drive",
    "Dyson Sphere",
    "Generation Ship",
    "Lagrange Point",
    "Orbital Maneuver",
    "Mission Control",
    "Moon Landing",
    "First Contact",
    "Interstellar Travel",
    "Rover",
    "Singularity",
    "Relativity",
    "Expansion",
    "Big Crunch",
];
const HELP: [&str; 69] = [
    "Star: New feature or enhancement",
    "Comet: Bug fix or error resolution",
    "Nebula: Code refactoring",
    "Pulsar: Performance improvement",
    "Quasar: Documentation or clarity improvement",
    "Asteroid Belt: Code cleanup and maintenance",
    "Solar Flare: Testing-related changes",
    "Dwarf Planet: Minor updates or fixes",
    "Terraform: Infrastructure changes",
    "Black Hole: Removing large chunks of code or features",
    "Wormhole: Merging branches or connecting code parts",
    "Big Bang: Initial commit or major feature start",
    "Launch: Deploying to production or releasing a version",
    "Lightspeed: Significant performance improvements",
    "Mission Control: Project management changes",
    "Spacewalk: Urgent hotfixes",
    "Moon Landing: Major milestone or goal completion",
    "First Contact: Initial integrations with external systems",
    "Interstellar Communication: Improving documentation or communication",
    "Solar Eclipse: Temporarily masking functionality",
    "Supernova: Major, transformative change",
    "Meteor Shower: Series of small changes or fixes",
    "Solar Wind: Refactoring code structure",
    "Lunar Eclipse: Temporarily disabling a feature",
    "Cosmic Dawn: Initial implementation of a feature",
    "Solar Storm: Rapid, impactful changes",
    "Lunar Transit: Minor, temporary change",
    "Perihelion: Brings the project closer to its goals or objectives",
    "Aphelion: Immediate goals, but is necessary for long-term progress",
    "White Dwarf: Improving code comments or documentation",
    "Red Giant: Expanding a feature or functionality",
    "Neutron Star: Optimizing code for performance",
    "Binary Star: Merging features or components",
    "Brown Dwarf: Undeveloped feature with potential",
    "Quark Star: Experimental or speculative change",
    "Rogue Planet: Independent change",
    "Stellar Nursery: Creation of new components",
    "Planetary Nebula: Removal or deprecation of a component",
    "Globular Cluster: Collection of related changes",
    "Void: Removal of a module, component, or feature",
    "Gravity: Resolving merge conflicts or dependencies",
    "Dark Matter: Fixing unknown or mysterious bugs",
    "Time Dilation: Improving code performance",
    "Spacetime: Changes to date, time, or scheduling",
    "Gravitational Lensing: Altering data or information flow",
    "Cosmic String: Connecting code parts",
    "Quantum Fluctuation: Small, random change",
    "Hawking Radiation: Removing technical debt",
    "Quantum Entanglement: Establishing close relationships between code parts",
    "Gravitational Redshift: Slowing down or reducing code performance",
    "Space Probe: Testing new features or technologies",
    "Station: Creating or improving environments",
    "Rocket Launch: Deploying to production",
    "Spacewalk: Urgent production hotfixes",
    "Space Elevator: Making codebase more accessible",
    "Warp Drive: Significant speed improvement",
    "Dyson Sphere: Comprehensive optimization of a specific area",
    "Generation Ship: Long-term project for a self -sustaining system",
    "Lagrange Point: Stabilizing or balancing code parts",
    "Orbital Maneuver: Changing project direction",
    "Mission Control: Represents project management-related changes",
    "Moon Landing: Celebrates the completion of major milestones",
    "First Contact: Indicates the initial establishment of connections or integrations",
    "Interstellar Travel: Migration to a new architecture or language.",
    "Rover: Exploration of new technologies or approaches.",
    "Singularity: Resolution of a complex or hard-to-reproduce issue",
    "Relativity: Changes related to time, dates, or timestamps.",
    "Expansion: Scaling up the system or increasing capacity",
    "Big Crunch: Reduction of codebase size or removal of features.",
];

fn commit(m: &str) {
    assert!(Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(m)
        .current_dir(".")
        .spawn()
        .unwrap()
        .wait()
        .unwrap()
        .success());
}

fn diff() {
    loop {
        clear();
        assert!(Command::new("git")
            .arg("status")
            .current_dir(".")
            .spawn()
            .unwrap()
            .wait()
            .unwrap()
            .success());
        assert!(Command::new("git")
            .arg("diff")
            .current_dir(".")
            .spawn()
            .unwrap()
            .wait()
            .unwrap()
            .success());
        if Confirm::new("Run git add ?: ")
            .with_default(false)
            .prompt()
            .unwrap()
            .eq(&true)
        {
            assert!(Command::new("git")
                .arg("add")
                .arg(".")
                .current_dir(".")
                .spawn()
                .unwrap()
                .wait()
                .unwrap()
                .success());
            if Confirm::new("Do you want commit changes ? ")
                .with_default(true)
                .prompt()
                .unwrap()
                .eq(&true)
            {
                break;
            }
        }
    }
}

fn fmt() {
    assert!(Command::new("cargo")
        .arg("fmt")
        .current_dir(".")
        .spawn()
        .unwrap()
        .wait()
        .unwrap()
        .success());
    clear();
}

fn zuu() -> bool {
    if Command::new("zuu")
        .current_dir(".")
        .spawn()
        .unwrap()
        .wait()
        .unwrap()
        .success()
    {
        clear();
        return true;
    }
    if Confirm::new("Check the code again ? ")
        .with_default(true)
        .prompt()
        .unwrap()
        .eq(&true)
    {
        return false;
    }
    false
}

fn clear() {
    if OS.eq("windows") {
        assert!(Command::new("cls")
            .current_dir(".")
            .spawn()
            .unwrap()
            .wait()
            .unwrap()
            .success());
    } else {
        assert!(Command::new("clear")
            .current_dir(".")
            .spawn()
            .unwrap()
            .wait()
            .unwrap()
            .success());
    }
}

fn commit_types() -> [&'static str; 69] {
    let mut x = TYPES;
    x.sort_unstable();
    x
}

fn commit_types_with_help() -> [&'static str; 69] {
    let mut x = HELP;
    x.sort_unstable();
    x
}

fn prepare_commit() {
    let help = Confirm::new("Display commit type with help message ?")
        .with_default(false)
        .prompt()
        .unwrap()
        .eq(&true);
    let t = if help {
        Select::new("Select a commit type : ", commit_types_with_help().to_vec())
            .prompt()
            .unwrap()
    } else {
        Select::new("Select a commit type : ", commit_types().to_vec())
            .prompt()
            .unwrap()
    };

    let s = Text::new("Please enter the commit scope : ")
        .prompt()
        .unwrap();
    if s.is_empty() {
        prepare_commit();
    }
    let message = Text::new("Please enter the commit message: ")
        .prompt()
        .unwrap();
    if message.is_empty() {
        prepare_commit();
    }
    let c: String = if help {
        let x: Vec<&str> = t.split(':').collect();
        format!(
            "{}({s}): {}",
            x.first().unwrap(),
            message.to_lowercase().replace('.', "")
        )
    } else {
        format!("{t}({s}): {}", message.to_lowercase().replace('.', ""))
    };
    commit(c.as_str());
}

fn publish() {
    if Confirm::new("Publish ? ")
        .with_default(false)
        .prompt()
        .unwrap()
        .eq(&true)
    {
        assert!(Command::new("cargo")
            .arg("publish")
            .current_dir(".")
            .spawn()
            .unwrap()
            .wait()
            .unwrap()
            .success());
    }
}

fn quit() -> bool {
    Confirm::new("Quit commiter?")
        .with_default(false)
        .prompt()
        .unwrap()
}

fn send() {
    if Confirm::new("Send to remotes?")
        .with_default(false)
        .prompt()
        .unwrap()
        .eq(&true)
    {
        assert!(Command::new("git")
            .arg("push")
            .arg("origin")
            .arg("--all")
            .current_dir(".")
            .spawn()
            .unwrap()
            .wait()
            .unwrap()
            .success());
        assert!(Command::new("git")
            .arg("push")
            .arg("origin")
            .arg("--tags")
            .current_dir(".")
            .spawn()
            .unwrap()
            .wait()
            .unwrap()
            .success());
    }
}

fn main() {
    loop {
        fmt();
        if zuu() {
            diff();
            prepare_commit();
            send();
            publish();
            if quit() {
                break;
            }
        }
    }
    println!("Bye");
}
