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
        name: "nixos-config",
        description: "dotfiles and nixos system configurations",
        languages: &["Nix"],
        url: "https://github.com/jervw/nixos-config",
        owner: "jervw",
        repo_name: "nixos-config",
    },
    Project {
        name: "dono",
        description: "a CLI tool to show your GitHub contributions",
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
];
