use failure::Error;
use chrono::NaiveDateTime;

#[derive(Debug, Clone)]
pub struct Paper {
    pub title: String,
    pub url: String,
    pub pdf_url: String,
    pub authors: Vec<String>,
    pub category: String,
    pub summary: String,
    pub comment: String,
    pub is_accepted: bool,
    pub updated: NaiveDateTime,
    pub published: NaiveDateTime,
}

pub trait ArxivAPITrait {
    fn query(&self) -> Result<Vec<Paper>, Error>;
}
