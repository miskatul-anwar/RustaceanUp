use std::io::{stdin, BufRead};

fn _rin() -> String {
    let mut input = String::new();
    stdin().lock().read_line(&mut input).unwrap();
    input
}

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl NewsArticle {
    fn new() -> Self {
        NewsArticle {
            headline: String::new(),
            location: String::new(),
            author: String::new(),
            content: String::new(),
        }
    }

    fn update(&mut self, headline: &str, location: &str, author: &str, content: &str) {
        self.headline = headline.to_string();
        self.location = location.to_string();
        self.author = author.to_string();
        self.content = content.to_string();
    }
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

fn main() {
    let mut nw = NewsArticle::new();
    let headline: &str = &_rin();
    let location: &str = &_rin();
    let author: &str = &_rin();
    let content: &str = &_rin();

    nw.update(headline, location, author, content);
    println!("{}", nw.summarize());
}
