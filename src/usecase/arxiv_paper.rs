use crate::domain::arxiv_paper::{Paper, PaperId, NewPaper, ArxivPaperRepositoryTrait};

pub trait ArxivPaperUseCaseTrait {
    fn find_by_id(&self, id: PaperId) -> anyhow::Result<Option<Paper>>;
    fn find_by_urls(&self, urls: &Vec<String>) -> anyhow::Result<Vec<Paper>>;
    fn save(&self, papers: &Vec<NewPaper>) -> anyhow::Result<usize>;
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
    fn find_by_id(&self, id: PaperId) -> anyhow::Result<Option<Paper>> {
        self.arxiv_paper_repository.find_by_id(id)
    }
    fn find_by_urls(&self, urls: &Vec<String>) -> anyhow::Result<Vec<Paper>> {
        self.arxiv_paper_repository.find_by_urls(urls)
    }
    fn save(&self, papers: &Vec<NewPaper>) -> anyhow::Result<usize> {
        self.arxiv_paper_repository.save(papers)
    }
}


pub fn extract_paper_urls(papers: &Vec<NewPaper>) -> Vec<String> {
    papers.iter().map(|p| p.url.clone()).collect()
}