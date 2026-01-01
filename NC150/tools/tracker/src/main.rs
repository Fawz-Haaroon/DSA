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
    // Works regardless of where `cargo run` is invoked from
    let exe = std::env::current_exe().expect("cannot locate executable");

    exe.parent().unwrap() // target/debug
        .parent().unwrap() // target
        .parent().unwrap() // tracker
        .parent().unwrap() // tools
        .parent().unwrap() // NC150
        .to_path_buf()
}

/* ------------------------- INPUT PARSING -------------------------- */

fn read_neetcode(path: &Path) -> Vec<(String, Vec<(String, String)>)> {
    let content = fs::read_to_string(path)
        .expect("failed to read neetcode150.txt");

    let mut order = Vec::new();
    let mut map: HashMap<String, Vec<(String, String)>> = HashMap::new();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let mut parts = line.splitn(3, ' ');
        let category = parts.next().expect("missing category").to_string();
        let lc = parts.next().expect("missing lc id").to_string();
        let name = parts.next().expect("missing problem name").to_string();

        if !map.contains_key(&category) {
            order.push(category.clone());
        }

        map.entry(category).or_default().push((lc, name));
    }

    order
        .into_iter()
        .map(|cat| (cat.clone(), map.remove(&cat).unwrap()))
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
    solved: HashSet<String>,
    in_progress: HashSet<String>,
) -> Vec<Category> {
    let mut categories = Vec::new();

    for (cat, mut items) in nc {
        // numeric sort: lc1, lc2, lc10
        items.sort_by_key(|(lc, _)| {
            lc.trim_start_matches("lc")
                .parse::<u32>()
                .expect("invalid lc id")
        });

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

        categories.push(Category {
            name: cat,
            problems,
        });
    }

    categories
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

    let neetcode_path = root.join("neetcode150.txt");
    let solved_path = root.join("solved.toml");

    // Default / progress
    if args.len() == 1 || args.get(1).map(String::as_str) == Some("progress") {
        let neetcode = read_neetcode(&neetcode_path);
        let (solved, in_progress) = read_solved(&solved_path);
        let categories = build_categories(neetcode, solved.clone(), in_progress.clone());

        let mut total = 0;
        let mut solved_instances = 0;

        println!("NeetCode150 Progress\n");

        for cat in &categories {
            let solved_in_cat = cat
                .problems
                .iter()
                .filter(|p| p.status == Status::Solved)
                .count();

            let count = cat.problems.len();
            total += count;
            solved_instances += solved_in_cat;

            println!("{:<25} {:>2} / {}", cat.name, solved_in_cat, count);
        }

        println!();
        println!("Unique solved:             {} / {}", solved.len(), total);
        println!("In progress:               {}", in_progress.len());
        println!("Category solved (overlap): {}", solved_instances);
        return;
    }

    // list commands
    if args.get(1).map(String::as_str) == Some("list") {
        let which = args.get(2).expect("missing list type");

        let neetcode = read_neetcode(&neetcode_path);
        let (solved, in_progress) = read_solved(&solved_path);
        let categories = build_categories(neetcode, solved, in_progress);

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

    // state mutation commands
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
            eprintln!("Commands:");
            eprintln!("  progress");
            eprintln!("  list solved|in-progress|unsolved");
            eprintln!("  solve <lc>");
            eprintln!("  unsolve <lc>");
            eprintln!("  in-progress <lc>");
            std::process::exit(1);
        }
    }

    save_state(&solved_path, &state);
}

