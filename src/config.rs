use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

const SOURCE_CODE_FILENAMES_DEFAULT: &str = "main.c";
const DEFAULT_BINARY_OUTPUT_NAME_DEFAULT: &str = "out";
const TEST_FOLDER_NAMES_DEFAULT: &[&str] = &["CZE", "ENG"];
const C_COMPILER_DEFAULT: &str = "g++";
const C_COMPILER_OPTS_DEFAULT: &[&str] = &[
    "std=c++20",
    "-Wall",
    "-pedantic",
    "-Wno-long-long",
    "-O2",
    "-c",
    "-o",
];

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub c_compiler: String,
    pub c_compiler_opts: Vec<String>,
    pub source_code_filenames: Vec<String>,
    pub test_folder_names: Vec<String>,
    pub default_bin_output_name: String,
    pub pipes: Option<HashMap<String, Vec<String>>>,
    pub regions: Option<HashMap<String, Vec<PathBuf>>>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            c_compiler: C_COMPILER_DEFAULT.to_string(),
            c_compiler_opts: C_COMPILER_OPTS_DEFAULT
                .iter()
                .map(|&s| s.to_string())
                .collect(),
            source_code_filenames: vec![SOURCE_CODE_FILENAMES_DEFAULT.to_string()],
            test_folder_names: TEST_FOLDER_NAMES_DEFAULT
                .iter()
                .map(|&s| s.to_string())
                .collect(),
            default_bin_output_name: DEFAULT_BINARY_OUTPUT_NAME_DEFAULT.to_string(),
            pipes: None,
            regions: None,
        }
    }
}

impl Config {
    pub fn load(path: impl AsRef<Path>) -> Result<Self, Box<dyn std::error::Error>> {
        let file = std::fs::File::open(path)?;
        let config = serde_json::from_reader(file)?;
        Ok(config)
    }

    pub fn save(&self, path: impl AsRef<Path>) -> Result<(), Box<dyn std::error::Error>> {
        let file = std::fs::File::create(path)?;
        serde_json::to_writer_pretty(file, self)?;
        Ok(())
    }
}
