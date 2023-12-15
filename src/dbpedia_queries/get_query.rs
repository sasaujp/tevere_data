use super::{
    battle::{gen_battle_query, BattleQuery},
    country::{gen_country_query, CountryQuery},
};
use std::str::FromStr;
use strum::ParseError;
use strum_macros::{Display, EnumIter, EnumString};

#[derive(Debug, PartialEq, EnumString, Display, EnumIter)]
pub enum QueryTypes {
    #[strum(serialize = "country")]
    Country(CountryQuery),
    // #[strum(serialize = "capital")]
    // Capital(CapitalQuery),
    // #[strum(serialize = "war")]
    // War(WarQuery),
    #[strum(serialize = "battle")]
    Battle(BattleQuery),
    // #[strum(serialize = "state")]
    // State(StateQuery),
    // #[strum(serialize = "league")]
    // League(LeagueQuery),
    // #[strum(serialize = "league_member")]
    // LeagueMember(LeagueMemberQuery),
}

pub fn get_query_type(category: &str, target: &str) -> Result<QueryTypes, ParseError> {
    let query_type = QueryTypes::from_str(&category);
    match query_type {
        Ok(QueryTypes::Country(_)) => {
            let country_query = CountryQuery::from_str(&target);
            match country_query {
                Ok(query) => Ok(QueryTypes::Country(query)),
                Err(_) => Err(ParseError::VariantNotFound),
            }
        }
        Ok(QueryTypes::Battle(_)) => {
            let battle_query = BattleQuery::from_str(&target);
            match battle_query {
                Ok(query) => Ok(QueryTypes::Battle(query)),
                Err(_) => Err(ParseError::VariantNotFound),
            }
        }

        _ => Err(ParseError::VariantNotFound),
    }
}

pub fn gen_query(query_type: QueryTypes) -> String {
    match query_type {
        QueryTypes::Country(query) => gen_country_query(query),
        QueryTypes::Battle(query) => gen_battle_query(query),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_query() {
        let query = get_query_type("country", "inception");
        assert_eq!(query, Ok(QueryTypes::Country(CountryQuery::Inception)));

        let query = get_query_type("hoge", "hoge");
        assert_eq!(query, Err(ParseError::VariantNotFound));

        let query = get_query_type("country", "hoge");
        assert_eq!(query, Err(ParseError::VariantNotFound));
    }
}
