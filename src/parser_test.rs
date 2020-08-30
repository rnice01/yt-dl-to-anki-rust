use crate::subs::{SubTrack};
use crate::parser::{Parser, VttParser};

#[test]
fn parse_vtt_test() {
    let expected1 = SubTrack {
        timestamps: (String::from("00:01:14.815"), String::from("00:01:18.114")),
        subtitles: String::from("- What?\n- Where are we now?")
    };
    let expected2 = SubTrack {
        timestamps: (String::from("00:01:18.171"), String::from("00:01:20.991")),
        subtitles: String::from("- This is big bat country.")
    };
    let test_file = std::fs::File::open("./tests/multi_line_cues.vtt")
                                .expect("ruh roh, couldn't find test file");

    let parser: VttParser = Parser::new();
    let parsed = parser.parse(&test_file);

    assert_eq!(expected1.timestamps, parsed[0].timestamps);
    assert_eq!(expected1.subtitles, parsed[0].subtitles);

    assert_eq!(expected2.timestamps, parsed[1].timestamps);
    assert_eq!(expected2.subtitles, parsed[1].subtitles);
}