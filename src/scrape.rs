use error_chain::error_chain;
use rand::Rng;
use scraper::{Html, Selector};


error_chain! {
      foreign_links {
          ReqError(reqwest::Error);
          IoError(std::io::Error);
      }
}

pub async fn juke() -> Result<String> {
    let body = reqwest::get("http://www.funnyshortjokes.com/")  
        .await?
        .text()
        .await?;

    let document = Html::parse_fragment(&body);
    let selector = Selector::parse("p").unwrap();
    let jokes = document
        .select(&selector)
        .map(|joke| joke.text().collect::<String>())
        .collect::<Vec<_>>();

    let mut rng = rand::thread_rng();
    let n = rng.gen_range(0..12);
  

    let s = jokes[n].clone();

    Ok(s)
}
