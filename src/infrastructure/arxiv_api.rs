use crate::domain::arxiv_api as I;
use chrono::NaiveDateTime;
use failure::Error;
use percent_encoding::{utf8_percent_encode, AsciiSet, NON_ALPHANUMERIC};
use quick_xml;
use quick_xml::de::from_str;
use regex::Regex;
use reqwest;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Author {
    name: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Link {
    title: Option<String>,
    href: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct PrimaryCategory {
    term: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Arxiv {
    primary_category: PrimaryCategory,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Entry {
    id: String,
    updated: String,
    published: String,
    title: String,
    summary: String,
    author: Vec<Author>,
    comment: Option<String>,
    link: Vec<Link>,
    primary_category: PrimaryCategory,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Feed {
    entry: Vec<Entry>,
}

#[derive(Debug, Clone)]
pub enum SortBy {
    Relevance,
    LastUpdatedDate,
    SubmittedDate,
}

#[derive(Debug, Clone)]
pub enum SortOrder {
    Ascending,
    Descending,
}

#[derive(Debug, Clone)]
pub struct ArxivAPI {
    search_categories: Vec<String>,
    search_title_words: Option<Vec<String>>,
    exclude_title_words: Option<Vec<String>>,
    search_abstract_words: Option<Vec<String>>,
    exclude_abstract_words: Option<Vec<String>>,
    sort_by: SortBy,
    sort_order: SortOrder,
    max_result: u32,
    start: Option<u32>,
    filter_by_main_category: bool,
}

impl ArxivAPI {
    pub fn new(
        search_categories: &Vec<String>,
        search_title_words: &Option<Vec<String>>,
        exclude_title_words: &Option<Vec<String>>,
        search_abstract_words: &Option<Vec<String>>,
        exclude_abstract_words: &Option<Vec<String>>,
        sort_by: SortBy,
        sort_order: SortOrder,
        max_result: u32,
        start: Option<u32>,
        filter_by_main_category: bool,
    ) -> Self {
        Self {
            search_categories: search_categories.clone(),
            search_title_words: search_title_words.clone(),
            exclude_title_words: exclude_title_words.clone(),
            search_abstract_words: search_abstract_words.clone(),
            exclude_abstract_words: exclude_abstract_words.clone(),
            sort_by,
            sort_order,
            max_result,
            start,
            filter_by_main_category,
        }
    }

    fn generate_search_queries(&self) -> String {
        let q = self
            .search_categories
            .iter()
            .map(|s| format!("cat:{}", s))
            .collect::<Vec<String>>()
            .join("+OR+");
        let q = format!("({})", &q);
        let q = match &self.search_title_words {
            Some(ws) => format!(
                "{}+AND+({})",
                &q,
                &ws.iter()
                    .map(|s| format!("ti:\"{}\"", s))
                    .collect::<Vec<String>>()
                    .join("+OR+"),
            ),
            None => q,
        };
        let q = match &self.exclude_title_words {
            Some(ws) => format!(
                "{}+ANDNOT+({})",
                &q,
                &ws.iter()
                    .map(|s| format!("ti:\"{}\"", s))
                    .collect::<Vec<String>>()
                    .join("+OR+"),
            ),
            None => q,
        };
        let q = match &self.search_abstract_words {
            Some(ws) => format!(
                "{}+AND+({})",
                &q,
                &ws.iter()
                    .map(|s| format!("abs:\"{}\"", s))
                    .collect::<Vec<String>>()
                    .join("+OR+"),
            ),
            None => q,
        };
        let q = match &self.exclude_abstract_words {
            Some(ws) => format!(
                "{}+ANDNOT+({})",
                &q,
                &ws.iter()
                    .map(|s| format!("abs:\"{}\"", s))
                    .collect::<Vec<String>>()
                    .join("+OR+"),
            ),
            None => q,
        };
        q
    }

    fn generate_api_url(&self) -> String {
        let search_query = format!("&search_query={}", self.generate_search_queries());
        let sort_by = format!(
            "&sortBy={}",
            match self.sort_by {
                SortBy::LastUpdatedDate => "lastUpdatedDate",
                SortBy::Relevance => "relevance",
                SortBy::SubmittedDate => "submittedDate",
            }
        );
        let sort_order = format!(
            "&sortOrder={}",
            match self.sort_order {
                SortOrder::Ascending => "ascending",
                SortOrder::Descending => "descending",
            }
        );
        let max_result = format!("&max_results={}", self.max_result);
        let start = format!(
            "&start={}",
            match self.start {
                Some(n) => n,
                None => 0,
            }
        );
        let url = "https://export.arxiv.org/api/query";
        let params = &format!(
            "?{}{}{}{}{}",
            search_query, sort_by, sort_order, max_result, start
        );
        const FRAGMENT: &AsciiSet = &NON_ALPHANUMERIC
            .remove(b'?')
            .remove(b'&')
            .remove(b'=')
            .remove(b':')
            .remove(b'+')
            .remove(b'.')
            .remove(b'-')
            .remove(b'_');
        let params = utf8_percent_encode(params, FRAGMENT).to_string();
        return url.to_owned() + &params;
    }

    fn fetch_xml(&self) -> reqwest::Result<String> {
        let api_url = self.generate_api_url();
        println!("{}", api_url);
        let body = reqwest::blocking::get(&api_url)?.text();
        body
    }

    fn convert(&self, feed: &Feed) -> Vec<I::Paper> {
        let mut papers: Vec<I::Paper> = Vec::new();
        for entry in &feed.entry {
            let comment = entry.comment.to_owned().unwrap_or("".to_string());
            let l_comment = comment.to_lowercase();
            let is_accepted = l_comment.contains("accept") || l_comment.contains("appear");
            papers.push(I::Paper {
                title: entry.title.replace("\n", " "),
                updated: NaiveDateTime::parse_from_str(&entry.updated, "%Y-%m-%dT%H:%M:%SZ")
                    .unwrap(),
                published: NaiveDateTime::parse_from_str(&entry.published, "%Y-%m-%dT%H:%M:%SZ")
                    .unwrap(),
                url: entry.id.replace("\n", ""),
                pdf_url: entry
                    .link
                    .iter()
                    .filter_map(|x| {
                        if x.title == Some("pdf".to_string()) {
                            Some(x.href.replace("\n", ""))
                        } else {
                            None
                        }
                    })
                    .next()
                    .unwrap_or(entry.id.replace("abs", "pdf")),
                authors: entry
                    .author
                    .iter()
                    .map(|x| x.name.replace("\n", " "))
                    .collect(),
                category: entry.primary_category.term.replace("\n", ""),
                summary: entry.summary.replace("\n", " "),
                comment: comment.replace("\n", " "),
                is_accepted,
            })
        }
        papers
    }

    fn to_papers(&self, xml: String) -> Result<Vec<I::Paper>, Error> {
        let re = Regex::new("<link title=\"doi\".*>\n")?;
        let xml = re.replace_all(&xml, "");
        let feed: Feed = from_str(&xml)?;
        Ok(self.convert(&feed))
    }
}

impl I::ArxivAPITrait for ArxivAPI {
    fn query(&self) -> Result<Vec<I::Paper>, Error> {
        let xml = self.fetch_xml()?;
        let mut papers = self.to_papers(xml)?;
        if self.filter_by_main_category {
            papers = papers
                .iter()
                .filter(|&p| self.search_categories.contains(&p.category))
                .cloned()
                .collect::<Vec<I::Paper>>();
        }
        Ok(papers)
    }
}
