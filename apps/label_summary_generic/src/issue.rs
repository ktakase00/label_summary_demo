use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct Issue {
    project_id: usize,
    milestone: Option<Milestone>,
    author: User,
    description: Option<String>,
    state: String,
    iid: usize,
    assignees: Vec<User>,
    assignee: Option<User>,
    labels: Vec<String>,
    upvotes: usize,
    downvotes: usize,
    merge_requests_count: usize,
    id: usize,
    title: String,
    updated_at: String,
    created_at: String,
    closed_at: Option<String>,
    closed_by: Option<User>,
    user_notes_count: usize,
    due_date: Option<String>,
    web_url: String,
    references: References,
    time_stats: TimeStat,
    has_tasks: bool,
    #[serde(default)]
    task_status: String,
    confidential: bool,
    discussion_locked: Option<bool>,
    _links: Links,
    task_completion_status: TaskCompletionStatus,
}

#[derive(Debug, Deserialize, Default)]
pub struct Milestone {
    due_date: Option<String>,
    project_id: usize,
    state: String,
    description: Option<String>,
    iid: usize,
    id: usize,
    title: String,
    created_at: String,
    updated_at: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct User {
    state: String,
    web_url: String,
    avatar_url: String,
    username: String,
    id: usize,
    name: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct References {
    short: String,
    relative: String,
    full: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct TimeStat {
    time_estimate: usize,
    total_time_spent: usize,
    human_time_estimate: Option<String>,
    human_total_time_spent: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
pub struct Links {
    #[serde(alias = "self")]
    self_url: String,
    notes: String,
    award_emoji: String,
    project: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct TaskCompletionStatus {
    count: usize,
    completed_count: usize,
}

impl Issue {
    pub fn labels(&self) -> &Vec<String> {
        &self.labels
    }
}
