#![allow(unused)]

use diesel::prelude::*;
use chrono::NaiveDateTime;
use std::path::Iter;
use std::collections::{HashSet, HashMap};
use std::iter::FromIterator;

use super::sqlite::SQLite;
use crate::db::models::{PaperAuthor, Paper, Category, Author};
use crate::db::schema::*;
use crate::domain::arxiv_paper as I;
use crate::domain::arxiv_paper::PaperId;

#[derive(Insertable, Debug)]
#[table_name = "papers"]
struct NewPaper {
    pub title: String,
    pub updated: NaiveDateTime,
    pub published: NaiveDateTime,
    pub url: String,
    pub pdf_url: String,
    pub category_id: i32,
    pub summary: String,
    pub comment: String,
    pub accepted: i32,
}

#[derive(Insertable, Debug)]
#[table_name = "authors"]
struct NewAuthor {
    pub name: String,
}

#[derive(Insertable, Debug)]
#[table_name = "categories"]
struct NewCategory {
    pub name: String,
}

#[derive(Insertable, Debug)]
#[table_name = "paper_authors"]
struct NewPaperAuthor {
    pub paper_id: i32,
    pub author_id: i32,
}


#[derive(Clone)]
pub struct ArxivPaperRepository {}

impl ArxivPaperRepository {
    pub fn new() -> Self {
        Self {}
    }
}

impl I::ArxivPaperRepositoryTrait for ArxivPaperRepository {
    fn find_by_id(&self, id: I::PaperId) -> anyhow::Result<Option<I::Paper>> {
        let conn = SQLite::create().connect();
        let paper = conn.transaction::<Option<I::Paper>, _, _>(|| {
            // joinすれば一発だが、[(paper1, author1), (paper1, author2), (paper1, author3)]となり
            // 後処理が複雑化するのと、関数化しづらくなるため敢えて分割している
            let paper_with_category: (Paper, Category) = papers::table
                .find(id.0)
                .inner_join(categories::table)
                .select((papers::all_columns, categories::all_columns))
                .first(&conn)?;
            let authors: Vec<Author> = paper_authors::table
                .filter(paper_authors::paper_id.eq(id.0))
                .inner_join(authors::table)
                .select(authors::all_columns)
                .load(&conn)?;
            Ok(Some(I::Paper {
                id: I::PaperId(paper_with_category.0.id),
                title: paper_with_category.0.title,
                updated: paper_with_category.0.updated,
                published: paper_with_category.0.published,
                url: paper_with_category.0.url,
                pdf_url: paper_with_category.0.pdf_url,
                authors: authors.iter().map(
                    |a| I::Author { id: a.id, name: a.name.to_owned() }).collect(),
                category: I::Category {
                    id: paper_with_category.1.id,
                    name: paper_with_category.1.name,
                },
                comment: paper_with_category.0.comment,
                is_accepted: paper_with_category.0.accepted > 0,
                summary: paper_with_category.0.summary,
            }))
        });
        paper
    }
    fn find_by_urls(&self, urls: &Vec<String>) -> anyhow::Result<Vec<I::Paper>> {
        let conn = SQLite::create().connect();
        let load_papers: Vec<(Paper, Category)> = papers::table
            .inner_join(categories::table)
            .filter(papers::url.eq_any(urls))
            .select((papers::all_columns, categories::all_columns))
            .load(&conn)?;
        Ok(load_papers.iter().map(|p| I::Paper{
            id: PaperId(p.0.id),
            title: p.0.title.to_string(),
            url: p.0.url.to_string(),
            pdf_url: p.0.pdf_url.to_string(),
            authors: vec![],
            category: I::Category {
                id: p.1.id,
                name: p.1.name.to_string(),
            },
            summary: p.0.summary.to_string(),
            comment: p.0.comment.to_string(),
            is_accepted: p.0.accepted > 0,
            updated: p.0.updated,
            published: p.0.published,
        }).collect())
    }

