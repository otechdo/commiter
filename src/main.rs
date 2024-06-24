#![allow(clippy::multiple_crate_versions)]

use std::env::consts::OS;
use std::process::Command;

use inquire::{Confirm, Select, Text};

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

fn prepare_commit() {
    let mut scopes = [
        "auth",
        "email-template",
        "devops",
        "localization",
        "api",
        "logging",
        "navigation",
        "middleware",
        "service",
        "model",
        "view",
        "controllers",
        "subscription",
        "cli",
        "lang",
        "theme",
        "perf",
        "search",
        "payment",
        "deps",
        "forms",
        "design",
        "seo",
        "ui",
        "ux",
        "router",
        "db",
    ];
    let mut commit_types = [
        "build", "ci", "docs", "improve", "feat", "fix", "perf", "refactor", "test", "e2e",
    ];
    scopes.sort_unstable();
    commit_types.sort_unstable();
    let t = Select::new("Select a commit type : ", commit_types.to_vec())
        .prompt()
        .unwrap();
    let s = Select::new("Select a commit scope : ", scopes.to_vec())
        .prompt()
        .unwrap();
    let message = Text::new("Please enter the commit message: ")
        .prompt()
        .unwrap();
    if message.is_empty() {
        prepare_commit();
    }
    let c = format!("{t}({s}): {}", message.to_lowercase().replace('.', ""));
    commit(c.as_str());
}

fn quit() -> bool {
    Confirm::new("Quit commiter ?")
        .with_default(false)
        .prompt()
        .unwrap()
}

fn send() {
    if Confirm::new("Send to remotes ?")
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
            if quit() {
                break;
            }
        }
    }
    println!("bye");
}
