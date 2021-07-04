pub mod normalize2ascii {
    use kana::wide2ascii;
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(value: &str, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(wide2ascii(value).as_str())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(wide2ascii(s.as_str()))
    }
}

#[cfg(test)]
mod test_normalize2ascii {
    use super::*;
    use anyhow::Result;
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    #[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Clone, Hash)]
    struct Point {
        #[serde(with = "normalize2ascii")]
        name: String,
        lat: i32,
        lng: i32,
    }

    #[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Clone, Hash)]
    struct Human {
        id: String,
        #[serde(with = "normalize2ascii")]
        name: String,
        home: Point,
    }

    #[test]
    fn normalize_deserialize_alphabets_numbers_and_brackets_to_half_wide() -> Result<()> {
        let ichiro = json!({
            "id": "１３",
            "name": "ＩＣＨＩＲＯ（｛［イチロウ］｝）",
            "home": {
                "name": "ＡＯＭＯＲＩ",
                "lat": 35,
                "lng": 135,
            }
        });
        let actual: Human = serde_json::from_value(ichiro)?;

        assert_eq!(actual.id, "１３", "Not applied");
        assert_eq!(actual.name, "ICHIRO({[イチロウ]})", "Applied");
        assert_eq!(actual.home.name, "AOMORI", "Applied nest case");
        assert_eq!(actual.home.lat, 35);
        assert_eq!(actual.home.lng, 135);

        Ok(())
    }

    #[test]
    fn normalize_serialize_alphabets_numbers_and_brackets_to_half_wide() -> Result<()> {
        let e = Human {
            id: "１３".to_string(),
            name: "ＩＣＨＩＲＯ（｛［イチロウ］｝）".to_string(),
            home: Point {
                name: "ＡＯＭＯＲＩ".to_string(),
                lat: 35,
                lng: 135,
            },
        };
        let actual = serde_json::to_string_pretty(&e)?;

        assert_eq!(
            actual,
            r#"
{
  "id": "１３",
  "name": "ICHIRO({[イチロウ]})",
  "home": {
    "name": "AOMORI",
    "lat": 35,
    "lng": 135
  }
}"#
            .trim()
        );

        Ok(())
    }
}
