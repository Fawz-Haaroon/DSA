use std::{
    collections::{HashMap, HashSet},
    fs,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Status {
    Solved,
    InProgress,
    Unsolved,
}

#[derive(Debug)]
struct Problem {
    lc: String,
    name: String,
    status: Status,
}

#[derive(Debug)]
struct Category {
    name: String,
    problems: Vec<Problem>,
}

#[derive(Debug, Deserialize, Serialize)]
struct SolvedFile {
    #[serde(default)]
    solved: HashMap<String, bool>,
    #[serde(default)]
    in_progress: HashMap<String, bool>,
}

/* ------------------------- PATH RESOLUTION ------------------------- */

fn project_root() -> PathBuf {
    let mut dir = std::env::current_exe()
        .expect("cannot locate executable")
        .canonicalize()
        .expect("cannot canonicalize path");

    loop {
        if dir.join("solved.toml").exists() {
            return dir;
        }
        if !dir.pop() {
            panic!("could not find project root (missing solved.toml)");
        }
    }
}

/* ------------------------- INPUT PARSING -------------------------- */

fn read_neetcode(path: &Path) -> Vec<(String, Vec<(String, String)>)> {
    let content = fs::read_to_string(path)
        .expect("failed to read neetcode250.txt");

    let mut order: Vec<String> = Vec::new();
    let mut map: HashMap<String, Vec<(String, String)>> = HashMap::new();

    for (idx, raw) in content.lines().enumerate() {
        let line = raw.trim();

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // STRICT: <category> <lcID> <slug>
        let mut parts = line.splitn(3, ' ');
        let (cat, lc, name) = match (parts.next(), parts.next(), parts.next()) {
            (Some(c), Some(l), Some(n)) => (c, l, n),
            _ => {
                eprintln!("Skipping malformed line {}: {}", idx + 1, raw);
                continue;
            }
        };

        if !lc.starts_with("lc") {
            eprintln!("Skipping invalid lc id at line {}: {}", idx + 1, raw);
            continue;
        }

        if !map.contains_key(cat) {
            order.push(cat.to_string());
        }

        map.entry(cat.to_string())
            .or_default()
            .push((lc.to_string(), name.to_string()));
    }

    order
        .into_iter()
        .map(|cat| {
            let mut items = map.remove(&cat).unwrap();
            items.sort_by_key(|(lc, _)| {
                lc.trim_start_matches("lc")
                    .parse::<u32>()
                    .unwrap_or(u32::MAX)
            });
            (cat, items)
        })
        .collect()
}

fn read_solved(path: &Path) -> (HashSet<String>, HashSet<String>) {
    let content = fs::read_to_string(path)
        .expect("failed to read solved.toml");

    let parsed: SolvedFile =
        toml::from_str(&content).expect("invalid solved.toml");

    let solved = parsed
        .solved
        .into_iter()
        .filter(|(_, v)| *v)
        .map(|(k, _)| k)
        .collect();

    let in_progress = parsed
        .in_progress
        .into_iter()
        .filter(|(_, v)| *v)
        .map(|(k, _)| k)
        .collect();

    (solved, in_progress)
}

/* ------------------------- CORE LOGIC ----------------------------- */

fn build_categories(
    nc: Vec<(String, Vec<(String, String)>)>,
    solved: &HashSet<String>,
    in_progress: &HashSet<String>,
) -> Vec<Category> {
    nc.into_iter()
        .map(|(cat, items)| {
            let problems = items
                .into_iter()
                .map(|(lc, name)| {
                    let status = if solved.contains(&lc) {
                        Status::Solved
                    } else if in_progress.contains(&lc) {
                        Status::InProgress
                    } else {
                        Status::Unsolved
                    };
                    Problem { lc, name, status }
                })
                .collect();

            Category {
                name: cat,
                problems,
            }
        })
        .collect()
}

/* ------------------------- STATE MUTATION -------------------------- */

fn load_state(path: &Path) -> SolvedFile {
    let content = fs::read_to_string(path)
        .expect("failed to read solved.toml");

    toml::from_str(&content).expect("invalid solved.toml")
}

fn save_state(path: &Path, state: &SolvedFile) {
    let toml = toml::to_string_pretty(state)
        .expect("failed to serialize solved.toml");

    fs::write(path, toml)
        .expect("failed to write solved.toml");
}

/* ------------------------- CLI OUTPUT ----------------------------- */

fn list_by_status(categories: &[Category], status: Status) {
    for cat in categories {
        let matches: Vec<_> = cat
            .problems
            .iter()
            .filter(|p| p.status == status)
            .collect();

        if matches.is_empty() {
            continue;
        }

        println!("{}", cat.name);
        for p in matches {
            println!("  {:<6} {}", p.lc, p.name.replace('-', " "));
        }
        println!();
    }
}

/* ------------------------- MAIN ---------------------------------- */

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let root = project_root();

    let neetcode_path = root.join("neetcode250.txt");
    let solved_path = root.join("solved.toml");

    let neetcode = read_neetcode(&neetcode_path);
    let (solved, in_progress) = read_solved(&solved_path);
    let categories = build_categories(neetcode, &solved, &in_progress);

    let total: usize = categories.iter().map(|c| c.problems.len()).sum();

    // progress (default)
    if args.len() == 1 || args.get(1).map(String::as_str) == Some("progress") {
        let mut solved_instances = 0;

        println!("NeetCode Progress\n");

        for cat in &categories {
            let solved_in_cat = cat
                .problems
                .iter()
                .filter(|p| p.status == Status::Solved)
                .count();

            solved_instances += solved_in_cat;
            println!("{:<25} {:>2} / {}", cat.name, solved_in_cat, cat.problems.len());
        }

        println!();
        println!("Unique solved:             {} / {}", solved.len(), total);
        println!("In progress:               {}", in_progress.len());
        println!("Category solved (overlap): {}", solved_instances);
        return;
    }

    // list
    if args.get(1).map(String::as_str) == Some("list") {
        let which = args.get(2).expect("missing list type");
        match which.as_str() {
            "solved" => list_by_status(&categories, Status::Solved),
            "in-progress" => list_by_status(&categories, Status::InProgress),
            "unsolved" => list_by_status(&categories, Status::Unsolved),
            _ => {
                eprintln!("list options: solved | in-progress | unsolved");
                std::process::exit(1);
            }
        }
        return;
    }

    // mutation
    let cmd = args.get(1).expect("missing command");
    let lc = args.get(2).expect("missing lc id").to_string();

    let mut state = load_state(&solved_path);

    match cmd.as_str() {
        "solve" => {
            state.solved.insert(lc.clone(), true);
            state.in_progress.remove(&lc);
            println!("Marked {} as SOLVED", lc);
        }
        "unsolve" => {
            state.solved.remove(&lc);
            state.in_progress.remove(&lc);
            println!("Removed {} from solved/in-progress", lc);
        }
        "in-progress" => {
            state.in_progress.insert(lc.clone(), true);
            println!("Marked {} as IN PROGRESS", lc);
        }
        _ => {
            eprintln!("Unknown command: {}", cmd);
            std::process::exit(1);
        }
    }

    save_state(&solved_path, &state);
}
