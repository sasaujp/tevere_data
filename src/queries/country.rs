use strum_macros::{Display, EnumIter, EnumString};

#[derive(Debug, PartialEq, EnumString, Display, EnumIter, Clone, Copy, Default)]
pub enum CountryQuery {
    #[strum(serialize = "Inception")]
    Inception,
    #[strum(serialize = "Dissolution")]
    Dissolution,
    #[strum(serialize = "Coordinates")]
    Coordinates,
    #[strum(serialize = "Capital")]
    Capital,
    #[strum(serialize = "Label")]
    Label,
    #[strum(serialize = "Flag")]
    Flag,
    #[default]
    Unknown,
}

static COUNTRY_QUERY: &str = "
{
?country wdt:P31 wd:Q6256 .
} UNION {
?country wdt:P31 wd:Q3024240 .
}
";

pub fn gen_country_query(country_query: CountryQuery) -> String {
    let result = match country_query {
        CountryQuery::Inception => {
            format!(
                "SELECT DISTINCT ?country ?inception WHERE {{
                    {}
                    ?country wdt:P571 ?inception .
                }}",
                COUNTRY_QUERY
            )
        }
        CountryQuery::Dissolution => {
            format!(
                "SELECT DISTINCT ?country ?dissolution WHERE {{
                    {}
                    ?country wdt:P576 ?dissolution .
                }}",
                COUNTRY_QUERY
            )
        }
        CountryQuery::Coordinates => {
            format!(
                "SELECT DISTINCT ?country ?coordinates WHERE {{
                    {}
                    ?country wdt:P625 ?coordinates .
                }}",
                COUNTRY_QUERY
            )
        }
        CountryQuery::Capital => {
            format!(
                "SELECT DISTINCT ?country ?capital ?startTime ?endTime ?pointInTime WHERE {{
                    {}
                    ?country p:P36 ?capital_statement .
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
                }}",
                COUNTRY_QUERY
            )
        }
        CountryQuery::Label => {
            format!(
                "SELECT DISTINCT ?country ?label ?language WHERE {{
                    {}
                    ?country rdfs:label ?label .
                    BIND (LANG(?label) AS ?language)
                }}",
                COUNTRY_QUERY
            )
        }
        CountryQuery::Flag => {
            format!(
                "SELECT DISTINCT ?country ?flag WHERE {{
                    {}
                    ?country wdt:P41 ?flag .
                }}",
                COUNTRY_QUERY
            )
        }
        CountryQuery::Unknown => "Unknown".to_string(),
    };
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn test_queries_start_with_select_distinct() {
        for variant in CountryQuery::iter() {
            if variant == CountryQuery::Unknown {
                continue;
            }
            let query = gen_country_query(variant);
            assert!(
                query.starts_with("SELECT DISTINCT"),
                "Failed for variant: {:?}",
                variant
            );
        }
    }
}
