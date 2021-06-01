use super::arxiv_api as API;
use std::convert::From;
use chrono::NaiveDateTime;

#[derive(Debug, Clone)]
pub struct Author {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Category {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct PaperId(pub i32);

#[derive(Debug, Clone)]
pub struct Paper {
    pub id: PaperId,
    pub title: String,
    pub url: String,
    pub pdf_url: String,
    pub authors: Vec<Author>,
    pub category: Category,
    pub summary: String,
    pub comment: String,
    pub is_accepted: bool,
    pub updated: NaiveDateTime,
    pub published: NaiveDateTime,
}

#[derive(Debug, Clone)]
pub struct NewAuthor {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct NewCategory {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct NewPaper {
    pub title: String,
    pub url: String,
    pub pdf_url: String,
    pub category: NewCategory,
    pub authors: Vec<NewAuthor>,
    pub summary: String,
    pub updated: NaiveDateTime,
    pub published: NaiveDateTime,
    pub comment: String,
    pub is_accepted: bool,
}

impl From<API::Paper> for NewPaper {
    fn from(paper: API::Paper) -> Self {
        Self {
            title: paper.title,
            updated: paper.updated,
            published: paper.published,
            url: paper.url,
            pdf_url: paper.pdf_url,
            category: NewCategory { name: paper.category },
            authors: paper.authors.iter()
                .map(|a| NewAuthor { name: a.to_owned() })
                .collect(),
            summary: paper.summary,
            comment: paper.comment,
            is_accepted: paper.is_accepted,
        }
    }
}

pub trait ArxivPaperRepositoryTrait {
    fn find_by_id(&self, id: PaperId) -> anyhow::Result<Option<Paper>>;
    fn find_by_urls(&self, urls: &Vec<String>) -> anyhow::Result<Vec<Paper>>;
    fn save(&self, papers: &Vec<NewPaper>) -> anyhow::Result<usize>;
    fn download(&self, papers: &Vec<Paper>) -> bool;
}
