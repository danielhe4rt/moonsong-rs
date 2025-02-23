use crate::midi::track_name::MoonTrackName;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum MoonDifficulty {
    Easy,
    Medium,
    Hard,
    Expert,
    UNDEFINED,
}

impl MoonDifficulty {
    pub fn get_difficulty(key_value: &u8) -> Self {
        match key_value {
            60..=71 => MoonDifficulty::Easy,
            72..=83 => MoonDifficulty::Medium,
            84..=95 => MoonDifficulty::Hard,
            96..=107 => MoonDifficulty::Expert,
            _ => MoonDifficulty::UNDEFINED,
        }
    }
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
pub struct MoonTempo {
    pub bpm: f32,   //
    pub time: u32,  // in delta | delta += event.delta.as_int();
    pub delta: u32, // in delta
}

#[derive(Debug)]
pub struct Moonsong {
    pub name: String,                              // name of the song
    pub resolution: u16,                           // ticks per beat
    pub tracks: HashMap<MoonTrackName, MoonTrack>, // Instruments w/ notes
    // TODO: ask related to Events vs Sections
    pub events: Vec<TrackEvent>,       // Events in the song
    pub tempo_changes: Vec<MoonTempo>, // BPM changes
    pub time_in_seconds: u32,          // Song length in seconds
}

impl Moonsong {
    pub fn overview(&self) {
        println!("[Moonsong] Song Overview: {}", self.name);
        println!(" -> [Moonsong] Resolution: {}", self.resolution);
        println!(" -> [Moonsong] Time in Seconds: {}", self.time_in_seconds);
        println!(" -> [Moonsong] Tempo Changes: {}", self.tempo_changes.len());
        println!(" -> [Moonsong] Events: {}", self.events.len());
        for (name, track) in self.tracks.iter() {
            track.overview();
        }
    }
}

impl Moonsong {
    pub fn new(resolution: u16) -> Self {
        Self {
            name: String::new(),
            resolution,
            tracks: HashMap::new(),
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

    pub fn set_time_in_seconds(&mut self) {
        let mut seconds = 0f32;
        let mut last_time = 0;

        // Find the last event time across all tracks and events
        for (_, track) in self.tracks.iter() {
            for (_, lane) in track.lanes.iter() {
                match lane.notes.last() {
                    Some(note) => {
                        last_time = last_time.max(note.time);
                    }
                    None => {}
                }
            }
        }
        println!("Last Time: {}", last_time);
        for event in &self.events {
            last_time = last_time.max(event.time);
        }

        // If no tempo changes, use default tempo
        if self.tempo_changes.is_empty() {
            println!("No tempo changes found, using default tempo");
            let seconds_per_beat = 60.0 / 120.0; // Default 120 BPM
            let total_beats = last_time as f32 / self.resolution as f32;
            seconds = total_beats * seconds_per_beat;
            self.time_in_seconds = seconds.ceil() as u32;
            return;
        }

        // Sort tempo changes by time for easier lookup
        self.tempo_changes.sort_by_key(|tempo| tempo.time);

        // Handle time before first tempo change
        if self.tempo_changes[0].time > 0 {
            let delta_ticks = self.tempo_changes[0].time;
            let seconds_per_beat = 60.0 / 120.0; // Default 120 BPM
            let delta_beats = delta_ticks as f32 / self.resolution as f32;
            seconds += delta_beats * seconds_per_beat;
        }

        // Calculate time using tempo changes
        for i in 0..self.tempo_changes.len() {
            let current_tempo = self.tempo_changes[i].bpm;
            let current_time = self.tempo_changes[i].time;

            let next_time = if i + 1 < self.tempo_changes.len() {
                self.tempo_changes[i + 1].time
            } else {
                last_time
            };

            if next_time > current_time {
                // Only calculate if there's a positive time difference
                let delta_ticks = next_time - current_time;
                let seconds_per_beat = 60.0 / current_tempo;
                let delta_beats = delta_ticks as f32 / self.resolution as f32;
                seconds += delta_beats * seconds_per_beat;
            }
        }

        self.time_in_seconds = seconds.ceil() as u32;
    }
}

#[derive(Debug)]
pub struct TrackEvent {
    pub name: String,
    pub time: u32,
    pub delta: u32,
}

#[derive(Debug)]
struct MoonNote {
    key: u8,
    note_state: bool, // NOTE ON or NOTE OFF
    time: u32,        // in delta
    delta: u32,       // in delta
}
#[derive(Debug)]
pub struct MoonLane {
    notes: Vec<MoonNote>,
}

#[derive(Debug)]
pub struct MoonTrack {
    name: MoonTrackName,
    pub lanes: HashMap<MoonDifficulty, MoonLane>,
}

impl MoonTrack {
    pub(crate) fn get_difficulty_lane(&mut self, difficulty: &MoonDifficulty) -> &MoonLane {
        println!("Getting difficulty lane for {:?}", difficulty);
        self.lanes.get(difficulty).unwrap()
    }

    pub fn overview(&self) {
        println!("[Moontrack] Track Overview: {:?} ", self.name);
        for (difficulty, lane) in self.lanes.iter() {
            println!(" -> [Lane] Difficulty: {:?}", difficulty);
            lane.overview();
        }
    }
}

impl MoonLane {
    pub(crate) fn add_note(&mut self, key: u8, note_state: bool, delta: u32, time: u32) {
        self.notes.push(MoonNote {
            key,
            note_state,
            time,
            delta,
        });
    }

    pub fn count_on_notes(&self) -> usize {
        self.notes.iter().filter(|note| note.note_state).count()
    }

    pub fn count_off_notes(&self) -> usize {
        self.notes.iter().filter(|note| !note.note_state).count()
    }

    pub fn count_notes(&self) -> usize {
        self.notes.len()
    }

    pub fn overview(&self) {
        println!("   -> [Lane] Total Notes: {}", self.count_notes());
        println!(
            "   -> [Lane] Notes: ON {}/{} OFF",
            self.count_on_notes(),
            self.count_off_notes()
        );

        // for note in self.notes.iter() {
        //     println!(
        //         "     -> [Note] Key: {} | State: {} | Time: {} | Delta: {}",
        //         note.key, note.note_state, note.time, note.delta
        //     );
        // }

        println!(" ----------------- ");
    }
}

impl MoonTrack {
    pub fn new(name: MoonTrackName) -> Self {
        let mut lanes = HashMap::new();
        lanes.insert(MoonDifficulty::Easy, MoonLane { notes: Vec::new() });
        lanes.insert(MoonDifficulty::Medium, MoonLane { notes: Vec::new() });
        lanes.insert(MoonDifficulty::Hard, MoonLane { notes: Vec::new() });
        lanes.insert(MoonDifficulty::Expert, MoonLane { notes: Vec::new() });
        lanes.insert(MoonDifficulty::UNDEFINED, MoonLane { notes: Vec::new() });

        Self { name, lanes }
    }
}
