pub mod dummy;
pub mod wallabag;

use chrono::FixedOffset;
use fxhash::FxHashSet;
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::{self, File},
    io::Error,
    os::unix::fs::MetadataExt,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use crate::{
    articles::wallabag::Wallabag,
    metadata::{FileInfo, Info},
    settings::{self, ArticleList},
    view::Hub,
};

pub const ARTICLES_DIR: &str = ".articles";

#[derive(Serialize, Deserialize)]
pub struct ArticleIndex {
    pub articles: BTreeMap<String, Article>,
}

impl Default for ArticleIndex {
    fn default() -> Self {
        ArticleIndex {
            articles: BTreeMap::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum Changes {
    Deleted,
    Starred,
    Archived,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Article {
    pub id: String,
    #[serde(skip_serializing_if = "FxHashSet::is_empty")]
    #[serde(default)]
    pub changed: FxHashSet<Changes>,
    pub title: String,
    pub domain: String,
    pub authors: Vec<String>,
    pub format: String,
    pub language: String,
    pub reading_time: u32,
    pub added: chrono::DateTime<FixedOffset>,
    pub starred: bool,
    pub archived: bool,
}

impl Article {
    fn path(&self) -> PathBuf {
        std::path::absolute(PathBuf::from(format!(
            "{}/article-{}.{}",
            ARTICLES_DIR, self.id, self.format
        )))
        .unwrap()
    }

    pub fn file(&self) -> FileInfo {
        let path = self.path();
        let size = match fs::metadata(&path) {
            Ok(metadata) => metadata.size(),
            Err(_err) => 0,
        };
        FileInfo {
            path: path,
            kind: self.format.to_owned(),
            size: size,
        }
    }

    pub fn info(&self) -> Info {
        Info {
            title: self.title.to_owned(),
            subtitle: self.domain.to_owned(),
            author: self.authors.join(", "),
            year: "".to_string(),
            language: self.language.to_owned(),
            publisher: "".to_string(),
            series: "".to_string(),
            edition: "".to_string(),
            volume: "".to_string(),
            number: "".to_string(),
            identifier: "".to_string(),
            categories: BTreeSet::new(),
            file: self.file(),
            reader: None,
            reader_info: None,
            toc: None,
            added: self.added.naive_local(),
        }
    }
}

pub fn read_index() -> Result<ArticleIndex, Error> {
    let file = File::open(ARTICLES_DIR.to_owned() + "/index.json")?;
    let index: ArticleIndex = serde_json::from_reader(file)?;

    Ok(index)
}

pub trait Service {
    fn filter(&self, list: ArticleList) -> Vec<Article>;

    fn index(&self) -> Arc<Mutex<ArticleIndex>>;

    fn save_index(&self);

    // Update the list of articles.
    // Returns true when the update was started, false when an update is already
    // in progress.
    fn update(&mut self, hub: &Hub) -> bool;
}

pub fn load(auth: settings::ArticleAuth) -> Box<dyn Service> {
    match auth.api.as_str() {
        "wallabag" => Box::new(Wallabag::load(auth)),
        _ => Box::new(dummy::Dummy::new()),
    }
}
