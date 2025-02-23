use crate::moonsong::{Moonsong, TempoMap, TrackEvent};
use midly::{MetaMessage, Track, TrackEventKind};

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
                    moonsong.tempo_changes.push(TempoMap {
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
