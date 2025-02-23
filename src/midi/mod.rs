use crate::midi::track_name::TrackName;
use midly::Track;
use midly::TrackEventKind::Meta;

pub mod track_name;

pub fn get_track_name(track: &Track) -> Option<TrackName> {
    track
        .iter()
        .take_while(|event| event.delta.as_int() == 0)
        .find_map(|event| {
            if let Meta(midly::MetaMessage::TrackName(track_name)) = event.kind {
                std::str::from_utf8(track_name).ok()
            } else {
                None
            }
        })
        .and_then(|name| TrackName::from_str(name).ok())
}
