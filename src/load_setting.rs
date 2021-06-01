use toml;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;


#[derive(Deserialize, Debug)]
pub struct Config {
    pub arxiv: Vec<ArxivConfig>,
}

#[derive(Deserialize, Debug)]
pub struct ArxivConfig {
    pub categories: Vec<String>,
    pub search_title_words: Option<Vec<String>>,
    pub exclude_title_words: Option<Vec<String>>,
    pub search_abstract_words: Option<Vec<String>>,
    pub exclude_abstract_words: Option<Vec<String>>,
    pub filter_by_main_category: bool,
    pub slack: String,
    pub star_keywords: Option<Vec<String>>,
}

pub fn load_config(path: &str) -> anyhow::Result<Config> {
    let mut config_toml = String::new();

    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(_)  => {
            panic!("Could not find config file, using default!");
        }
    };

    file.read_to_string(&mut config_toml)
        .unwrap_or_else(|err| panic!("Error while reading config: [{}]", err));

    Ok(toml::from_str(&config_toml)?)
}

#[test]
fn test_load_config() {
    let config = load_config("setting.toml").unwrap();
    println!("{:?}", &config);
    assert_eq!(config.arxiv.len(), 4usize);
}