use std::env;
use std::process::exit;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum CommitType {
    Build,
    Chore,
    Ci,
    Docs,
    Feat,
    Fix,
    Perf,
    Refactor,
    Revert,
    Style,
    Test,
}

#[derive(Debug, PartialEq)]
struct ConventionalCommit {
    commit_type: CommitType,
    scope: Option<String>,
    is_breaking_change: bool,
    summary: String,
    body: Option<String>,
    footer: Option<(String, String)>, // (token, value)
}

impl FromStr for CommitType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "build" => Ok(CommitType::Build),
            "chore" => Ok(CommitType::Chore),
            "ci" => Ok(CommitType::Ci),
            "docs" => Ok(CommitType::Docs),
            "feat" => Ok(CommitType::Feat),
            "fix" => Ok(CommitType::Fix),
            "perf" => Ok(CommitType::Perf),
            "refactor" => Ok(CommitType::Refactor),
            "revert" => Ok(CommitType::Revert),
            "style" => Ok(CommitType::Style),
            "test" => Ok(CommitType::Test),
            _ => Err(()),
        }
    }
}

fn parse_conventional_commit(commit_msg: &str) -> Option<ConventionalCommit> {
    // Split the commit message into parts
    let parts: Vec<&str> = commit_msg.splitn(2, ": ").collect();

    if parts.len() != 2 {
        return None;
    }

    let header = parts[0];
    let summary_and_body = parts[1];

    // Parse header
    let mut header_parts = header.splitn(3, "(");
    let commit_type_str = header_parts.next()?;
    let commit_type = CommitType::from_str(commit_type_str).ok()?;
    let scope = match header_parts.next() {
        Some(scope_and_breaking) => {
            let scope = scope_and_breaking.trim_end_matches(")");
            if scope.is_empty() {
                None
            } else {
                Some(scope.to_string())
            }
        }
        None => None,
    };
    let is_breaking_change = header.contains("!");

    // Parse summary and body
    let mut summary_and_body_parts = summary_and_body.split("\n\n");
    let summary = summary_and_body_parts.next()?.to_string();
    let body_and_footer = summary_and_body_parts.next();
    let (body, footer) = match body_and_footer {
        Some(body_and_footer) => {
            let mut body_and_footer_parts = body_and_footer.splitn(2, "\n");
            let body = body_and_footer_parts.next().map(|s| s.to_string());
            let footer = body_and_footer_parts.next().map(|s| {
                let mut footer_parts = s.splitn(2, ": ");
                let token = footer_parts.next().unwrap_or("").to_string();
                let value = footer_parts.next().unwrap_or("").to_string();
                (token, value)
            });
            (body, footer)
        }
        None => (None, None),
    };

    Some(ConventionalCommit {
        commit_type,
        scope,
        is_breaking_change,
        summary,
        body,
        footer,
    })
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let commit_msg = &args[1];
    match parse_conventional_commit(commit_msg) {
        None => {
            println!("Invalid Conventional Commit message.");
            exit(1)
        }
        Some(commit) => println!("Conventional Commit parsed successfully: {:?}", commit),
    };
}
