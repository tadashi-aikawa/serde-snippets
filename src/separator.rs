pub mod separate_by_comma {
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(value: &Vec<String>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&value.join(","))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(s.split(',')
            .into_iter()
            .map(String::from)
            .collect::<Vec<_>>())
    }
}

pub mod option_separate_by_comma {
    use serde::{self, Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(value: &Option<Vec<String>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        value.as_ref().map(|xs| xs.join(",")).serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<String>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = Option::<String>::deserialize(deserializer)?;
        Ok(s.map(|x| {
            x.split(',')
                .into_iter()
                .map(String::from)
                .collect::<Vec<_>>()
        }))
    }
}

#[cfg(test)]
mod test_separate_by_comma {
    use super::*;
    use anyhow::Result;
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    #[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Clone, Hash)]
    struct Human {
        id: String,
        #[serde(with = "separate_by_comma")]
        names: Vec<String>,
    }

    #[test]
    fn serialize_separate_by_comma() -> Result<()> {
        let ichiro = json!({
            "id": "1",
            "names": "aa,いい,uu",
        });
        let actual: Human = serde_json::from_value(ichiro)?;

        assert_eq!(actual.id, "1", "Not applied");
        assert_eq!(actual.names, vec!["aa", "いい", "uu"], "Applied");

        Ok(())
    }
}

#[cfg(test)]
mod test_option_separate_by_comma {
    use super::*;
    use anyhow::Result;
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    #[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Clone, Hash)]
    struct Human {
        id: String,
        #[serde(with = "option_separate_by_comma")]
        names: Option<Vec<String>>,
    }

    #[test]
    fn serialize_separate_by_comma() -> Result<()> {
        let ichiro = json!({
            "id": "1",
            "names": "aa,いい,uu",
        });
        let actual: Human = serde_json::from_value(ichiro)?;

        assert_eq!(actual.id, "1", "Not applied");
        assert_eq!(
            actual.names,
            Some(vec!["aa".to_string(), "いい".to_string(), "uu".to_string()]),
            "Applied"
        );

        Ok(())
    }

    #[test]
    fn serialize_none() -> Result<()> {
        let ichiro = json!({
            "id": "1",
            "names": null,
        });
        let actual: Human = serde_json::from_value(ichiro)?;

        assert_eq!(actual.id, "1", "Not applied");
        assert_eq!(actual.names, None, "Applied");

        Ok(())
    }
}
