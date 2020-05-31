use failure::Error;
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
    fn save(&self, notifications: &Vec<NewSlackNotification>) -> Result<usize, Error>;
    fn find_not_send(&self, slack_url: &str) -> Result<Vec<Paper>, Error>;
    fn mark_as_send(&self, slack_url: &str, paper_id: &PaperId) -> Result<usize, Error>;
    fn delete(&self, slack_url: &str, paper_id: &PaperId) -> Result<(), Error>;
}
