use strum_macros::{Display, EnumIter, EnumString};

#[derive(Debug, PartialEq, EnumString, Display, EnumIter, Clone, Copy, Default)]
pub enum LeagueQuery {
    #[strum(serialize = "Inception")]
    Inception,
    #[strum(serialize = "Dissolution")]
    Dissolution,
    #[strum(serialize = "Label")]
    Label,
    #[strum(serialize = "State")]
    State,
    #[strum(serialize = "Flag")]
    Flag,
    #[default]
    Unknown,
}

pub static LEAGUE_QUERY: &str = "
    ?league wdt:P31 wd:Q170156 .
    FILTER NOT EXISTS { ?league wdt:P31 wd:Q6256 . }
    FILTER NOT EXISTS { ?league wdt:P31 wd:Q3024240 . }
    {
        ?state wdt:P463 ?league .
    } UNION {
        ?league wdt:P150 ?state .
    }
    {
        ?state wdt:P31 wd:Q6256 .
    } UNION {
        ?state wdt:P31 wd:Q3024240 .
    } UNION {
        ?state wdt:P31 wd:Q7275 .
    } UNION {
        ?state wdt:P31/wdt:P279* wd:Q515 .
    } UNION {
        ?state wdt:P31 wd:Q133442 .
    } UNION {
        ?state wdt:P31 wd:Q148837 .
    }
";

pub fn gen_league_query(league_query: LeagueQuery) -> String {
    let result = match league_query {
        LeagueQuery::Label => {
            format!(
                "SELECT DISTINCT ?league ?label ?language WHERE {{
                    {}
                    ?league rdfs:label ?label .
                    BIND (LANG(?label) AS ?language)
                }}
                ",
                LEAGUE_QUERY
            )
        }
        LeagueQuery::Inception => {
            format!(
                "SELECT DISTINCT ?league ?inception WHERE {{
                    {}
                    ?league wdt:P571 ?inception .
                }}
                ",
                LEAGUE_QUERY
            )
        }
        LeagueQuery::Dissolution => {
            format!(
                "SELECT DISTINCT ?league ?dissolution WHERE {{
                    {}
                    ?league wdt:P576 ?dissolution .
                }}
                ",
                LEAGUE_QUERY
            )
        }
        LeagueQuery::State => {
            format!(
                "SELECT DISTINCT ?league ?state WHERE {{
                    {}
                }}
                ",
                LEAGUE_QUERY
            )
        }
        LeagueQuery::Flag => {
            format!(
                "SELECT DISTINCT ?league ?flag WHERE {{
                    {}
                    ?league wdt:P41 ?flag .
                }}
                ",
                LEAGUE_QUERY
            )
        }
        LeagueQuery::Unknown => {
            format!(
                "SELECT DISTINCT ?league ?label ?language WHERE {{
                    {}
                    ?league rdfs:label ?label .
                    BIND (LANG(?label) AS ?language)
                }}
                ",
                LEAGUE_QUERY
            )
        }
    };
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn test_queries_start_with_select_distinct() {
        for variant in LeagueQuery::iter() {
            if variant == LeagueQuery::Unknown {
                continue;
            }
            let query = gen_league_query(variant);
            assert!(
                query.starts_with("SELECT DISTINCT"),
                "Failed for variant: {:?}",
                variant
            );
        }
    }
}
