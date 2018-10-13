use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Target {
    pub kind: Vec<String>,
    pub name: String,
    pub src_path: String,
    pub crate_types: Vec<String>,
    pub edition: Edition,
    #[serde(rename = "required-features")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required_features: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Edition {
    Edition2015,
    Edition2018,
    Other(String),
}

impl Serialize for Edition {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let name = match *self {
            Edition::Edition2015 => "2015",
            Edition::Edition2018 => "2018",
            Edition::Other(ref name) => name,
        };
        name.serialize(s)
    }
}

impl<'de> Deserialize<'de> for Edition {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let name = String::deserialize(d)?;
        match name.as_str() {
            "2015" => return Ok(Edition::Edition2015),
            "2018" => return Ok(Edition::Edition2015),
            _ => {}
        };
        Ok(Edition::Other(name))
    }
}
