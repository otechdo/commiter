#![allow(clippy::multiple_crate_versions)]

use inquire::{Confirm, Select, Text};
use std::env::consts::OS;
use std::process::Command;

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
        if Confirm::new("Run git add ? : ")
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

fn commit_types() -> [&'static str; 72] {
    let mut x = [
        "Star",
        "Comet",
        "Nebula",
        "Pulsar",
        "Quasar",
        "Asteroid Belt",
        "Solar Flare",
        "Dwarf Planet",
        "Supermassive Black Hole",
        "Eclipse",
        "Supernova",
        "Red Giant",
        "White Dwarf",
        "Neutron Star",
        "Black Hole",
        "Wormhole",
        "Solar Wind",
        "Lunar Eclipse",
        "Cosmic Microwave",
        "Gravitational",
        "Event Horizon",
        "Big Bang",
        "Launch",
        "Probe",
        "Slingshot Maneuver",
        "Wormhole",
        "Lightspeed",
        "Mission Control",
        "Spacewalk",
        "Moon Landing",
        "First Contact",
        "Terraform",
        "Interstellar Communication",
        "UFO",
        "Houston, We Have a Problem",
        "E.T.Phone Home",
        "Warp Speed",
        "Space Junk",
        "Houston, We Have a Solution",
        "Alien Technology",
        "May the Force Be With You",
        "Celestial Dance",
        "Cosmic Symphony",
        "Stardust Whispers",
        "Nebula's Embrace",
        "Stellar Nursery",
        "Quantum Leap",
        "Cosmic Alignment",
        "Warp Drive",
        "Hyperspace",
        "Jump",
        "First Contact",
        "Stargate",
        "Prime",
        "Directive",
        "Holo",
        "Replicator",
        "Cosmic Dawn",
        "Galactic Harmony",
        "Solar Wind",
        "Lunar Eclipse",
        "Stellar Evolution",
        "Nebula's Breath",
        "Red Shift",
        "Blue Shift",
        "Parallax",
        "Accretion Disk",
        "Singularity",
        "Quantum Entanglement",
        "Dark Matter",
        "Gravitational Lensing",
        "Time Dilation",
    ];
    x.sort_unstable();
    x
}

fn prepare_commit() {
    let t = Select::new("Select a commit type : ", commit_types().to_vec())
        .prompt()
        .unwrap();

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
    let c = format!("{t})({s}): {}", message.to_lowercase().replace('.', ""));
    commit(c.as_str());
}

fn publish() {
    if Confirm::new("Publish?")
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
