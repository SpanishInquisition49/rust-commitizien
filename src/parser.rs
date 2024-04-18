use crate::commit::{CommitFooter, CommitType, ConventionalCommit, BREAKING_CHANGES};
use colored::Colorize;
use regex::Regex;
use std::{process::exit, str::FromStr};

pub fn parse_conventional_commit(commit_msg: &str) -> ConventionalCommit {
    // Split the commit message into parts
    let parts: Vec<&str> = commit_msg.splitn(2, ": ").collect();

    if parts.len() != 2 {
        eprintln!("{}", " Conventional Commit: The commit must have at least the structure: \"<type>: <description>\"".red());
        exit(1);
    }

    let header = parts[0];
    let summary_and_body = parts[1];
    // Parse header
    let mut header_parts = header.splitn(3, '(');
    let commit_type_str = header_parts.next();
    let commit_type = CommitType::from_str(commit_type_str.unwrap_or("Missing")).unwrap();
    let scope = match header_parts.next() {
        Some(scope_and_breaking) => {
            let scope = scope_and_breaking.trim_end_matches(')');
            let scope = scope.trim_end_matches(")!");
            if scope.is_empty() {
                None
            } else {
                Some(scope.to_string())
            }
        }
        None => None,
    };

    // Parse summary and body
    let mut summary_and_body_parts = summary_and_body.splitn(2, "\n\n");
    let summary = summary_and_body_parts.next().unwrap_or("").to_string();
    let body_and_footers = summary_and_body_parts.next();
    let (body, footers) = parse_body_and_footers(body_and_footers);
    let is_breaking_change = header.contains('!') || exitst_breaking_change_footer(&footers);
    let commit = ConventionalCommit {
        commit_type,
        scope,
        is_breaking_change,
        summary,
        body,
        footer: footers,
    };
    validate_commit(&commit);
    commit
}

/// Validate the given commit
/// Printing on  stdout erevy error found.
fn validate_commit(commit: &ConventionalCommit) {
    let mut errors: String = "".to_owned();
    // Check if the type is valid
    match &commit.commit_type {
        CommitType::InvalidType(ct) => {
            errors.push_str(&format!("  Type \"{}\" is not a valid type\n", ct));
        }
        CommitType::MissingType => {
            errors.push_str("  The commit must have a type\n");
        }
        _ => (),
    }
    if commit.summary.is_empty() {
        errors.push_str("  The commit must have a summary\n");
    }
    if !errors.is_empty() {
        eprint!(
            "{}\n{}",
            "Conventional Commit: commit is not valid:".red(),
            errors.red()
        );
        exit(1);
    }
}

fn exitst_breaking_change_footer(footers: &Option<Vec<CommitFooter>>) -> bool {
    match footers {
        None => false,
        Some(footers) => {
            for footer in footers {
                if footer.token == BREAKING_CHANGES {
                    return true;
                }
            }
            false
        }
    }
}

fn parse_body_and_footers(
    body_and_footers: Option<&str>,
) -> (Option<String>, Option<Vec<CommitFooter>>) {
    match body_and_footers {
        Some(body_and_footer) => {
            let mut footers: Vec<CommitFooter> = Vec::new();
            let body_and_footer_parts = body_and_footer.split('\n');
            let mut body = "".to_owned();
            for line in body_and_footer_parts {
                // Check if the line is a footer
                if line.contains(": ") {
                    let mut footer = line.split(": ");
                    let footer = CommitFooter {
                        token: footer.next().unwrap_or("").to_string(),
                        value: footer.next().unwrap_or("").to_string(),
                    };
                    footers.push(footer);
                } else {
                    body.push_str(&format!("{}\n", line));
                }
            }

            let body = if body.is_empty() {
                None
            } else {
                let re = Regex::new(r"\n\n$").unwrap();
                let result = re.replace_all(&body, "");
                Some(result.into_owned())
            };
            let footer = if footers.is_empty() {
                None
            } else {
                Some(footers)
            };
            (body, footer)
        }
        None => (None, None),
    }
}
