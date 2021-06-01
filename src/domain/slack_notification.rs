use chrono::NaiveDateTime;
use super::arxiv_paper::{Paper, PaperId};

#[derive(Debug, Clone)]
pub struct SlackNotification {
    pub id: i32,
    pub slack_url: String,
    pub send: bool,
    pub paper: Paper
}

#[derive(Debug, Clone)]
pub struct NewSlackNotification {
    pub paper_id: PaperId,
    pub slack_url: String,
    pub updated: NaiveDateTime,
}

pub trait SlackNotificationRepositoryTrait {
    fn save(&self, notifications: &Vec<NewSlackNotification>) -> anyhow::Result<usize>;
    fn find_not_send(&self, slack_url: &str) -> anyhow::Result<Vec<Paper>>;
    fn mark_as_send(&self, slack_url: &str, paper_id: &PaperId) -> anyhow::Result<usize>;
    fn delete(&self, slack_url: &str, paper_id: &PaperId) -> anyhow::Result<()>;
}
