use failure::Error;

use crate::domain::arxiv_paper::{Paper, PaperId};
use crate::domain::slack_notification::{NewSlackNotification, SlackNotificationRepositoryTrait};


pub trait SlackNotificationUseCaseTrait {
    fn enqueue_slack_notifications(&self, notifications: &Vec<NewSlackNotification>) -> Result<usize, Error>;
    fn find_not_send(&self, slack_url: &str) -> Result<Vec<Paper>, Error>;
    fn mark_as_send(&self, slack_url: &str, id: &PaperId) -> Result<usize, Error>;
    fn delete(&self, slack_url: &str, paper_id: &PaperId) -> Result<(), Error>;
}

#[derive(Clone)]
pub struct SlackNotificationUseCase<S>
    where S: SlackNotificationRepositoryTrait
{
    pub slack_notification_repository: S,
}

impl<S> SlackNotificationUseCase<S>
    where S: SlackNotificationRepositoryTrait
{
    pub fn new(slack_notification_repository: S) -> Self {
        Self {
            slack_notification_repository
        }
    }
}

impl<S> SlackNotificationUseCaseTrait for SlackNotificationUseCase<S>
    where S: SlackNotificationRepositoryTrait
{
    fn enqueue_slack_notifications(&self, notifications: &Vec<NewSlackNotification>) -> Result<usize, Error> {
        self.slack_notification_repository.save(notifications)
    }
    fn find_not_send(&self, slack_url: &str) -> Result<Vec<Paper>, Error> {
        self.slack_notification_repository.find_not_send(slack_url)

    }
    fn mark_as_send(&self, slack_url: &str, id: &PaperId) -> Result<usize, Error> {
        self.slack_notification_repository.mark_as_send(slack_url, id)
    }
    fn delete(&self, slack_url: &str, paper_id: &PaperId) -> Result<(), Error> {
        self.slack_notification_repository.delete(slack_url, paper_id)
    }
}
