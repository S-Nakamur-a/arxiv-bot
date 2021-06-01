use crate::domain::arxiv_api::{Paper as ApiPaper, ArxivAPITrait};

pub trait ArxivAPIUseCaseTrait {
    fn query(&self) -> anyhow::Result<Vec<ApiPaper>>;
}

#[derive(Clone)]
pub struct ArxivAPIUseCase<A>
    where A: ArxivAPITrait
{
    pub arxiv_api: A,
}

impl<A> ArxivAPIUseCase<A>
    where A: ArxivAPITrait
{
    pub fn new(arxiv_api: A) -> Self {
        Self {
            arxiv_api
        }
    }
}

impl<A> ArxivAPIUseCaseTrait for ArxivAPIUseCase<A>
    where A: ArxivAPITrait
{
    fn query(&self) -> anyhow::Result<Vec<ApiPaper>> {
        let new_papers = self.arxiv_api.query()?;
        Ok(new_papers)
    }
}
