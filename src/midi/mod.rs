use crate::midi::track_name::MoonTrackName;
use midly::Track;
use midly::TrackEventKind::Meta;
use std::str::FromStr;

pub mod track_name;

pub fn get_track_name(track: &Track) -> Option<MoonTrackName> {
    track
        .iter()
        .take_while(|event| event.delta.as_int() == 0)
        .find_map(|event| {
            if let Meta(midly::MetaMessage::TrackName(track_name)) = event.kind {
                std::str::from_utf8(track_name).ok()
            } else {
                // TODO: still don't know how to track the tempo. Maybe I should avoid the idea of having more keys and stick on what YARG parsed already.
                Some("")
            }
        })
        .and_then(|name| MoonTrackName::from_str(name).ok())
}
