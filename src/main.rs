#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate percent_encoding;
extern crate quick_xml;
extern crate regex;
extern crate structopt;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_json;
use std::{thread, time};

use crate::usecase::arxiv_paper::ArxivPaperUseCaseTrait;

pub mod db;
pub mod domain;
pub mod infrastructure;
pub mod load_setting;
pub mod usecase;

use crate::domain::arxiv_paper::NewPaper;
use crate::domain::slack_notification::NewSlackNotification;
use crate::usecase::arxiv_api::ArxivAPIUseCaseTrait;
use crate::usecase::slack_api::SlackAPIUseCaseTrait;
use crate::usecase::slack_notifications::SlackNotificationUseCaseTrait;
use infrastructure::arxiv_api::{ArxivAPI, SortBy, SortOrder};
use infrastructure::arxiv_paper::ArxivPaperRepository;
use infrastructure::slack_api::SlackAPI;
use infrastructure::slack_notifications::SlackNotificationRepository;
use load_setting::load_config;
use structopt::clap;
use structopt::clap::arg_enum;
use structopt::StructOpt;
use usecase::arxiv_api::ArxivAPIUseCase;
use usecase::arxiv_paper::{extract_paper_urls, ArxivPaperUseCase};
use usecase::slack_api::SlackAPIUseCase;
use usecase::slack_notifications::SlackNotificationUseCase;

#[derive(StructOpt, Debug)]
#[structopt(name = "arXiv API")]
#[structopt(setting(clap::AppSettings::ColoredHelp))]
pub struct Opt {
    #[structopt(short, long)]
    max_results: Option<u32>,

    #[structopt(short, long)]
    start: u32,

    #[structopt(long = "slack")]
    slack: bool,

    #[structopt(long = "send")]
    send: bool,

    #[structopt(long = "save")]
    save: bool,

    #[structopt(possible_values = & OptSortBy::variants(), case_insensitive = false)]
    sort_by: OptSortBy,
}

arg_enum! {
    #[derive(Debug)]
    enum OptSortBy {
        Relevance,
        LastUpdatedBy,
        SubmittedDate,
    }
}

fn main() {
    dotenv::dotenv().ok();
    let config = load_config("setting.toml").unwrap();

    let opt: Opt = Opt::from_args();

    for c in &config.arxiv {
        let arxiv_api_interface = ArxivAPI::new(
            &c.categories,
            &c.search_title_words,
            &c.exclude_title_words,
            &c.search_abstract_words,
            &c.exclude_abstract_words,
            match &opt.sort_by {
                OptSortBy::Relevance => SortBy::Relevance,
                OptSortBy::LastUpdatedBy => SortBy::LastUpdatedDate,
                OptSortBy::SubmittedDate => SortBy::SubmittedDate,
            },
            SortOrder::Descending,
            opt.max_results.unwrap_or(100),
            Some(opt.start as u32),
            c.filter_by_main_category,
        );

        let arxiv_api = ArxivAPIUseCase::new(arxiv_api_interface);

        let papers = arxiv_api.query().unwrap();

        match papers.len() {
            0 => println!("No paper found"),
            1 => println!("1 paper found"),
            _ => println!("{} papers found", papers.len()),
        }

        if opt.save {
            let papers = papers.iter().map(|p| NewPaper::from(p.clone())).collect();

            let arxiv_paper_repository = ArxivPaperRepository::new();
            let slack_notification_repository = SlackNotificationRepository::new();
            let arxiv_paper = ArxivPaperUseCase::new(arxiv_paper_repository);
            let slack_notification = SlackNotificationUseCase::new(slack_notification_repository);

            let n_papers = arxiv_paper.save(&papers).unwrap();
            match n_papers {
                0 => println!("No paper saved"),
                1 => println!("1 paper saved"),
                _ => println!("{} papers saved", &n_papers),
            }

            let paper_urls = extract_paper_urls(&papers);
            let papers = arxiv_paper.find_by_urls(&paper_urls).unwrap();

            if !papers.is_empty() {
                let notifications = papers
                    .iter()
                    .map(|p| NewSlackNotification {
                        paper_id: p.id.clone(),
                        slack_url: c.slack.clone(),
                        updated: p.updated,
                    })
                    .collect::<Vec<NewSlackNotification>>();
                slack_notification
                    .enqueue_slack_notifications(&notifications)
                    .unwrap();
            }
        }

        if opt.slack {
            let slack_notification_repository = SlackNotificationRepository::new();
            let slack_notification = SlackNotificationUseCase::new(slack_notification_repository);

            let papers = slack_notification.find_not_send(&c.slack).unwrap();
            if papers.is_empty() {
                println!("All papers have been sent to slack");
            } else {
                let slack_api_interface = SlackAPI::new(&c.slack);
                let slack_api = SlackAPIUseCase::new(slack_api_interface);

                //一つずつsendしたほうが伝わる
                let messages = slack_api.build_messages(&papers, &c.star_keywords);
                if opt.send {
                    for m in &messages {
                        slack_api.send(&m.1);
                        slack_notification.mark_as_send(&c.slack, &m.0.id).unwrap();
                    }
                } else {
                    for m in &messages {
                        println!("{:?}", &m.1);
                    }
                    println!("{} messages to send", messages.len());
                }
            }
        }
        thread::sleep(time::Duration::from_millis(3000))  // for arxiv api limit
    }
}
