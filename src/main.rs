mod queries;

use reqwest::blocking::Client;
use std::error::Error;
use urlencoding::encode;

const WIKIDATA_ENDPOINT: &str = "https://query.wikidata.org/sparql";

#[argopt::cmd]
fn main(category: String, target: String) -> Result<(), Box<dyn Error>> {
    let query_type = queries::get_query::get_query_type(category, target);
    let sparql = match query_type {
        Ok(query) => queries::get_query::gen_query(query),
        Err(_) => {
            println!("Invalid query type.");
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Invalid query type.",
            )))?
        }
    };
    println!("{}", sparql);
    let url = format!(
        "{}?query={}&format=json",
        WIKIDATA_ENDPOINT,
        encode(&sparql)
    );
    let client = Client::builder().user_agent("Reqwest").build().unwrap();
    let result = client.get(url).send().unwrap();

    println!("{:?}", result.status());
    Ok(())
}
