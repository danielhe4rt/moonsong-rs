use crate::midi::track_name::TrackName;
use midly::num::u7;
use std::collections::HashMap;

#[derive(Debug)]
enum Difficulty {
    Easy,
    Medium,
    Hard,
    Expert,
}

#[derive(Debug)]
enum LaneKey {
    Green,
    Red,
    Yellow,
    Blue,
    Orange,
    Bar,
}

#[derive(Debug)]
pub struct TempoMap {
    pub bpm: f32,   //
    pub time: u32,  // in delta | delta += event.delta.as_int();
    pub delta: u32, // in delta
}

#[derive(Debug)]
pub struct Moonsong {
    pub name: String,       // name of the song
    pub resolution: u16,    // ticks per beat
    pub tracks: Vec<Track>, // Instruments w/ notes
    // TODO: ask related to Events vs Sections
    pub events: Vec<TrackEvent>,      // Events in the song
    pub tempo_changes: Vec<TempoMap>, // BPM changes
    pub time_in_seconds: u32,         // Song length in seconds
}

impl Moonsong {
    pub fn new(resolution: u16) -> Self {
        Self {
            name: String::new(),
            resolution,
            tracks: Vec::new(),
            events: Vec::new(),
            tempo_changes: Vec::new(),
            time_in_seconds: 0,
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn add_event(&mut self, track: TrackEvent) {
        self.events.push(track);
    }

    pub fn set_time_in_seconds(&mut self, delta_ticks: u32) {
        let bpm = 120;
        let seconds_per_beat = 60.0 / bpm as f32;
        let delta_beats = delta_ticks / self.resolution as u32;
        self.time_in_seconds = (delta_beats as f32 * seconds_per_beat) as u32;
    }
}

#[derive(Debug)]
pub struct TrackEvent {
    pub name: String,
    pub time: u32,
    pub delta: u32,
}

#[derive(Debug)]
struct Note {
    key: u7,
    time: u32,
}
#[derive(Debug)]
struct Lane {
    notes: Vec<(u7, u32)>,
}

#[derive(Debug)]
struct Track {
    name: TrackName,
    lanes: HashMap<Difficulty, Lane>,
}
