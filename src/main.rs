#![feature(str_as_str)]

mod midi;

use midly::Smf;
use regex::Regex;
use rustly_lane::midi::get_track_name;
use rustly_lane::midi::track_name::MoonTrackName;
use rustly_lane::moonsong::{MoonDifficulty, Moonsong};
use rustly_lane::parsers::{parse_events, parse_notes, parse_tempo};
use std::fmt::Debug;
use std::str::FromStr;

fn main() {
    let smf = Smf::parse(include_bytes!("../assets/eye-of-the-tiger.mid")).unwrap();

    let resolution = get_resolution(&smf);
    let mut moonsong = Moonsong::new(resolution);

    for (i, track) in smf.tracks.iter().enumerate() {
        let name = get_track_name(&track);

        let Some(name) = name else {
            // TODO: Track zero still unknown for me. Remember to ask Nathanator for help.
            // TODO: Track Zero contains the name and tempo/bpm changes of the song
            println!(
                "[EVENT NOT FOUND] Track {} has no name. Please investigate.",
                i
            );

            continue;
        };

        match name {
            MoonTrackName::Events => {
                parse_events(&mut moonsong, track);
            }
            MoonTrackName::Meta => {
                parse_tempo(&mut moonsong, track);
            }
            _ => {
                parse_notes(&mut moonsong, name, track);
            }
        };
    }

    moonsong.set_time_in_seconds();
    moonsong.overview();

}

fn get_resolution(smf: &Smf) -> u16 {
    let raw_resolution = format!("{:?}", smf.header.clone());
    let re = Regex::new(r"Metrical\(u15\((\d+)\)\)").unwrap();

    let resolution = if let Some(captures) = re.captures(raw_resolution.as_str()) {
        if let Some(timing_value) = captures.get(1) {
            timing_value.as_str().parse::<u16>().unwrap()
        } else {
            0u16
        }
    } else {
        0u16
    };
    resolution
}
