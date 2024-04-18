use colored::Colorize;
use std::{fmt::Display, str::FromStr};

pub const BREAKING_CHANGES: &str = "BREAKING CHANGE";

#[derive(Debug, PartialEq)]
pub enum CommitType {
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
    InvalidType(String),
    MissingType,
}

impl FromStr for CommitType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Ignore the ! for BREAKING CHANGE commits
        let t: String = s.replace('!', "");
        match t.as_str() {
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
            "tests" => Ok(CommitType::Test),
            _ => {
                if t.is_empty() {
                    Ok(CommitType::MissingType)
                } else {
                    Ok(CommitType::InvalidType(t))
                }
            }
        }
    }
}

impl Display for CommitType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt: String = "".to_owned();
        match self {
            CommitType::Build => fmt.push_str(&format!("{}", "\"build\"".yellow())),
            CommitType::Chore => fmt.push_str(&format!("{}", "\"chore\"".yellow())),
            CommitType::Ci => fmt.push_str(&format!("{}", "\"ci\"".yellow())),
            CommitType::Docs => fmt.push_str(&format!("{}", "\"docs\"".yellow())),
            CommitType::Feat => fmt.push_str(&format!("{}", "\"feat\"".yellow())),
            CommitType::Fix => fmt.push_str(&format!("{}", "\"fix\"".yellow())),
            CommitType::Perf => fmt.push_str(&format!("{}", "\"perf\"".yellow())),
            CommitType::Refactor => fmt.push_str(&format!("{}", "\"refactor\"".yellow())),
            CommitType::Revert => fmt.push_str(&format!("{}", "\"revert\"".yellow())),
            CommitType::Style => fmt.push_str(&format!("{}", "\"style\"".yellow())),
            CommitType::Test => fmt.push_str(&format!("{}", "\"test\"".yellow())),
            CommitType::InvalidType(_) => fmt.push_str("Invalid Commit Type"),
            CommitType::MissingType => fmt.push_str("Missing Commit Type"),
        };
        write!(f, "{}", fmt)
    }
}

#[derive(Debug, PartialEq)]
pub struct CommitFooter {
    pub token: String,
    pub value: String,
}

impl Display for CommitFooter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{{}: {}}}",
            format_string(self.token.clone()),
            format_string(self.value.clone())
        )
    }
}

#[derive(Debug, PartialEq)]
pub struct ConventionalCommit {
    pub commit_type: CommitType,
    pub scope: Option<String>,
    pub is_breaking_change: bool,
    pub summary: String,
    pub body: Option<String>,
    pub footer: Option<Vec<CommitFooter>>,
}

impl Display for ConventionalCommit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = "".to_owned();
        fmt.push_str(&format!("- Commit Type: {}\n", self.commit_type));
        match &self.scope {
            Some(s) => fmt.push_str(&format!("- Scope: {}\n", format_string(s.clone()))),
            None => fmt.push_str("- Scope:\n"),
        };
        fmt.push_str(&format!(
            "- Is Breaking Change: {}\n",
            self.is_breaking_change.to_string().purple()
        ));
        fmt.push_str(&format!(
            "- Summary: {}\n",
            format_string(self.summary.clone())
        ));
        match &self.body {
            Some(b) => fmt.push_str(&format!("- Body: {}\n", format_string(b.clone()))),
            None => fmt.push_str("- Body:\n"),
        }
        match &self.footer {
            Some(footers) => {
                fmt.push_str("- Footers: [");
                for footer in footers {
                    fmt.push_str(&format!("{},", footer));
                }
            }
            None => fmt.push_str("- Footers: []"),
        }

        write!(f, "{}", fmt)
    }
}

fn format_string(string: String) -> String {
    format!("{}{}{}", "\"".yellow(), string.yellow(), "\"".yellow())
}
