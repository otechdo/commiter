# What is it ?

It's a git commit helper with format, check source code quality before commit with commit message like AngularJS.

## Remote

Call your remote `origin`.

## Installation

```bash
cargo install zuu commiter
```

## Usage

```bash
commiter
```

# Commit Message Format

## Commit Message Header

```
<type>(<scope>): <short summary>
  │       │             │
  │       │             └─ Summary in present tense. Not capitalized. No period at the end.
  │       │   
  │       └─ Commit Scope: auth|email-template|devops|localization|api|logging|navigation|middleware|service|
  │                         model|view|controllers|subscription|cli|ui|ux|seo|deps|lang|theme|perf|search|
  │                         payment|forms|design|router|db
  │
  └─ Commit Type: build|ci|docs|improve|feat|fix|perf|refactor|test|e2e
```