1. split into modules:
  - main.rs   for CLI handling
  - diff_analyzer/mod.rs  extract gix stuff here
    - grouping (for g)
  - ai/mod.rs  for AI integration
  - 

API something like:


```rust
//  in main.rs after using gix to get repo

let analyzer = DiffAnalyzer::new(repo);

let ai = CommitSuggester::new(options);

let changes = analyzer.analyze(base,feature);
let suggestions = ai.suggest_groupings(&changes)?;

```