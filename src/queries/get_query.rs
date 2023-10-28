use super::country::{gen_country_query, CountryQuery};
use std::str::FromStr;
use strum::ParseError;
use strum_macros::{Display, EnumIter, EnumString};

#[derive(Debug, PartialEq, EnumString, Display, EnumIter)]
pub enum QueryTypes {
    #[strum(serialize = "Country")]
    Country(CountryQuery),
}

pub fn get_query_type(category: String, target: String) -> Result<QueryTypes, ParseError> {
    let query_type = QueryTypes::from_str(&category);
    match query_type {
        Ok(QueryTypes::Country(_)) => {
            let country_query = CountryQuery::from_str(&target);
            match country_query {
                Ok(query) => Ok(QueryTypes::Country(query)),
                Err(_) => Err(ParseError::VariantNotFound),
            }
        }
        _ => Err(ParseError::VariantNotFound),
    }
}

pub fn gen_query(query_type: QueryTypes) -> String {
    match query_type {
        QueryTypes::Country(query) => gen_country_query(query),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_query() {
        let query = get_query_type("Country".to_string(), "Inception".to_string());
        assert_eq!(query, Ok(QueryTypes::Country(CountryQuery::Inception)));

        let query = get_query_type("Country".to_string(), "Dissolution".to_string());
        assert_eq!(query, Ok(QueryTypes::Country(CountryQuery::Dissolution)));

        let query = get_query_type("hoge".to_string(), "hoge".to_string());
        assert_eq!(query, Err(ParseError::VariantNotFound));

        let query = get_query_type("Country".to_string(), "hoge".to_string());
        assert_eq!(query, Err(ParseError::VariantNotFound));
    }
}
