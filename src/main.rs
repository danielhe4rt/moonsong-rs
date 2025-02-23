#![feature(str_as_str)]

mod midi;

use crate::midi::track_name::TrackName;
use midly::MetaMessage::{MidiChannel, TrackName as TrackNameMeta};
use midly::TrackEventKind::Meta;
use midly::num::u7;
use midly::{MidiMessage, Smf, TrackEventKind};
use regex::Regex;
use rustly_lane::midi::get_track_name;
use rustly_lane::moonsong::Moonsong;
use rustly_lane::parsers::{parse_events, parse_tempo};
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
            println!("[EVENT NOT FOUND] Track {} has no name. Please investigate.", i);

            continue;
        };

        //println!("track {} has name {:#?}", i, name);
        println!("{:?}", name);
        match name {
            TrackName::Events => {
                parse_events(&mut moonsong, track);
            }
            TrackName::Meta => {
                parse_tempo(&mut moonsong, track);
            }
            _ => {}
        };

        continue;

        println!("track {} has name {:#?}", i, name);

        println!("track {} has {} events", i, track.len());
        let mut easy_notes: Vec<(u7, u32)> = Vec::new();
        let mut medium_notes: Vec<(u7, u32)> = Vec::new();
        let mut hard_notes: Vec<(u7, u32)> = Vec::new();
        let mut expert_notes: Vec<(u7, u32)> = Vec::new();

        let mut time = 0;

        for event in track.iter() {
            time += event.delta.as_int();

            match event.kind {
                Meta(kind) => match kind {
                    TrackNameMeta(name) => {
                        let track_name = std::str::from_utf8(name).unwrap().as_str();
                        //println!("Track Name: {:?}", track_name);
                    }
                    MidiChannel(channel) => {
                        // println!("Midi Channel: {:?}", channel);
                    }
                    _ => {}
                },
                TrackEventKind::Midi { channel, message } => match message {
                    MidiMessage::NoteOn { key, vel } | MidiMessage::NoteOff { key, vel } => {
                        let key_value = key.as_int();
                        if (60..=71).contains(&key_value) {
                            easy_notes.push((key, time));
                        } else if (72..=83).contains(&key_value) {
                            medium_notes.push((key, time));
                        } else if (84..=95).contains(&key_value) {
                            hard_notes.push((key, time));
                        } else if (96..=107).contains(&key_value) {
                            expert_notes.push((key, time));
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        println!("Easy Notes: {:#?}", easy_notes.len());
        println!("Medium Notes: {:#?}", medium_notes.len());
        println!("Hard Notes: {:#?}", hard_notes.len());
        println!("Expert Notes: {:#?}", expert_notes.len());
        println!("-----------------------------------");
    }

    // println!("Moonsong: {:#?}", moonsong);
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
