use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PackageId {
    pub name: String,
    pub version: semver::Version,
    pub source_id: String,
}

impl Serialize for PackageId {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.collect_str(&format_args!(
            "{} {} ({})",
            self.name, self.version, self.source_id,
        ))
    }
}

impl<'de> Deserialize<'de> for PackageId {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        use serde::de::Error;

        let string = String::deserialize(d)?;

        let mut s = string.splitn(3, " ");
        let name = s.next().unwrap();
        if name == "" {
            return Err(Error::missing_field("name"));
        }
        let version = s.next().ok_or_else(|| Error::missing_field("version"))?;
        let version = semver::Version::parse(version).map_err(Error::custom)?;
        let source_id = s.next().ok_or_else(|| Error::missing_field("source_id"))?;
        if !(source_id.starts_with("(") && source_id.ends_with(")")) {
            return Err(Error::custom("source_id not enclosed by ()"));
        }
        let source_id = &source_id[1..source_id.len() - 1];

        Ok(Self {
            name: name.to_owned(),
            version: version.to_owned(),
            source_id: source_id.to_owned(),
        })
    }
}
