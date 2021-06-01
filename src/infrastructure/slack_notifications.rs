use diesel::prelude::*;
use chrono::NaiveDateTime;

use super::sqlite::SQLite;
use crate::db::models::{SlackNotification, Paper, Category};
use crate::db::schema::*;
use crate::domain::slack_notification as I;
use crate::domain::arxiv_paper as J;

#[derive(Insertable, Debug)]
#[table_name = "slack_notifications"]
struct NewSlackNotification {
    pub paper_id: i32,
    pub slack_url: String,
    pub updated_at: NaiveDateTime
}

#[derive(Clone)]
pub struct SlackNotificationRepository {}

impl SlackNotificationRepository {
    pub fn new() -> Self {
        Self {}
    }
}

impl I::SlackNotificationRepositoryTrait for SlackNotificationRepository {
    fn save(&self, notifications: &Vec<I::NewSlackNotification>) -> anyhow::Result<usize> {
        let new_notifications = notifications.iter().map(|n| NewSlackNotification {
            paper_id: n.paper_id.0,
            slack_url: n.slack_url.clone(),
            updated_at: n.updated,
        }).collect::<Vec<NewSlackNotification>>();
        let conn = SQLite::create().connect();
        let n = diesel::insert_or_ignore_into(slack_notifications::table)
            .values(&new_notifications)
            .execute(&conn)?;
        Ok(n)
    }

    fn find_not_send(&self, slack_url: &str) -> anyhow::Result<Vec<J::Paper>> {
        let conn = SQLite::create().connect();
        let notifications: Vec<(SlackNotification, Paper, Category)> = slack_notifications::table
            .inner_join(papers::table
                .inner_join(categories::table))
            .filter(slack_notifications::slack_url.eq(slack_url.to_string())
                .and(slack_notifications::send.eq(false)))
            .select((slack_notifications::all_columns, papers::all_columns, categories::all_columns))
            .load::<(SlackNotification, Paper, Category)>(&conn)?;
        Ok(notifications.iter().map(|n| J::Paper {
            id: J::PaperId(n.1.id),
            title: n.1.title.to_string(),
            url: n.1.url.to_string(),
            pdf_url: n.1.pdf_url.to_string(),
            authors: vec![],
            category: J::Category {
                id: n.2.id,
                name: n.2.name.to_string(),
            },
            summary: n.1.summary.to_string(),
            comment: n.1.comment.to_string(),
            is_accepted: n.1.accepted > 0,
            updated: n.1.updated,
            published: n.1.published,
        }).collect())
    }

    fn mark_as_send(&self, slack_url: &str, paper_id: &J::PaperId) -> anyhow::Result<usize> {
        let conn = SQLite::create().connect();
        let target = slack_notifications::table
            .filter(
                slack_notifications::paper_id.eq(&paper_id.0)
                    .and(slack_notifications::slack_url.eq(slack_url)));
        let n_update = diesel::update(target)
            .set(slack_notifications::send.eq(true))
            .execute(&conn)?;
        Ok(n_update)
    }

    fn delete(&self, slack_url: &str, paper_id: &J::PaperId) -> anyhow::Result<()> {
        let conn = SQLite::create().connect();
        let target = slack_notifications::table
            .filter(slack_notifications::paper_id.eq(&paper_id.0)
                    .and(slack_notifications::slack_url.eq(slack_url)));
        diesel::delete(target)
            .execute(&conn)?;
        Ok(())
    }
}