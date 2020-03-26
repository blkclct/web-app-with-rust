use serde::{de, Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    id: u64,
    name: Name,
}

impl User {
    pub fn id(&self) -> u64 {
        self.id
    }
}

#[derive(Clone, Debug, Serialize)]
struct Name(String);

impl Name {
    pub fn new(name: &str) -> Result<Self, String> {
        let size = name.chars().count();
        if size < 1 || size > 10 {
            return Err("名前は10文字以内です".to_string());
        }

        if name.chars().any(|c| !c.is_ascii_alphabetic()) {
            return Err("名前が使用できる文字種はA-Z, a-zです".to_string());
        }
        Ok(Name(name.to_string()))
    }
}

impl<'de> de::Deserialize<'de> for Name {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Name::new(&s).map_err(de::Error::custom)
    }
}

impl std::str::FromStr for Name {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Name::new(s)
    }
}

impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[test]
fn test_name() {
    let ok_value = "blkclct";
    assert!(Name::new(ok_value).is_ok());

    let ok_value = "A";
    assert!(Name::new(ok_value).is_ok());

    let ok_value = "abcde";
    assert!(Name::new(ok_value).is_ok());

    let ng_value = "0";
    assert!(Name::new(ng_value).is_err());

    let ng_value = "";
    assert!(Name::new(ng_value).is_err());

    let ng_value = "abcdeabcdea";
    assert!(Name::new(ng_value).is_err());
}
