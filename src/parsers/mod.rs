use midly::{MetaMessage, Track, TrackEventKind};
use crate::moonsong::{Moonsong, TrackEvent};

pub fn parse_events(moonsong: &mut Moonsong, track: &Track) {
    let mut delta = 0;
    for event in track.iter() {

        match event.kind {
            TrackEventKind::Meta(meta) => {
                match meta {
                    MetaMessage::Text(text) => {
                        moonsong.add_event(TrackEvent {
                            name: std::str::from_utf8(text).unwrap().to_string(),
                            time: delta,
                            delta: event.delta.as_int(),
                        });
                        println!("[T: {}]Text: {:?}", event.delta, std::str::from_utf8(text).unwrap());
                    }
                    MetaMessage::EndOfTrack => {
                        moonsong.add_event(TrackEvent {
                            name: "End of Track".to_string(),
                            time: delta,
                            delta: event.delta.as_int(),
                        });
                        println!("[T: {}]End of Track", event.delta);
                    }
                    _ => {
                        println!("[T: {}]Meta: {:?}", event.delta, meta);
                    }
                }
            }
            _ => {}
        }

        delta += event.delta.as_int();
    }

    moonsong.set_time_in_seconds(delta);
}