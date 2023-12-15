use strum_macros::{Display, EnumIter, EnumString};

#[derive(Debug, PartialEq, EnumString, Display, EnumIter, Clone, Copy, Default)]
pub enum BattleQuery {
    #[strum(serialize = "abstract")]
    Abstract,
    #[default]
    Unknown,
}

static BATTLE_QUERY: &str = "
    ?battle rdf:type dbo:MilitaryConflict .
    ?battle rdf:type dbo:Event .
";

pub fn gen_battle_query(battle_query: BattleQuery) -> String {
    let result = match battle_query {
        BattleQuery::Abstract => {
            format!(
                "
                prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#>
                prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
                prefix dbo: <http://dbpedia.org/ontology/>
                select distinct ?battle ?abstract ?language where {{
                    {}
                    ?battle dbo:abstract ?abstract .
                    BIND (LANG(?abstract) AS ?language)
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
    fn test_queries_start_with_abstract() {
        for variant in BattleQuery::iter() {
            if variant == BattleQuery::Unknown {
                continue;
            }
            let query = gen_battle_query(variant);
            assert!(
                query.starts_with("prefix"),
                "Failed for variant: {:?}",
                variant
            );
        }
    }
}
