mod queries;
use crate::queries::{
    battle::BattleQuery, capital::CapitalQuery, country::CountryQuery, league::LeagueQuery,
    league_member::LeagueMemberQuery, sparql_types::SparqlResponse, state::StateQuery,
    war::WarQuery,
};
use argopt::{cmd_group, subcmd};
use queries::get_query::QueryTypes;
use reqwest::blocking::Client;
use serde_json;
use std::thread;
use std::time::Duration;
use std::{fs, path::PathBuf};
use strum::IntoEnumIterator;
use urlencoding::encode;

const WIKIDATA_ENDPOINT: &str = "https://query.wikidata.org/sparql";

fn fetch(category: &str, target: &str, data_dir: &str) {
    let query_type = queries::get_query::get_query_type(&category, &target);
    let sparql = match query_type {
        Ok(query) => queries::get_query::gen_query(query),
        Err(_) => {
            println!("Invalid query type.");
            return; // Add this line to return early from the function
        }
    };
    let url = format!(
        "{}?query={}&format=json",
        WIKIDATA_ENDPOINT,
        encode(&sparql)
    );

    let directory = format!("{}/sparql/{}", data_dir, category);
    let mut path = PathBuf::from(directory);
    fs::create_dir_all(&path).unwrap();
    path.push(format!("{}.json", target));

    let client = Client::builder().user_agent("Reqwest").build().unwrap();
    let result = client.get(url).send().unwrap();
    let value = result.json::<SparqlResponse>();
    match value {
        Ok(value) => {
            let json = serde_json::to_string_pretty(&value).unwrap();
            fs::write(path, json).expect("Unable to write file");
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
    sleep();
}

#[subcmd]
fn get(
    category: String,
    target: String,
    #[opt(short = 'o', long = "output", default_value = "data")] output: String,
) {
    fetch(&category, &target, &output);
}

fn sleep() {
    thread::sleep(Duration::from_secs(3));
}

#[subcmd]
fn get_all(#[opt(short = 'o', long = "output", default_value = "data")] output: String) {
    for query_type in QueryTypes::iter() {
        match query_type {
            QueryTypes::Country(_) => {
                for variant in CountryQuery::iter() {
                    if variant == CountryQuery::Unknown {
                        continue;
                    }
                    println!("{} -> {}", query_type.to_string(), variant.to_string());
                    fetch(&query_type.to_string(), &variant.to_string(), &output);
                }
            }
            QueryTypes::Capital(_) => {
                for variant in CapitalQuery::iter() {
                    if variant == CapitalQuery::Unknown {
                        continue;
                    }
                    println!("{} -> {}", query_type.to_string(), variant.to_string());
                    fetch(&query_type.to_string(), &variant.to_string(), &output);
                }
            }
            QueryTypes::War(_) => {
                for variant in WarQuery::iter() {
                    if variant == WarQuery::Unknown {
                        continue;
                    }
                    println!("{} -> {}", query_type.to_string(), variant.to_string());
                    fetch(&query_type.to_string(), &variant.to_string(), &output);
                }
            }
            QueryTypes::Battle(_) => {
                for variant in BattleQuery::iter() {
                    if variant == BattleQuery::Unknown {
                        continue;
                    }
                    println!("{} -> {}", query_type.to_string(), variant.to_string());
                    fetch(&query_type.to_string(), &variant.to_string(), &output);
                }
            }
            QueryTypes::State(_) => {
                for variant in StateQuery::iter() {
                    if variant == StateQuery::Unknown {
                        continue;
                    }
                    println!("{} -> {}", query_type.to_string(), variant.to_string());
                    fetch(&query_type.to_string(), &variant.to_string(), &output);
                }
            }
            QueryTypes::League(_) => {
                for variant in LeagueQuery::iter() {
                    if variant == LeagueQuery::Unknown {
                        continue;
                    }
                    println!("{} -> {}", query_type.to_string(), variant.to_string());
                    fetch(&query_type.to_string(), &variant.to_string(), &output);
                }
            }
            QueryTypes::LeagueMember(_) => {
                for variant in LeagueMemberQuery::iter() {
                    if variant == LeagueMemberQuery::Unknown {
                        continue;
                    }
                    println!("{} -> {}", query_type.to_string(), variant.to_string());
                    fetch(&query_type.to_string(), &variant.to_string(), &output);
                }
            }
        }
    }
}

#[cmd_group(commands = [get, get_all])]
#[opt(author, version, about, long_about = None)]
fn main() {}
