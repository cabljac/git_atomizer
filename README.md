# Git Atomizer

A CLI tool for intelligently splitting large feature branches into atomic, semantic commits using git diff analysis and AI-powered suggestions.

## Motivation

Large pull requests are hard to review. This tool aims to help developers break down feature branches into smaller, logical commits that tell a clear story of the changes made.

## Current Status

🚧 **Active Development** - Learning Rust by building something useful!

### Implemented
- [x] Basic git repository interaction using `gix`
- [x] Tree diff analysis between commits
- [x] Change detection (additions, deletions, modifications)

### In Progress
- [ ] Diff analyzer layer
- [ ] AI Semantic grouping of related changes
- [ ] AI integration for intelligent commit suggestions

### Planned
- [ ] Interactive commit selection UI
- [ ] Automatic commit message generation
- [ ] Integration tests with real repositories
- [ ] Performance optimization for large diffs

## Architecture

(Work in progress)

```
src/
├── main.rs              # CLI entry point
├── diff_analyzer/       # Git diff abstraction (planned)
│   ├── mod.rs
│   └── types.rs
└── ai/                  # AI integration layer (planned)
└── mod.rs
```

## Tech Stack

- **Rust**
- **gix** - Pure Rust implementation of Git
- **clap** - Command-line argument parsing
- **anyhow** - nice error handling

## Usage (Current)

```bash
# Compare two branches
cargo run -- --feature feature-branch --base main

# View tree contents
cargo run -- --branch main