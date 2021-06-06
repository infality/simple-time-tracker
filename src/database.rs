use rusqlite::{params, Connection};
use std::collections::HashMap;

use crate::{SimpleTimeTracker, TrackedTime};

pub const TIME_KEY: &str = "time";
pub const DARKMODE_KEY: &str = "darkmode";

pub fn load_states() -> HashMap<String, i32> {
    let db = Connection::open("simple_time_tracker.sqlite").unwrap();

    db.execute(
        "CREATE TABLE IF NOT EXISTS States (
                Key TEXT PRIMARY KEY,
                Value INTEGER NOT NULL
        )",
        [],
    )
    .unwrap();

    let mut stmt = db.prepare("SELECT Key, Value FROM States").unwrap();
    let mut rows = stmt.query([]).unwrap();

    let mut states: HashMap<String, i32> = HashMap::new();
    while let Some(row) = rows.next().unwrap() {
        states.insert(row.get(0).unwrap(), row.get(1).unwrap());
    }
    return states;
}

pub fn load_tracked_times() -> Vec<TrackedTime> {
    let db = Connection::open("simple_time_tracker.sqlite").unwrap();

    db.execute(
        "CREATE TABLE IF NOT EXISTS TrackedTimes (
                ID INTEGER PRIMARY KEY,
                Seconds INTEGER NOT NULL,
                Description TEXT NOT NULL
        )",
        [],
    )
    .unwrap();

    let mut stmt = db
        .prepare("SELECT Seconds, Description FROM TrackedTimes")
        .unwrap();
    let mut rows = stmt.query([]).unwrap();

    let mut tracked_times = Vec::new();
    while let Some(row) = rows.next().unwrap() {
        tracked_times.push(TrackedTime::new(
            chrono::Duration::seconds(row.get(0).unwrap()),
            row.get(1).unwrap(),
        ));
    }
    return tracked_times;
}

impl SimpleTimeTracker {
    pub fn store_state(&self) {
        let db = Connection::open("simple_time_tracker.sqlite").unwrap();

        db.execute("DELETE FROM States", []).unwrap();
        let mut stmt = db
            .prepare("INSERT INTO States (Key, Value) VALUES (?1, ?2)")
            .unwrap();

        stmt.execute(params![TIME_KEY, self.get_current_duration().num_seconds()])
            .unwrap();

        stmt.execute(params![DARKMODE_KEY, self.is_dark_mode as i32])
            .unwrap();
    }

    pub fn store_tracked_times(&self) {
        let db = Connection::open("simple_time_tracker.sqlite").unwrap();

        db.execute("DELETE FROM TrackedTimes", []).unwrap();
        let mut stmt = db
            .prepare("INSERT INTO TrackedTimes (Seconds, Description) VALUES (?1, ?2)")
            .unwrap();
        for tracked_time in self.tracked_times.iter() {
            stmt.execute(params![
                tracked_time.duration.num_seconds(),
                tracked_time.description
            ])
            .unwrap();
        }
    }
}
