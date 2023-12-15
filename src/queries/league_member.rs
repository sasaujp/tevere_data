use crate::queries::league::LEAGUE_QUERY;
use strum_macros::{Display, EnumIter, EnumString};
#[derive(Debug, PartialEq, EnumString, Display, EnumIter, Clone, Copy, Default)]
pub enum LeagueMemberQuery {
    #[strum(serialize = "label")]
    Label,
    #[strum(serialize = "coordinates")]
    Coordinates,
    #[strum(serialize = "flag")]
    Flag,

    #[default]
    Unknown,
}

pub fn gen_league_member_query(league_member_query: LeagueMemberQuery) -> String {
    let result = match league_member_query {
        LeagueMemberQuery::Label => {
            format!(
                "SELECT DISTINCT ?league_member ?label ?language WHERE {{
                    {}
                    BIND (?state AS ?league_member)
                    ?league_member rdfs:label ?label .
                    BIND (LANG(?label) AS ?language)
                }}
                ",
                LEAGUE_QUERY
            )
        }
        LeagueMemberQuery::Coordinates => {
            format!(
                "SELECT DISTINCT ?league_member ?coordinates WHERE {{
                    {}
                    BIND (?state AS ?league_member)
                    ?league_member wdt:P625 ?coordinates .
                }}
                ",
                LEAGUE_QUERY
            )
        }
        LeagueMemberQuery::Flag => {
            format!(
                "SELECT DISTINCT ?league_member ?flag WHERE {{
                    {}
                    BIND (?state AS ?league_member)
                    ?league_member wdt:P41 ?flag .
                }}
                ",
                LEAGUE_QUERY
            )
        }
        LeagueMemberQuery::Unknown => "".to_string(),
    };
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn test_queries_start_with_select_distinct() {
        for variant in LeagueMemberQuery::iter() {
            if variant == LeagueMemberQuery::Unknown {
                continue;
            }
            let query = gen_league_member_query(variant);
            assert!(
                query.starts_with("SELECT DISTINCT"),
                "Failed for variant: {:?}",
                variant
            );
        }
    }
}
