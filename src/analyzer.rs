use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

use url::Url;

use crate::RekordboxXml;

pub struct Analyzer {
    xml: RekordboxXml,
    pub report: AnalyzerReport,
    folder_contents: Vec<String>,
}

const SUPPORTED_EXTENSIONS: &'static [&'static str] = &["mp3", "aac", "wav", "flac"];

impl Analyzer {
    pub fn new(xml: RekordboxXml, path: &str) -> Self {
        Self {
            xml,
            report: AnalyzerReport::default(),
            folder_contents: list_supported_files(path),
        }
    }

    // - Locating files referenced by tracks in the collection...
    //
    //
    // - MISSING, searching for file '3-09 Inside My Love.m4a'...
    //  - Found one matching filename, relocating in rekordbox to '/Users/Ed/rekordbox/Compilations/Defected In The House - Louie Vega/3-09 Inside My Love.m4a'
    //
    // - MISSING, searching for file '01 Who Are The Warriors - Basskleph Remix.mp3'...
    //  - Found more than one matching filename so it's not safe to automatically relocate. Matches were:
    //     /Users/Ed/rekordbox/Stantons Remixed/01 Who Are The Warriors - Basskleph Remix.mp3
    //     /Users/Ed/rekordbox/Stantons Remixed Duplicate/01 Who Are The Warriors - Basskleph Remix.mp3
    //
    // - Checking for files in the search directory which don't exist in rekordbox...
    // - Checking for files with a path too long for rekordbox...
    //
    // ************* Results Summary *************
    // Total tracks in collection: 17
    // Tracks OK: 13
    // Tracks repaired: 2
    // Tracks with multiple matches: 1
    // Tracks with missing files: 1
    // Tracks with path too long: 1
    // Tracks on disk but not in rekordbox: 2
    //
    pub fn run(&mut self) {
        self.check_paths();
        self.check_not_imported();
        self.check_duplicates();
        self.check_relocated();
    }

    /// Check that all tracks locations in the XML file actually exist.
    fn check_paths(&mut self) {
        for t in self.xml.tracks().iter() {
            if !check_file_exists(&t.location) {
                self.report.missing.push(t.location.clone());
            }
        }
    }

    /// Check that all the files into a given path actually exist in the XML file.
    fn check_not_imported(&mut self) {
        let xml_tracks: Vec<String> = self
            .xml
            .tracks()
            .iter()
            .map(|t| get_file_path(&t.location.clone()).unwrap())
            .collect();

        self.report.not_imported = paths_diff(&self.folder_contents, &xml_tracks);
    }

    fn check_duplicates(&mut self) {
        for f in self.xml.tracks().iter() {
            let file_path = get_file_path(&f.location).unwrap();

            self.check_file_duplicates(file_path);
        }
    }

    fn check_relocated(&mut self) {
        for f in self.report.missing.iter() {
            let missing_file = Path::new(&f).file_name().unwrap().to_str().unwrap();

            for n in self.report.not_imported.iter() {
                let not_imported_file = Path::new(&n).file_name().unwrap().to_str().unwrap();
                if missing_file == not_imported_file {
                    self.report
                        .relocated
                        .entry(missing_file.to_string())
                        .or_default()
                        .push(not_imported_file.to_string());
                }
            }
        }
    }

    fn check_file_duplicates(&mut self, file_path: String) {
        let main_file = Path::new(&file_path).file_name().unwrap().to_str().unwrap();

        for file in self.folder_contents.iter() {
            if file_path == *file {
                continue;
            }

            let content_file = Path::new(&file).file_name().unwrap().to_str().unwrap();
            if main_file == content_file {
                self.report
                    .duplicates
                    .entry(file_path.to_string())
                    .or_default()
                    .push(file.to_string());
            }
        }
    }
}

#[derive(Default, Debug)]
pub struct AnalyzerReport {
    pub missing: Vec<String>,
    pub not_imported: Vec<String>,
    pub duplicates: HashMap<String, Vec<String>>,
    pub relocated: HashMap<String, Vec<String>>,
}

// Utils

fn get_file_path(url: &str) -> Option<String> {
    if let Ok(url) = Url::parse(url) {
        if url.scheme() == "file" {
            if let Some(file_path) = url.to_file_path().ok() {
                return Some(file_path.to_str().unwrap().to_owned());
            }
        }
    }
    None
}

fn check_file_exists(file_url: &str) -> bool {
    if let Some(file_path) = get_file_path(file_url) {
        return Path::new(&file_path).exists();
    }
    false
}

fn list_supported_files(dir_path: &str) -> Vec<String> {
    let dir = Path::new(dir_path);
    if let Ok(entries) = fs::read_dir(dir) {
        let mut files = Vec::new();
        for entry in entries.flatten() {
            if let Some(file_name) = entry.file_name().to_str() {
                if entry.file_type().map(|t| t.is_file()).unwrap_or(false) {
                    if let Some(extension) = Path::new(file_name).extension() {
                        if SUPPORTED_EXTENSIONS.iter().any(|ext| extension == *ext) {
                            // files.push(file_name.to_owned());
                            files.push(entry.path().to_str().unwrap().to_owned());
                        }
                    }
                }
            }
        }
        return files;
    }
    Vec::new()
}

fn paths_diff(paths_a: &[String], paths_b: &[String]) -> Vec<String> {
    let set_a: HashSet<_> = paths_a.iter().collect();
    let set_b: HashSet<_> = paths_b.iter().collect();

    set_a
        .difference(&set_b)
        .into_iter()
        .map(|p| p.to_string())
        .collect()
}
