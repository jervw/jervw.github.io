#[derive(PartialEq, Eq)]
pub struct Project {
    pub name: &'static str,
    pub description: &'static str,
    pub languages: &'static [&'static str],
    pub url: &'static str,
    pub owner: &'static str,
    pub repo_name: &'static str,
}

pub type ProjectList = [Project; 3];

pub static PROJECT_LIST: ProjectList = [
    Project {
        name: "dono",
        description: "A CLI tool to show your GitHub contributions",
        languages: &["Rust"],
        url: "https://github.com/jervw/dono",
        owner: "jervw",
        repo_name: "dono",
    },
    Project {
        name: "urbaani-cli",
        description: "word dictionary on your terminal",
        languages: &["Rust"],
        url: "https://github.com/jervw/urbaani-cli",
        owner: "jervw",
        repo_name: "urbaani-cli",
    },
    Project {
        name: "chess-engine",
        description: "chess program using Negamax Alpha-Beta Pruning",
        languages: &["C++"],
        url: "https://github.com/jervw/chess-engine",
        owner: "jervw",
        repo_name: "chess-engine",
    },
];
