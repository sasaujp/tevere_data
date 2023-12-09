use strum_macros::{Display, EnumIter, EnumString};

#[derive(Debug, PartialEq, EnumString, Display, EnumIter, Clone, Copy, Default)]
pub enum StateQuery {
    #[strum(serialize = "Inception")]
    Inception,
    #[strum(serialize = "Dissolution")]
    Dissolution,
    #[strum(serialize = "Coordinates")]
    Coordinates,
    #[strum(serialize = "Label")]
    Label,
    #[strum(serialize = "Flag")]
    Flag,
    #[strum(serialize = "Capital")]
    Capital,
    #[default]
    Unknown,
}

static STATE_QUERY: &str = "
    {
        ?state wdt:P31 wd:Q7275 .
    } UNION {
        ?state wdt:P31 wd:Q133442 .
    } UNION {
        ?state wdt:P31 wd:Q148837 .
    }
    FILTER NOT EXISTS { ?state wdt:P31 wd:Q6256 . }
    FILTER NOT EXISTS { ?state wdt:P31 wd:Q3024240 .}
    ?state wdt:P571 ?inception .
    ?state wdt:P576 ?dissolution .  
";

pub fn gen_state_query(state_query: StateQuery) -> String {
    let result = match state_query {
        StateQuery::Inception => {
            format!(
                "SELECT DISTINCT ?state ?inception WHERE {{
                    {}
                }}
                ",
                STATE_QUERY
            )
        }
        StateQuery::Dissolution => {
            format!(
                "SELECT DISTINCT ?state ?dissolution WHERE {{
                    {}
                }}
                ",
                STATE_QUERY
            )
        }
        StateQuery::Coordinates => {
            format!(
                "SELECT DISTINCT ?state ?coordinates WHERE {{
                    {}
                    ?state wdt:P625 ?coordinates .
                }}
                ",
                STATE_QUERY
            )
        }
        StateQuery::Label => {
            format!(
                "SELECT DISTINCT ?state ?label ?language WHERE {{
                    {}
                    ?state rdfs:label ?label .
                    BIND (LANG(?label) AS ?language) .
                }}
                ",
                STATE_QUERY
            )
        }
        StateQuery::Flag => {
            format!(
                "SELECT DISTINCT ?state ?flag WHERE {{
                    {}
                    ?state wdt:P41 ?flag .
                }}
                ",
                STATE_QUERY
            )
        }
        StateQuery::Capital => {
            format!(
                "SELECT DISTINCT ?state ?capital ?startTime ?endTime ?pointInTime WHERE {{
                    {}
                    ?state p:P36 ?capital_statement .
                    ?capital_statement ps:P36 ?capital .
                    OPTIONAL {{
                        ?capital_statement pq:P580 ?startTime .
                    }}
                    OPTIONAL {{
                        ?capital_statement pq:P582 ?endTime .
                    }}
                    OPTIONAL {{
                        ?capital_statement pq:P585 ?pointInTime .
                    }}
                }}
                ",
                STATE_QUERY
            )
        }
        StateQuery::Unknown => "Unknown".to_string(),
    };
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn test_queries_start_with_select_distinct() {
        for variant in StateQuery::iter() {
            if variant == StateQuery::Unknown {
                continue;
            }
            let query = gen_state_query(variant);
            assert!(
                query.starts_with("SELECT DISTINCT"),
                "Failed for variant: {:?}",
                variant
            );
        }
    }
}
