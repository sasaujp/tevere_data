use strum_macros::{Display, EnumIter, EnumString};

use crate::wikidata_queries::country::COUNTRY_QUERY;

#[derive(Debug, PartialEq, EnumString, Display, EnumIter, Clone, Copy, Default)]
pub enum CapitalQuery {
    #[strum(serialize = "label")]
    Label,
    #[strum(serialize = "coordinates")]
    Coordinates,
    #[default]
    Unknown,
}

pub fn gen_capital_query(capital_query: CapitalQuery) -> String {
    let result = match capital_query {
        CapitalQuery::Label => {
            format!(
                "SELECT DISTINCT ?capital ?label ?language WHERE {{
                    {}
                    ?country wdt:P36 ?capital .
                    ?capital rdfs:label ?label .
                    BIND (LANG(?label) AS ?language)
                }}
            ",
                COUNTRY_QUERY
            )
        }
        CapitalQuery::Coordinates => {
            format!(
                "SELECT DISTINCT ?capital ?coordinates WHERE {{
                    {}
                    ?country wdt:P36 ?capital .
                    ?capital wdt:P625 ?coordinates .
                }}",
                COUNTRY_QUERY
            )
        }
        CapitalQuery::Unknown => "Unknown".to_string(),
    };
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn test_queries_start_with_select_distinct() {
        for variant in CapitalQuery::iter() {
            if variant == CapitalQuery::Unknown {
                continue;
            }
            let query = gen_capital_query(variant);
            assert!(
                query.starts_with("SELECT DISTINCT"),
                "Failed for variant: {:?}",
                variant
            );
        }
    }
}
