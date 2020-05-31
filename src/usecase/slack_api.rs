use crate::domain::arxiv_paper::Paper;
use crate::domain::slack_api::SlackAPITrait;

pub trait SlackAPIUseCaseTrait {
    fn send(&self, message: &str);
    fn build_messages(&self, papers: &Vec<Paper>, keywords: &Option<Vec<String>>) -> Vec<(Paper, String)>;
}


#[derive(Clone)]
pub struct SlackAPIUseCase<A>
    where A: SlackAPITrait
{
    pub slack_api: A,
}

impl<A> SlackAPIUseCase<A>
    where A: SlackAPITrait
{
    pub fn new(slack_api: A) -> Self {
        Self {
            slack_api
        }
    }
}

impl<A> SlackAPIUseCaseTrait for SlackAPIUseCase<A>
    where A: SlackAPITrait
{
    fn send(&self, message: &str) {
        self.slack_api.send(message);
    }
    fn build_messages(&self, papers: &Vec<Paper>, keywords: &Option<Vec<String>>) -> Vec<(Paper, String)> {
        self.slack_api.build_messages(papers, keywords)
    }
}
