use crate::subs::{SubTrack};
use std::io::{prelude::*, BufReader};
use lazy_static::lazy_static;
use regex::Regex;

pub trait Parser {
  fn new() -> Self;
  fn parse(&self, file: &std::fs::File) -> Vec<SubTrack>;
}

pub struct VttParser {}

lazy_static! {
    static ref TIMESTAMP_PATTERN: Regex = Regex::new(r"^((\d\d:)?\d\d:\d\d\.\d+\s-->\s)").unwrap();
}

impl VttParser {
  fn parse_timestamps(timestamp: &str) -> (String, String) {
      let pieces: Vec<&str> = timestamp.split("-->").collect();
      (String::from(pieces[0].trim()), String::from(pieces[1].trim()))
  }
}

impl Parser for VttParser {
  fn new() -> Self {
    VttParser {}
  }

  fn parse(&self, file: &std::fs::File) -> Vec<SubTrack> {
    let reader = BufReader::new(file);
    let mut timestamps: Option<(String, String)> = None;
    let mut subtitles: Vec<String> = Vec::new();
    let mut tracks = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(line) => {
                if TIMESTAMP_PATTERN.is_match(&line) {
                    timestamps = Some(Self::parse_timestamps(&line)); 
                }
                else if !timestamps.is_none() && line != "" {
                    subtitles.push(line);
                } else if !timestamps.is_none() && line == "" {
                    tracks.push(
                        SubTrack{
                            timestamps: timestamps.unwrap(),
                            subtitles: subtitles.clone().join("\n")
                        }
                    );
                    subtitles.clear();
                    timestamps = None;
                }
            },
            Err(err) => println!("{}", err)
        }
    };

    tracks
  }
}
