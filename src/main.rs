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
                    tracks.push(
                        subs::SubTrack{
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
            timestamps: (String::from("00:01:14.815"), String::from("00:01:18.114")),
            subtitles: String::from("- What?\n- Where are we now?")
        };
        let expected2 = subs::SubTrack {
            timestamps: (String::from("00:01:18.171"), String::from("00:01:20.991")),
            subtitles: String::from("- This is big bat country.")
        };
        let test_file = std::fs::File::open("./src/tests/multi_line_cues.vtt")
                                        .expect("ruh roh, couldn't find test file");

        let parsed = parse_vtt(&test_file);

        assert_eq!(expected1.timestamps, parsed[0].timestamps);
        assert_eq!(expected1.subtitles, parsed[0].subtitles);

        assert_eq!(expected2.timestamps, parsed[1].timestamps);
        assert_eq!(expected2.subtitles, parsed[1].subtitles);
    }
}