    fn save(&self, papers: &Vec<I::NewPaper>) -> anyhow::Result<usize> {
        let conn = SQLite::create().connect();
        let res = conn.transaction::<_, _, _>(|| {
            // Insert authors
            let new_authors = new_authors(&papers);
            // 未知のauthorのみinsert
            diesel::insert_or_ignore_into(authors::table)
                .values(&new_authors)
                .execute(&conn)?;
            let authors_map: HashMap<String, i32> = authors::table
                .filter(authors::name.eq_any(
                    new_authors.iter()
                        .map(|a| a.name.clone())
                        .collect::<Vec<String>>()))
                .select((authors::name, authors::id))
                .load(&conn)?
                .into_iter().collect();
            // Insert categories
            let new_categories = new_categories(&papers);
            diesel::insert_or_ignore_into(categories::table)
                .values(&new_categories)
                .execute(&conn)?;
            let categories_map: HashMap<String, i32> = categories::table
                .filter(categories::name.eq_any(
                    new_categories.iter()
                        .map(|c| c.name.clone())
                        .collect::<Vec<String>>()))
                .select((categories::name, categories::id))
                .load(&conn)?
                .into_iter().collect();

            // Insert papers
            let new_papers = new_papers(&papers, &categories_map);
            let n_inserted = diesel::insert_or_ignore_into(papers::table)
                .values(&new_papers)
                .execute(&conn)?;
            let papers_map: HashMap<String, i32> = papers::table
                .filter(papers::url.eq_any(
                    new_papers.iter()
                        .map(|p| p.url.clone())
                        .collect::<Vec<String>>()
                ))
                .select((papers::url, papers::id))
                .load(&conn)?
                .into_iter().collect();

            // Insert paper_authors
            let new_paper_authors = new_paper_authors(&papers, &papers_map, &authors_map);
            diesel::insert_or_ignore_into(paper_authors::table)
                .values(&new_paper_authors)
                .execute(&conn);
            Ok(n_inserted)
        });
        res
    }

    fn download(&self, papers: &Vec<I::Paper>) -> bool {
        unimplemented!()
    }
}

fn new_authors(papers: &Vec<I::NewPaper>) -> Vec<NewAuthor> {
    let mut new_authors: Vec<&str> = papers
        .iter()
        .map(|p| p.authors.iter().map(|a| a.name.as_ref()))
        .flatten()
        .collect();
    new_authors.sort_unstable();
    new_authors.dedup();
    new_authors.iter().map(|a| NewAuthor { name: a.to_string() }).collect()
}

fn new_categories(papers: &Vec<I::NewPaper>) -> Vec<NewCategory> {
    let mut new_categories: Vec<&str> = papers
        .iter()
        .map(|p| p.category.name.as_ref())
        .collect();
    new_categories.sort_unstable();
    new_categories.dedup();
    new_categories.iter().map(|x| NewCategory { name: x.to_string() }).collect()
}

fn new_papers(papers: &Vec<I::NewPaper>, category_map: &HashMap<String, i32>) -> Vec<NewPaper> {
    papers.iter().map(|p|
        NewPaper {
            title: p.title.to_owned(),
            updated: p.updated,
            published: p.published,
            url: p.url.to_owned(),
            pdf_url: p.pdf_url.to_owned(),
            category_id: category_map.get(&p.category.name).unwrap().to_owned(),
            summary: p.summary.to_owned(),
            comment: p.comment.to_owned(),
            accepted: p.is_accepted as i32,
        }).collect()
}

fn new_paper_authors(papers: &Vec<I::NewPaper>,
                     papers_map: &HashMap<String, i32>,
                     authors_map: &HashMap<String, i32>,
) -> Vec<NewPaperAuthor> {
    papers.iter().map(|p| p.authors.iter().map(
        move |a|
            NewPaperAuthor {
                paper_id: papers_map.get(&p.url).unwrap().to_owned(),
                author_id: authors_map.get(&a.name).unwrap().to_owned(),
            }
    )).flatten().collect()
}