use lazy_static::lazy_static;
use std::io::{self, prelude::*, BufReader};
use regex::Regex;
mod subs;

lazy_static! {
    static ref TIMESTAMP_PATTERN: Regex = Regex::new(r"^((\d\d:)?\d\d:\d\d\.\d+\s-->\s)").unwrap();
}

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = std::env::args().collect();
}

fn parse_vtt(vtt_file: &std::fs::File) -> Vec<subs::SubTrack> {
    let reader = BufReader::new(vtt_file);
    let mut timestamps: Option<(String, String)> = None;
    let mut subtitles: Vec<String> = Vec::new();
    let mut tracks = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(line) => {
                if TIMESTAMP_PATTERN.is_match(&line) {
                    timestamps = Some(parse_timestamps(&line)); 
                }
                else if !timestamps.is_none() && line != "" {
                    subtitles.push(line);
                } else if !timestamps.is_none() && line == "" {
                    let stamps = timestamps.clone().unwrap();
                    tracks.push(
                        subs::SubTrack{
                            time_start: stamps.0.clone(),
                            time_end: stamps.1.clone(),
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

fn parse_timestamps(timestamp: &str) -> (String, String) {
    let pieces: Vec<&str> = timestamp.split("-->").collect();
    (String::from(pieces[0].trim()), String::from(pieces[1].trim()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_vtt_test() {
        let expected1 = subs::SubTrack {
            time_start: String::from("00:01:14.815"),
            time_end: String::from("00:01:18.114"),
            subtitles: String::from("- What?\n- Where are we now?")
        };
        let expected2 = subs::SubTrack {
            time_start: String::from("00:01:18.171"),
            time_end: String::from("00:01:20.991"),
            subtitles: String::from("- This is big bat country.")
        };
        let test_file = std::fs::File::open("./src/tests/multi_line_cues.vtt")
                                        .expect("ruh roh, couldn't find test file");

        let parsed = parse_vtt(&test_file);

        assert_eq!(expected1.time_start, parsed[0].time_start);
        assert_eq!(expected1.time_end, parsed[0].time_end);
        assert_eq!(expected1.subtitles, parsed[0].subtitles);

        assert_eq!(expected2.time_start, parsed[1].time_start);
        assert_eq!(expected2.time_end, parsed[1].time_end);
        assert_eq!(expected2.subtitles, parsed[1].subtitles);
    }
}
