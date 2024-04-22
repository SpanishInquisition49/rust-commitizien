mod commit;
mod parser;

use colored::Colorize;

use crate::parser::parse_conventional_commit;
use std::env;

fn help() {
    println!(
        "{} {}
Check if the commmit message follow the Conventional Commit Standard.",
        "Usage:".green(),
        "rs-commitizien <commit message>".blue(),
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => {
            let commit_message = &args[1];
            let commit = parse_conventional_commit(commit_message);
            println!("Conventional Commit parsed successfully:\n{}", commit);
        }
        _ => help(),
    }
}

#[cfg(test)]
mod tests {
    use crate::commit::{CommitFooter, CommitType, ConventionalCommit, BREAKING_CHANGES};
    use crate::parse_conventional_commit;

    #[test]
    fn commit_message_with_description_and_breaking_change_footer() {
        let commit_message = "feat: allow provided config object to extend other configs\n\nBREAKING CHANGE: `extends` key in config file is now used for extending other config files";
        let commit = parse_conventional_commit(commit_message);
        let commit_footer = CommitFooter {
            token: "BREAKING CHANGE".to_owned(),
            value: "`extends` key in config file is now used for extending other config files"
                .to_owned(),
        };
        let commit_footers: Vec<CommitFooter> = vec![commit_footer];
        let expected_commit = ConventionalCommit {
            commit_type: CommitType::Feat,
            scope: None,
            is_breaking_change: true,
            summary: "allow provided config object to extend other configs".to_owned(),
            body: None,
            footer: Some(commit_footers),
        };
        assert_eq!(commit, expected_commit);
    }

    #[test]
    fn commit_message_with_bang_to_draw_attention_to_breaking_change() {
        let commit_message = "feat!: send an email to the customer when a product is shipped";
        let commit = parse_conventional_commit(commit_message);
        let expected_commit = ConventionalCommit {
            commit_type: CommitType::Feat,
            scope: None,
            is_breaking_change: true,
            summary: "send an email to the customer when a product is shipped".to_owned(),
            body: None,
            footer: None,
        };
        assert_eq!(commit, expected_commit);
    }

    #[test]
    fn commit_message_with_scope_and_bang_to_draw_attention_to_breaking_change() {
        let commit_message = "feat(api)!: send an email to the customer when a product is shipped";
        let commit = parse_conventional_commit(commit_message);
        let expected_commit = ConventionalCommit {
            commit_type: CommitType::Feat,
            scope: Some("api".to_owned()),
            is_breaking_change: true,
            summary: "send an email to the customer when a product is shipped".to_owned(),
            body: None,
            footer: None,
        };
        assert_eq!(commit, expected_commit);
    }

    #[test]
    fn commit_message_with_both_bang_and_breaking_change_footer() {
        let commit_message = "chore!: drop support for Node 6\n\nBREAKING CHANGE: use JavaScript features not available in Node 6.";
        let commit = parse_conventional_commit(commit_message);
        let commit_footer = CommitFooter {
            token: BREAKING_CHANGES.to_owned(),
            value: "use JavaScript features not available in Node 6.".to_owned(),
        };
        let expected_commit = ConventionalCommit {
            commit_type: CommitType::Chore,
            scope: None,
            is_breaking_change: true,
            summary: "drop support for Node 6".to_owned(),
            body: None,
            footer: Some(vec![commit_footer]),
        };
        assert_eq!(commit, expected_commit);
    }

    #[test]
    fn commit_message_with_no_body() {
        let commit_message = "docs: correct spelling of CHANGELOG";
        let commit = parse_conventional_commit(commit_message);
        let expected_commit = ConventionalCommit {
            commit_type: CommitType::Docs,
            scope: None,
            is_breaking_change: false,
            summary: "correct spelling of CHANGELOG".to_owned(),
            body: None,
            footer: None,
        };
        assert_eq!(commit, expected_commit);
    }

    #[test]
    fn commit_message_with_scope() {
        let commit_message = "feat(lang): add Polish language";
        let commit = parse_conventional_commit(commit_message);
        let expected_commit = ConventionalCommit {
            commit_type: CommitType::Feat,
            scope: Some("lang".to_owned()),
            is_breaking_change: false,
            summary: "add Polish language".to_owned(),
            body: None,
            footer: None,
        };
        assert_eq!(commit, expected_commit);
    }

    #[test]
    fn commit_message_with_multi_paragraph_body_and_multiple_footers() {
        let commit_message = "fix: prevent racing of requests

Introduce a request id and a reference to latest request. Dismiss
incoming responses other than from latest request.

Remove timeouts which were used to mitigate the racing issue but are
obsolete now.

Reviewed-by: Z
Refs: #123";
        let commit = parse_conventional_commit(commit_message);
        let expected_commit = ConventionalCommit {
            commit_type: CommitType::Fix,
            scope: None,
            is_breaking_change: false,
            summary: "prevent racing of requests".to_owned(),
            body: Some(
                "Introduce a request id and a reference to latest request. Dismiss
incoming responses other than from latest request.

Remove timeouts which were used to mitigate the racing issue but are
obsolete now."
                    .to_owned(),
            ),
            footer: Some(vec![
                CommitFooter {
                    token: "Reviewed-by".to_owned(),
                    value: "Z".to_owned(),
                },
                CommitFooter {
                    token: "Refs".to_owned(),
                    value: "#123".to_owned(),
                },
            ]),
        };
        assert_eq!(commit, expected_commit);
    }
}
