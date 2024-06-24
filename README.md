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
  │       └─ Commit Scope: animations|bazel|benchpress|common|compiler|compiler-cli|core|
  │                          elements|forms|http|language-service|localize|platform-browser|
  │                          platform-browser-dynamic|platform-server|router|service-worker|
  │                          upgrade|zone.js|packaging|changelog|docs-infra|migrations|
  │                          devtools
  │
  └─ Commit Type: build|ci|docs|feat|fix|perf|refactor|test
```