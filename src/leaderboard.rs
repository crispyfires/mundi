use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fs;
use std::path::PathBuf;

const MAX_ENTRIES: usize = 50;

#[derive(Clone, Serialize, Deserialize)]
pub struct LeaderboardEntry {
    pub name: String,
    pub score: u32,
    pub total: u32,
    pub time_secs: u64,
}

impl LeaderboardEntry {
    fn cmp_rank(&self, other: &Self) -> Ordering {
        other
            .score
            .cmp(&self.score)
            .then(self.time_secs.cmp(&other.time_secs))
    }
}

pub struct Leaderboard {
    pub entries: Vec<LeaderboardEntry>,
}

impl Leaderboard {
    fn file_path(country_id: &str, exercise_id: &str) -> PathBuf {
        glib::user_data_dir()
            .join("mundi")
            .join(format!("{country_id}-{exercise_id}.json"))
    }

    pub fn load(country_id: &str, exercise_id: &str) -> Self {
        let path = Self::file_path(country_id, exercise_id);
        let entries = fs::read_to_string(&path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default();
        Leaderboard { entries }
    }

    pub fn save(&self, country_id: &str, exercise_id: &str) {
        let path = Self::file_path(country_id, exercise_id);
        if let Some(dir) = path.parent() {
            let _ = fs::create_dir_all(dir);
        }
        if let Ok(json) = serde_json::to_string_pretty(&self.entries) {
            let _ = fs::write(&path, json);
        }
    }

    pub fn qualifies(&self, score: u32, total: u32, time_secs: u64) -> bool {
        if self.entries.len() < MAX_ENTRIES {
            return true;
        }
        let candidate = LeaderboardEntry {
            name: String::new(),
            score,
            total,
            time_secs,
        };
        self.entries
            .last()
            .map(|worst| candidate.cmp_rank(worst) == Ordering::Less)
            .unwrap_or(true)
    }

    pub fn insert(&mut self, entry: LeaderboardEntry) -> usize {
        let rank = self
            .entries
            .iter()
            .position(|e| entry.cmp_rank(e) == Ordering::Less)
            .unwrap_or(self.entries.len());
        self.entries.insert(rank, entry);
        self.entries.truncate(MAX_ENTRIES);
        rank
    }
}
