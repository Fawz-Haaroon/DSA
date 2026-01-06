# NeetCode 150

All problems live under `problems/`, grouped by topic. Each topic is a Rust crate. You solve problems directly in those files. Nothing is inferred or auto-detected.

Progress is tracked manually in `solved.toml`.

Core rule:
If a problem exists in `solved`, it is considered solved. If it does not exist in `solved` but exists in `in_progress`, it is considered in progress. Anything else is unsolved. Higher priority always wins; lower priority is ignored.

That’s it.

## Tracking

Edit `solved.toml` directly or use the tracker CLI.

Example:

```
[solved]
lc217 = true

[in_progress]
lc207 = true
```

## Commands

Run from anywhere:

```
cargo run --manifest-path NC150/tools/tracker/Cargo.toml -- progress
```

Mark solved:

```
tracker solve lc217
```

Mark in progress:

```
tracker in-progress lc207
```

Clear state:

```
tracker unsolve lc207
```

List by state:

```
tracker list solved
tracker list in-progress
tracker list unsolved
```

