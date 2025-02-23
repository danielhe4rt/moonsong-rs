use crate::midi::track_name::MoonTrackName;
use crate::moonsong::{MoonDifficulty, MoonTempo, MoonTrack, Moonsong, TrackEvent};
use midly::{MetaMessage, MidiMessage, Track, TrackEventKind};

pub fn parse_debug_events(track: &Track) {
    for event in track.iter() {
        println!("{:?}", event);
    }
}

pub fn parse_tempo(moonsong: &mut Moonsong, track: &Track) {
    let mut delta = 0;
    println!("[Tempo] Parsing {} tempo changes...", track.len());

    for event in track.iter() {
        match event.kind {
            TrackEventKind::Meta(meta) => match meta {
                MetaMessage::Tempo(tempo) => {
                    let bpm = 60_000_000.0 / tempo.as_int() as f32;
                    moonsong.tempo_changes.push(MoonTempo {
                        bpm,
                        time: delta,
                        delta: event.delta.as_int(),
                    });
                    println!(
                        " -> [Tempo][Delta: {}] Tempo: {} {}",
                        event.delta, delta, bpm
                    );
                }
                _ => {}
            },
            _ => {
                println!(" -> [Tempo][Delta: {}]: {:?}", event.delta, event.kind);
            }
        }

        delta += event.delta.as_int();
    }
}

pub fn parse_events(moonsong: &mut Moonsong, track: &Track) {
    let mut delta = 0;
    println!("[Events] Parsing {} events...", track.len());

    for event in track.iter() {
        match event.kind {
            TrackEventKind::Meta(meta) => match meta {
                MetaMessage::Text(text) => {
                    moonsong.add_event(TrackEvent {
                        name: std::str::from_utf8(text).unwrap().to_string(),
                        time: delta,
                        delta: event.delta.as_int(),
                    });
                    println!(
                        " -> [Events][Delta: {}] Text: {:?}",
                        event.delta,
                        std::str::from_utf8(text).unwrap()
                    );
                }
                MetaMessage::EndOfTrack => {
                    moonsong.add_event(TrackEvent {
                        name: "End of Track".to_string(),
                        time: delta,
                        delta: event.delta.as_int(),
                    });
                    println!(" -> [Events][Delta: {}] End of Track", event.delta);
                }
                _ => {
                    println!(" -> [Events][Delta: {}] Meta: {:?}", event.delta, meta);
                }
            },
            _ => {}
        }

        delta += event.delta.as_int();
    }

    println!("[EVENTS] Done parsing events.");
}

pub fn parse_notes(moonsong: &mut Moonsong, track_name: MoonTrackName, track: &Track) {
    let mut absolute_time = 0;

    let mut moon_track = MoonTrack::new(track_name);
    for event in track.iter() {
        absolute_time += event.delta.as_int();

        match event.kind {
            TrackEventKind::Midi {
                channel: _,
                message,
            } => match message {
                MidiMessage::NoteOn { key, vel } | MidiMessage::NoteOff { key, vel } => {
                    let note_state = if message.eq(&MidiMessage::NoteOn { key, vel }) {
                        true
                    } else {
                        false
                    };
                    let key_value = key.as_int();
                    let difficulty = MoonDifficulty::get_difficulty(&key_value);

                    let lane = moon_track.lanes.get_mut(&difficulty).unwrap();
                    lane.add_note(key_value, note_state, event.delta.as_int(), absolute_time);
                }
                _ => {}
            },
            _ => {}
        }
    }

    moonsong.tracks.push(moon_track);
}
