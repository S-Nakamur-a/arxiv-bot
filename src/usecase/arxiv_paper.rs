use failure::Error;

use crate::domain::arxiv_paper::{Paper, PaperId, NewPaper, ArxivPaperRepositoryTrait};

pub trait ArxivPaperUseCaseTrait {
    fn find_by_id(&self, id: PaperId) -> Result<Option<Paper>, Error>;
    fn find_by_urls(&self, urls: &Vec<String>) -> Result<Vec<Paper>, Error>;
    fn save(&self, papers: &Vec<NewPaper>) -> Result<usize, Error>;
}

#[derive(Clone)]
pub struct ArxivPaperUseCase<A>
    where A: ArxivPaperRepositoryTrait
{
    pub arxiv_paper_repository: A,
}

impl<A> ArxivPaperUseCase<A>
    where A: ArxivPaperRepositoryTrait
{
    pub fn new(arxiv_paper_repository: A) -> Self {
        Self {
            arxiv_paper_repository
        }
    }
}

impl<A> ArxivPaperUseCaseTrait for ArxivPaperUseCase<A>
    where A: ArxivPaperRepositoryTrait
{
    fn find_by_id(&self, id: PaperId) -> Result<Option<Paper>, Error> {
        self.arxiv_paper_repository.find_by_id(id)
    }
    fn find_by_urls(&self, urls: &Vec<String>) -> Result<Vec<Paper>, Error> {
        self.arxiv_paper_repository.find_by_urls(urls)
    }
    fn save(&self, papers: &Vec<NewPaper>) -> Result<usize, Error> {
        self.arxiv_paper_repository.save(papers)
    }
}


pub fn extract_paper_urls(papers: &Vec<NewPaper>) -> Vec<String> {
    papers.iter().map(|p| p.url.clone()).collect()
}