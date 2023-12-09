use strum_macros::{Display, EnumIter, EnumString};

#[derive(Debug, PartialEq, EnumString, Display, EnumIter, Clone, Copy, Default)]
pub enum WarQuery {
    #[strum(serialize = "Label")]
    Label,
    #[strum(serialize = "Coordinates")]
    Coordinates,
    #[strum(serialize = "Person")]
    Person,
    #[strum(serialize = "StartDate")]
    StartDate,
    #[strum(serialize = "EndDate")]
    EndDate,
    #[strum(serialize = "Country")]
    Country,
    #[default]
    Unknown,
}
static WAR_QUERY: &str = "
    ?war wdt:P31/wdt:P279* wd:Q198 .
";

pub fn gen_war_query(war_query: WarQuery) -> String {
    let result = match war_query {
        WarQuery::Label => {
            format!(
                "SELECT DISTINCT ?war ?label ?language WHERE {{
                    {}
                    ?war rdfs:label ?label .
                    BIND (LANG(?label) AS ?language)
                }}
                ",
                WAR_QUERY
            )
        }
        WarQuery::Coordinates => {
            format!(
                "SELECT DISTINCT ?capital ?coordinates WHERE {{
                    {}
                    ?country wdt:P36 ?capital .
                    ?capital wdt:P625 ?coordinates .
                }}",
                WAR_QUERY
            )
        }
        WarQuery::Person => {
            format!(
                "SELECT DISTINCT ?war ?person WHERE {{
                    {}
                    ?war wdt:P710 ?person .
                    ?person wdt:P31 wd:Q5 .
                }}",
                WAR_QUERY
            )
        }
        WarQuery::StartDate => {
            format!(
                "SELECT DISTINCT ?war ?startDate WHERE {{
                    {}
                    ?war wdt:P580 ?startDate .
                }}",
                WAR_QUERY
            )
        }
        WarQuery::EndDate => {
            format!(
                "SELECT DISTINCT ?war ?endDate WHERE {{
                    {}
                    ?war wdt:P582 ?endDate .
                }}",
                WAR_QUERY
            )
        }
        WarQuery::Country => {
            format!(
                "SELECT DISTINCT ?war ?country WHERE {{
                    {}
                    ?war wdt:P710 ?country .
                    ?country wdt:P31 wd:Q5 .
                }}",
                WAR_QUERY
            )
        }
        WarQuery::Unknown => "Unknown".to_string(),
    };
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn test_queries_start_with_select_distinct() {
        for variant in WarQuery::iter() {
            if variant == WarQuery::Unknown {
                continue;
            }
            let query = gen_war_query(variant);
            assert!(
                query.starts_with("SELECT DISTINCT"),
                "Failed for variant: {:?}",
                variant
            );
        }
    }
}
