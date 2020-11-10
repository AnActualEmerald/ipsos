use reqwest;
use rustyline;
use serde::Deserialize;
use term;
use std::io::prelude::*;
use futures::future::{BoxFuture, FutureExt};

use crate::manager::Show;

//If you cloned this repo, create this file in the src directory
const SECRET: &'static str = include_str!("secret"); 

pub fn get_show_data(title: &str) -> BoxFuture<'_, Option<Show>>{
    let mut t = term::stdout().unwrap();
    async move{
        match search_show(title).await{
            Ok(res) => {
                let mut rl = rustyline::Editor::<()>::new();
                t.fg(term::color::WHITE).unwrap();
                write!(t, "Searched on IMDb and got: ");
                t.fg(term::color::CYAN).unwrap();
                write!(t, "{}\n", res.title);
                t.fg(term::color::WHITE).unwrap();
                write!(t, "Add this show to your watchlist? (Y/n/[r]etry) ");
                let input = rl.readline("").unwrap_or("y".to_owned());
                match input.to_lowercase().chars().next() {
                    Some('n') => {
                        writeln!(t, "Alright, this show won't be added to your watchlist");
                        None
                    }
                    Some('r') => {
                        write!(t, "Okay, what title should I search for: ");
                        let re_title = rl.readline("").unwrap();
                        get_show_data(&re_title).await
                    }
                    _ => { //the default is yes, so everything goes here
                        Some(Show {
                            title: res.title,
                            length: 0,
                            completed: false,
                            watched: 0,
                        })
                    }
                    
                }
            }
            Err(e) => {
                t.fg(term::color::RED).unwrap();
                writeln!(t, "There was an error searching IMDb: {}", e);
                None
            }
        }
    }.boxed()
}

// async fn get_details(id: String) -> IMDBResult {
//     // let req = format!("")

// }

async fn search_show(title: &str) -> Result<SearchResult, reqwest::Error> {
    let request = format!("https://imdb-api.com/en/API/SearchTitle/{}/{}", SECRET, title);
    let response: SearchData = reqwest::get(&request).await?.json().await?; 
    
    Ok(response.results[0].clone())
}

#[derive(Deserialize, Clone)]
struct SearchData{
    search_type: String,
    search_expression: String,
    results: Vec<SearchResult>,
    error_message: String
}

#[derive(Deserialize, Clone)]
struct SearchResult {
    id: String,
    result_type: String,
    image: String,
    title: String,
    description: String,
}

#[derive(Deserialize)]
struct IMDBResult{

}