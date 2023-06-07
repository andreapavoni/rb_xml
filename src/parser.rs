use std::fs;
use std::io::Write;

use quick_xml::de::from_str;
use quick_xml::se::Serializer;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default)]
pub struct Parser {
    xml: DjPlaylists,
}

impl Parser {
    pub fn from_file(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let xml = fs::read_to_string(file_path)?;

        Ok(Self::new(&xml)?)
    }

    pub fn new(xml: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            xml: from_str(xml)?,
        })
    }

    pub fn to_xml(&self) -> String {
        let mut buffer = String::new();
        let mut ser = Serializer::new(&mut buffer);
        ser.indent(' ', 2);

        self.xml.serialize(ser).unwrap();
        buffer

        // quick_xml::se::to_string(&self.xml).unwrap()
    }

    pub fn write_to_file(&self, file_path: &str) -> std::io::Result<()> {
        let mut file = fs::File::create(file_path)?;

        let xml_str = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\n";
        file.write_all(xml_str.as_bytes())?;

        let xml_str = self.to_xml();
        file.write_all(xml_str.as_bytes())?;

        Ok(())
    }
}

#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(default, rename = "DJ_PLAYLISTS")]
struct DjPlaylists {
    #[serde(rename = "@Version")]
    version: String,
    #[serde(rename = "PRODUCT")]
    product: Product,
    #[serde(rename = "COLLECTION")]
    collection: Collection,
    #[serde(rename = "PLAYLISTS")]
    playlists: Playlists,
}

#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(default)]
struct Product {
    #[serde(rename = "@Name")]
    name: String,
    #[serde(rename = "@Version")]
    version: String,
    #[serde(rename = "@Company")]
    company: String,
}

#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(default)]
struct Collection {
    #[serde(rename = "@Entries")]
    entries: u32,
    #[serde(rename = "TRACK")]
    track: Vec<Track>,
}

#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(default)]
struct Track {
    #[serde(rename = "@TrackID")]
    track_id: String,
    #[serde(rename = "@Name")]
    name: String,
    #[serde(rename = "@Artist")]
    artist: String,
    #[serde(rename = "@Composer")]
    composer: String,
    #[serde(rename = "@Album")]
    album: String,
    #[serde(rename = "@Grouping")]
    grouping: String,
    #[serde(rename = "@Genre")]
    genre: String,
    #[serde(rename = "@Kind")]
    kind: String,
    #[serde(rename = "@Size")]
    size: String,
    #[serde(rename = "@TotalTime")]
    total_time: String,
    #[serde(rename = "@DiscNumber")]
    disc_number: String,
    #[serde(rename = "@TrackNumber")]
    track_number: String,
    #[serde(rename = "@Year")]
    year: String,
    #[serde(rename = "@AverageBpm")]
    average_bpm: String,
    #[serde(rename = "@DateAdded")]
    date_added: String,
    #[serde(rename = "@BitRate")]
    bit_rate: String,
    #[serde(rename = "@SampleRate")]
    sample_rate: String,
    #[serde(rename = "@Comments")]
    comments: String,
    #[serde(rename = "@PlayCount")]
    play_count: String,
    #[serde(rename = "@Rating")]
    rating: String,
    #[serde(rename = "@Location")]
    location: String,
    #[serde(rename = "@Remixer")]
    remixer: String,
    #[serde(rename = "@Tonality")]
    tonality: String,
    #[serde(rename = "@Label")]
    label: String,
    #[serde(rename = "@Mix")]
    mix: String,
    #[serde(rename = "TEMPO")]
    tempo: Vec<Tempo>,
    #[serde(rename = "POSITION_MARK")]
    position_mark: Vec<PositionMark>,
}

#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(default)]
struct Tempo {
    #[serde(rename = "@Inizio")]
    inizio: String,
    #[serde(rename = "@Bpm")]
    bpm: String,
    #[serde(rename = "@Metro")]
    metro: String,
    #[serde(rename = "@Battito")]
    battito: String,
}

#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(default)]
struct PositionMark {
    #[serde(rename = "@Name")]
    name: String,
    #[serde(rename = "@Type")]
    type_: String,
    #[serde(rename = "@Start")]
    start: String,
    #[serde(rename = "@Num")]
    num: String,
    #[serde(rename = "@Red")]
    red: String,
    #[serde(rename = "@Green")]
    green: String,
    #[serde(rename = "@Blue")]
    blue: String,
}

#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(default)]
struct Playlists {
    #[serde(rename = "NODE")]
    node: Vec<PlaylistNode>,
}

#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(default, rename = "NODE")]
struct PlaylistNode {
    #[serde(rename = "@Name")]
    name: String,
    #[serde(rename = "@Type")]
    node_type: String,
    #[serde(rename = "@Count", skip_serializing_if = "Option::is_none")]
    count: Option<u32>,
    #[serde(rename = "@KeyType", skip_serializing_if = "Option::is_none")]
    key_type: Option<u32>,
    #[serde(rename = "@Entries", skip_serializing_if = "Option::is_none")]
    entries: Option<u32>,
    #[serde(rename = "TRACK", skip_serializing_if = "Option::is_none" )]
    track: Option<Vec<NodeTrack>>,
    #[serde(rename = "NODE", skip_serializing_if = "Option::is_none")]
    node: Option<Vec<PlaylistNode>>,
}

#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(default)]
struct NodeTrack {
    #[serde(rename = "@Key")]
    key: u32,
}
