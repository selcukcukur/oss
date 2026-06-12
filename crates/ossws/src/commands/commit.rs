use ossws_core::{
    normalize_subject, normalize_token, validate_commit_subject, CommitKind as CoreCommitKind,
};

use crate::definitions::{CommitCommand, CommitSubcommand};

pub fn run(command: CommitCommand) -> Result<(), String> {
    match command.command {
        CommitSubcommand::Validate(args) => {
            validate_commit_subject(&args.subject)?;
            println!("valid commit subject");
            Ok(())
        }
        CommitSubcommand::Build(args) => {
            let scope = normalize_token(&args.scope);
            let subject = normalize_subject(&args.subject);
            let marker = if args.breaking { "!" } else { "" };
            let kind: CoreCommitKind = args.kind.into();
            let built = format!("{kind}({scope}){marker}: {subject}");
            validate_commit_subject(&built)?;
            println!("{built}");
            Ok(())
        }
    }
}
