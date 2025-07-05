use std::sync::{Arc, Mutex};

use crate::{
    articles::{Article, ArticleIndex, Service},
    view::Hub,
};

pub struct Dummy {
    index: Arc<Mutex<ArticleIndex>>,
}

impl Dummy {
    pub fn new() -> Dummy {
        Dummy {
            index: Arc::new(Mutex::new(ArticleIndex::default())),
        }
    }
}

impl Service for Dummy {
    fn filter(&self, _list: crate::settings::ArticleList) -> Vec<Article> {
        Vec::new()
    }
    fn index(&self) -> Arc<Mutex<ArticleIndex>> {
        self.index.clone()
    }
    fn save_index(&self) {}
    fn update(&mut self, _hub: &Hub) -> bool {
        // nothing to do, always finishes immediately
        true
    }
}
