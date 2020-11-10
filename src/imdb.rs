use reqwest;
use rustyline;
use serde::Deserialize;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use std::io::prelude::*;
use futures::future::{BoxFuture, FutureExt};
use serde_json::Value;

use crate::manager::Show;

//If you cloned this repo, create this file in the src directory
const SECRET: &'static str = include_str!("secret"); 

pub fn get_show_data(title: &str) -> BoxFuture<'_, Option<Show>>{
    let mut t = StandardStream::stdout(ColorChoice::AlwaysAnsi);
    async move{
        match search_show(title).await{
            Ok(res) => {
                let mut rl = rustyline::Editor::<()>::new();
                t.set_color(ColorSpec::new().set_fg(Some(Color::White)));
                write!(t, "Searched on IMDb and got: ");
                t.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)));
                write!(t, "{}\n", res.title);
                t.set_color(ColorSpec::new().set_fg(Some(Color::White)));
                write!(t, "Add this show to your watchlist? (Y/n/[r]etry) ");
                let input = rl.readline("").unwrap_or("y".to_owned());
                match input.to_lowercase().chars().next() {
                    Some('n') => {
                        writeln!(t, "Alright, this show won't be added to your watchlist");
                        t.reset();
                        None
                    }
                    Some('r') => {
                        write!(t, "Okay, what title should I search for: ");
                        let re_title = rl.readline("").unwrap();
                        t.reset();
                        get_show_data(&re_title).await
                    }
                    _ => { //the default is yes, so everything goes here
                        t.reset();
                        Some(Show {
                            title: res.title,
                            length: get_episodes(res.id).await.unwrap(),
                            completed: false,
                            watched: 0,
                        })
                    }
                    
                }
            }
            Err(e) => {
                t.set_color(ColorSpec::new().set_fg(Some(Color::Red)));
                writeln!(t, "There was an error searching IMDb: {}", e);
                t.reset();
                None
            }
        }
    }.boxed()
}

async fn get_episodes(id: String) -> Result<i32, reqwest::Error> {
    let mut res: usize = 0;
    let mut season: i32 = 1;
    loop{
        let req = format!("https://imdb-api.com/en/API/SeasonEpisodes/{}/{}/{}", SECRET, id, season);
        let response: IMDBResult = reqwest::get(&req).await?.json().await?;
        if response.errorMessage.contains("404") {
            break;
        }else {
            res += response.episodes.len();
            season += 1;
        }
    }

    Ok(res as i32)

}

async fn search_show(title: &str) -> Result<SearchResult, reqwest::Error> {
    let request = format!("https://imdb-api.com/en/API/SearchTitle/{}/{}", SECRET, title);
    let response: SearchData = reqwest::get(&request).await?.json().await?; 
    
    Ok(response.results[0].clone())
}

#[derive(Deserialize, Clone)]
struct SearchData{
    searchType: String,
    expression: String,
    results: Vec<SearchResult>,
    errorMessage: String
}

#[derive(Deserialize, Clone)]
struct SearchResult {
    id: String,
    resultType: String,
    image: String,
    title: String,
    description: String,
}

#[derive(Deserialize)]
struct IMDBResult{
    imDbId: String,
    title: String,
    fullTitle: String,
    r#type: String,
    year: String,
    episodes: Vec<std::collections::HashMap<String, String>>,
    errorMessage: String,
}