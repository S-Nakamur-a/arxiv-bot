pub mod block;

use reqwest;
use crate::domain::slack_api::SlackAPITrait;
use crate::domain::arxiv_paper::Paper;
use super::reqwest::header::{CONTENT_TYPE, HeaderValue};

pub struct SlackAPI {
    url: String
}

impl SlackAPI {
    pub fn new(url: &str) -> Self {
        SlackAPI {
            url: url.to_string()
        }
    }
}

impl SlackAPITrait for SlackAPI {
    fn send(&self, message: &str) {
        let client = reqwest::blocking::Client::new();
        let _ = client
            .post(&self.url)
            .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
            .body(message.to_string())
            .send()
            .unwrap();
    }

    fn build_messages(&self, papers: &Vec<Paper>, keywords: &Option<Vec<String>>) -> Vec<(Paper, String)> {
        let mut blocks: Vec<(usize, Paper, String)> = papers.iter().map(|p| {
            let mut section = block::Section::new();
            let n_stars = get_stars(&p, &keywords);
            let text = get_text(&p, &keywords);
            section.text(&text).block_id(&p.url);

            let mut category_button = block::Button::new();
            category_button.text(&p.category.name)
                .emoji(true)
                .action_id(&p.url)
                .url(&format!("https://arxiv.org/list/{}/recent", &p.category.name));
            section.button(&category_button);

            let mut pdf_button = block::Button::new();
            let mut web_button = block::Button::new();
            pdf_button.text(":pdf: Open PDF").url(&p.pdf_url).action_id(&format!("b_{}", &p.pdf_url)).primary();
            web_button.text(":link: Open Web").url(&p.url).action_id(&format!("b_{}", &p.url)).primary();

            let mut actions = block::Actions::new();
            actions.button(&web_button).button(&pdf_button);

            (n_stars, p.to_owned(), json!({"blocks": [block::Block::Section(section), block::Block::Actions(actions)]}).to_string())
        }).collect();
        blocks.sort_by(|a, b| a.0.cmp(&b.0).reverse());
        let blocks: Vec<(Paper, String)> = blocks.iter()
            .map(|b| (b.1.clone(), b.2.clone())).collect();

        blocks
    }
}

fn get_text(paper: &Paper, keywords: &Option<Vec<String>>) -> String {
    let n_stars = get_stars(&paper, &keywords);
    let stars = ":star:".to_string().repeat(n_stars);

    let is_new = paper.published.eq(&paper.updated);
    let kazari = if is_new { ":new2:" } else { ":updated:" };

    let comments = format!("> {}\n", &paper.comment);

    let time = format!("公開 {} 変更 {}\n",
        &paper.published.format("%Y/%m/%d %H:%M").to_string(),
        &paper.updated.format("%Y/%m/%d %H:%M").to_string(),
    );

    const SLACK_MESSAGE_TRIM: usize = 200;
    let mut summary = paper.summary.to_owned();
    if summary.len() >= SLACK_MESSAGE_TRIM {
        summary = format!("{}...", &summary[0..SLACK_MESSAGE_TRIM - 1]);
    }
    let text = format!("{}{} *{}*\n{}{}{}", &kazari, stars, &paper.title, &time, comments, summary);
    text
}

fn get_stars(paper: &Paper, keywords: &Option<Vec<String>>) -> usize {
    match keywords {
        Some(k) => {
            let mut count: usize = 0;
            for keyword in k {
                if paper.comment.contains(keyword) {
                    count += 1
                }
            }
            count
        }
        None => 0usize
    }
}
