use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub enum MapName {
    #[serde(rename = "Desert_Main")]
    Desert,
    #[serde(rename = "Erangel_Main")]
    Erangel,
}

impl fmt::Display for MapName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            MapName::Desert => "Miramar",
            MapName::Erangel => "Erangel",
        };
        f.write_str(msg)
    }
}
