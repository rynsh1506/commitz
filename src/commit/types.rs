use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CommitType {
    pub key: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
struct CommitTypes {
    pub types: Vec<CommitType>,
}

#[derive(Debug, Deserialize)]
pub struct RenderCommit {
    pub chosen_type: String,
    pub final_scope: String,
    pub desc: String,
    pub longer_description: String,
    pub breaking_changes: String,
    pub issue_prefix: String,
    pub issue_refs: String,
}

impl RenderCommit {
    /// Constructor untuk RenderCommit
    pub fn new(
        chosen_type: String,
        final_scope: String,
        desc: String,
        longer_description: String,
        breaking_changes: String,
        issue_prefix: String,
        issue_refs: String,
    ) -> Self {
        Self {
            chosen_type,
            final_scope,
            desc,
            longer_description,
            breaking_changes,
            issue_prefix,
            issue_refs,
        }
    }
}

const COMMIT_TYPES_JSON: &str = r#"
{
  "types": [
    { "key": "feat", "description": "A new feature" },
    { "key": "fix", "description": "A bug fix" },
    { "key": "doc", "description": "Documentation only changes" },
    { "key": "style", "description": "Changes that do not affect the meaning of the code" },
    { "key": "refactor", "description": "A code change that neither fixes a bug nor adds a feature" },
    { "key": "pref", "description": "A code change that improves performance" },
    { "key": "test", "description": "Adding missing tests or correcting existing tests" },
    { "key": "ci", "description": "Continuous Integration related changes" },
    { "key": "chore", "description": "Other changes that do not modify src or test files" }
  ]
}
"#;

impl CommitType {
    pub fn new(key: &str, description: &str) -> Self {
        CommitType {
            key: key.to_string(),
            description: description.to_string(),
        }
    }
    pub fn load() -> Vec<CommitType> {
        let commit_types: CommitTypes =
            serde_json::from_str(COMMIT_TYPES_JSON).expect("Invalid embedded JSON format");
        commit_types.types
    }
}
