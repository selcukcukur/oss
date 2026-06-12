pub fn validate_commit_subject(subject: &str) -> Result<(), String> {
    if subject.trim() != subject {
        return Err("commit subject must not start or end with whitespace".to_string());
    }

    if subject.ends_with('.') {
        return Err("commit subject must not end with a period".to_string());
    }

    if subject != subject.to_lowercase() {
        return Err("commit subject must be lowercase".to_string());
    }

    let (head, summary) = subject
        .split_once(": ")
        .ok_or_else(|| "commit subject must use type(scope): subject".to_string())?;

    if summary.is_empty() {
        return Err("commit subject summary must not be empty".to_string());
    }

    let open = head
        .find('(')
        .ok_or_else(|| "commit subject must include a scope".to_string())?;
    let close = head
        .find(')')
        .ok_or_else(|| "commit subject scope must close with ')'".to_string())?;

    if close <= open + 1 {
        return Err("commit subject scope must not be empty".to_string());
    }

    let kind = &head[..open];
    if !matches!(
        kind,
        "feat" | "perf" | "docs" | "fix" | "refactor" | "chore"
    ) {
        return Err(format!("invalid commit type: {kind}"));
    }

    let after_scope = &head[close + 1..];
    if !after_scope.is_empty() && after_scope != "!" {
        return Err("only ! may appear after the scope".to_string());
    }

    let scope = &head[open + 1..close];
    validate_token(scope, "scope")?;

    Ok(())
}

pub fn validate_token(value: &str, label: &str) -> Result<(), String> {
    if value != value.to_lowercase() {
        return Err(format!("{label} must be lowercase"));
    }

    if !value.chars().all(|character| {
        character.is_ascii_lowercase() || character.is_ascii_digit() || character == '-'
    }) {
        return Err(format!(
            "{label} may only contain lowercase letters, digits, and hyphens"
        ));
    }

    Ok(())
}

pub fn normalize_token(value: &str) -> String {
    value
        .trim()
        .to_lowercase()
        .chars()
        .map(|character| {
            if character.is_ascii_lowercase() || character.is_ascii_digit() {
                character
            } else {
                '-'
            }
        })
        .collect::<String>()
        .split('-')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

pub fn normalize_subject(value: &str) -> String {
    value.trim().trim_end_matches('.').to_lowercase()
}

pub fn validate_release_heading(line: &str) -> Result<(), String> {
    let value = line
        .strip_prefix("## ")
        .ok_or_else(|| format!("invalid release heading: {line}"))?;
    let (version, date) = value
        .split_once(" - ")
        .ok_or_else(|| format!("release heading must use VERSION - YYYY-MM-DD: {line}"))?;

    if version.starts_with('v') {
        return Err(format!("release version must not start with v: {line}"));
    }

    if !is_iso_date(date) {
        return Err(format!("release date must use YYYY-MM-DD: {line}"));
    }

    Ok(())
}

pub fn is_iso_date(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.len() == 10
        && bytes[4] == b'-'
        && bytes[7] == b'-'
        && bytes[..4].iter().all(u8::is_ascii_digit)
        && bytes[5..7].iter().all(u8::is_ascii_digit)
        && bytes[8..].iter().all(u8::is_ascii_digit)
}
