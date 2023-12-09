use super::{
    battle::{gen_battle_query, BattleQuery},
    capital::{gen_capital_query, CapitalQuery},
    country::{gen_country_query, CountryQuery},
    league::{gen_league_query, LeagueQuery},
    league_member::{gen_league_member_query, LeagueMemberQuery},
    state::{gen_state_query, StateQuery},
    war::{gen_war_query, WarQuery},
};
use std::str::FromStr;
use strum::ParseError;
use strum_macros::{Display, EnumIter, EnumString};

#[derive(Debug, PartialEq, EnumString, Display, EnumIter)]
pub enum QueryTypes {
    #[strum(serialize = "Country")]
    Country(CountryQuery),
    Capital(CapitalQuery),
    War(WarQuery),
    Battle(BattleQuery),
    State(StateQuery),
    League(LeagueQuery),
    LeagueMember(LeagueMemberQuery),
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
        Ok(QueryTypes::Capital(_)) => {
            let capital_query = CapitalQuery::from_str(&target);
            match capital_query {
                Ok(query) => Ok(QueryTypes::Capital(query)),
                Err(_) => Err(ParseError::VariantNotFound),
            }
        }
        Ok(QueryTypes::War(_)) => {
            let war_query = WarQuery::from_str(&target);
            match war_query {
                Ok(query) => Ok(QueryTypes::War(query)),
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
        Ok(QueryTypes::State(_)) => {
            let state_query = StateQuery::from_str(&target);
            match state_query {
                Ok(query) => Ok(QueryTypes::State(query)),
                Err(_) => Err(ParseError::VariantNotFound),
            }
        }
        Ok(QueryTypes::League(_)) => {
            let league_query = LeagueQuery::from_str(&target);
            match league_query {
                Ok(query) => Ok(QueryTypes::League(query)),
                Err(_) => Err(ParseError::VariantNotFound),
            }
        }
        Ok(QueryTypes::LeagueMember(_)) => {
            let league_member_query = LeagueMemberQuery::from_str(&target);
            match league_member_query {
                Ok(query) => Ok(QueryTypes::LeagueMember(query)),
                Err(_) => Err(ParseError::VariantNotFound),
            }
        }
        _ => Err(ParseError::VariantNotFound),
    }
}

pub fn gen_query(query_type: QueryTypes) -> String {
    match query_type {
        QueryTypes::Country(query) => gen_country_query(query),
        QueryTypes::Capital(query) => gen_capital_query(query),
        QueryTypes::War(query) => gen_war_query(query),
        QueryTypes::Battle(query) => gen_battle_query(query),
        QueryTypes::State(query) => gen_state_query(query),
        QueryTypes::League(query) => gen_league_query(query),
        QueryTypes::LeagueMember(query) => gen_league_member_query(query),
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

        let query = get_query_type("State".to_string(), "hoge".to_string());
        assert_eq!(query, Err(ParseError::VariantNotFound));

        let query = get_query_type("League".to_string(), "hoge".to_string());
        assert_eq!(query, Err(ParseError::VariantNotFound));

        let query = get_query_type("League".to_string(), "hoge".to_string());
        assert_eq!(query, Err(ParseError::VariantNotFound));
    }
}
