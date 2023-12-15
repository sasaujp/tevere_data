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
use serde_json::{json, Value};
use std::fs::File;
use std::io::BufReader;
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

#[subcmd]
fn merge(
    category: String,
    #[opt(short = 'o', long = "output", default_value = "data")] output: String,
) {
    let directory = format!("{}/sparql/{}", output, category);
    let base_path = PathBuf::from(directory);

    let mut result = Value::Object(Default::default());
    match fs::read_dir(base_path) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let path = entry.path();
                        if !path.is_file() {
                            continue;
                        }

                        let file = File::open(path).unwrap();
                        let reader = BufReader::new(file);
                        let raw_data: SparqlResponse = serde_json::from_reader(reader).unwrap();
                        raw_data.results.bindings.iter().for_each(|binding| {
                            let entity = &binding[&category];
                            if result.get(&entity.value).is_none() {
                                result[&entity.value] = json!({});
                            }
                            if let Some(object) = result.get_mut(&entity.value) {
                                let capital = &binding.get("capital");
                                match capital {
                                    Some(capital) => {
                                        if !object["capital"].is_object() {
                                            object["capital"] = json!({});
                                        }

                                        let start_time = &binding.get("startTime");
                                        let end_time = &binding.get("endTime");
                                        let point_in_time = &binding.get("pointInTime");
                                        let mut capital_entry = json!({});
                                        match (start_time, end_time, point_in_time) {
                                            (Some(start_time), Some(end_time), _) => {
                                                capital_entry["start_time"] =
                                                    json!(start_time.value);
                                                capital_entry["end_time"] = json!(end_time.value);
                                            }
                                            (_, _, Some(point_in_time)) => {
                                                capital_entry["point_in_time"] =
                                                    json!(point_in_time.value);
                                            }
                                            _ => {}
                                        }
                                        object["capital"][&capital.value] = capital_entry;
                                        // entry.push(capital_entry);
                                        // println!("{}", binding[&keys[0]].value);
                                        return;
                                    }
                                    _ => {}
                                }
                                let label = &binding.get("label");
                                let language = &binding.get("language");
                                match (label, language) {
                                    (Some(label), Some(language)) => {
                                        if !object["label"].is_object() {
                                            object["label"] = json!({});
                                        }
                                        object["label"][&language.value] = json!(label.value);
                                        return;
                                    }
                                    _ => {}
                                }
                                let keys: Vec<_> = binding
                                    .keys()
                                    .filter(|&key| key != &category)
                                    .cloned()
                                    .collect();
                                if keys.len() == 1 {
                                    if !object[&keys[0]].is_array() {
                                        object[&keys[0]] = json!([]);
                                    }

                                    if let Some(Value::Array(entry)) = object.get_mut(&keys[0]) {
                                        entry.push(json!(binding[&keys[0]].value));
                                        // println!("{}", binding[&keys[0]].value);
                                    }
                                    return;
                                }
                            }
                        });
                    }
                    Err(e) => println!("エラー: {}", e),
                }
            }
        }
        Err(e) => println!("ディレクトリ読み込みエラー: {}", e),
    };
    let output_directory = PathBuf::from(format!("{}/result", output));
    let output_path = PathBuf::from(format!("{}/result/{}.json", output, category));

    fs::create_dir_all(&output_directory).unwrap();

    let json = serde_json::to_string_pretty(&result).unwrap();

    fs::write(output_path, json).expect("Unable to write file");
}

#[cmd_group(commands = [get, get_all, merge])]
#[opt(author, version, about, long_about = None)]
fn main() {}
