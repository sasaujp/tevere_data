use strum_macros::{Display, EnumIter, EnumString};

use crate::wikidata_queries::country::COUNTRY_QUERY;

#[derive(Debug, PartialEq, EnumString, Display, EnumIter, Clone, Copy, Default)]
pub enum BattleQuery {
    #[strum(serialize = "label")]
    Label,
    #[strum(serialize = "coordinates")]
    Coordinates,
    #[strum(serialize = "partOf")]
    PartOf,
    #[strum(serialize = "person")]
    Person,
    #[strum(serialize = "country")]
    Country,
    #[strum(serialize = "pointInTime")]
    PointInTime,
    #[strum(serialize = "image")]
    Image,
    #[default]
    Unknown,
}

static BATTLE_QUERY: &str = "
    ?battle wdt:P31 wd:Q178561 .
";

pub fn gen_battle_query(battle_query: BattleQuery) -> String {
    let result = match battle_query {
        BattleQuery::Label => {
            format!(
                "SELECT DISTINCT ?battle ?label ?language WHERE {{
                    {}
                    ?battle rdfs:label ?label .
                    BIND (LANG(?label) AS ?language)
                }}",
                BATTLE_QUERY
            )
        }
        BattleQuery::Coordinates => {
            format!(
                "SELECT DISTINCT ?battle ?coordinates WHERE {{
                    {}
                    ?battle wdt:P625 ?coordinates .
                }}",
                BATTLE_QUERY
            )
        }
        BattleQuery::PartOf => {
            format!(
                "SELECT DISTINCT ?battle ?partOf WHERE {{
                    {}
                    ?battle wdt:P361 ?partOf .
                }}",
                BATTLE_QUERY
            )
        }
        BattleQuery::Person => {
            format!(
                "SELECT DISTINCT ?battle ?person WHERE {{
                    {}
                    ?battle wdt:P710 ?person .
                    ?person wdt:P31 wd:Q5 .
                }}",
                BATTLE_QUERY
            )
        }
        BattleQuery::Country => {
            format!(
                "SELECT DISTINCT ?battle ?country WHERE {{
                    {}
                    ?battle wdt:P710 ?country .
                    {}
                }}",
                BATTLE_QUERY, COUNTRY_QUERY
            )
        }
        BattleQuery::PointInTime => {
            format!(
                "SELECT DISTINCT ?battle ?pointInTime WHERE {{
                    {}
                    ?battle wdt:P585 ?pointInTime .
                }}",
                BATTLE_QUERY
            )
        }
        BattleQuery::Image => {
            format!(
                "SELECT DISTINCT ?battle ?image WHERE {{
                    {}
                    ?battle wdt:P18 ?image .
                }}",
                BATTLE_QUERY
            )
        }

        BattleQuery::Unknown => "Unknown".to_string(),
    };
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn test_queries_start_with_select_distinct() {
        for variant in BattleQuery::iter() {
            if variant == BattleQuery::Unknown {
                continue;
            }
            let query = gen_battle_query(variant);
            assert!(
                query.starts_with("SELECT DISTINCT"),
                "Failed for variant: {:?}",
                variant
            );
        }
    }
}
