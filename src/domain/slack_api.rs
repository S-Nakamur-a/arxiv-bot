use crate::domain::arxiv_paper::Paper;


pub trait SlackAPITrait {
    fn send(&self, message: &str);
    fn build_messages(&self, papers: &Vec<Paper>, keywords: &Option<Vec<String>>) -> Vec<(Paper, String)>;
}
