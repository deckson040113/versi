use chrono::NaiveDate;
use serde::Deserialize;
use std::collections::HashMap;

const SCHEDULE_URL: &str = "https://raw.githubusercontent.com/nodejs/Release/main/schedule.json";

#[derive(Debug, Clone, Deserialize)]
pub struct VersionSchedule {
    pub start: String,
    #[serde(default)]
    pub lts: Option<String>,
    #[serde(default)]
    pub maintenance: Option<String>,
    pub end: String,
    #[serde(default)]
    pub codename: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ReleaseSchedule {
    pub versions: HashMap<u32, VersionSchedule>,
}

impl ReleaseSchedule {
    pub fn is_active(&self, major: u32) -> bool {
        let Some(schedule) = self.versions.get(&major) else {
            return major >= 18;
        };

        let Ok(end_date) = NaiveDate::parse_from_str(&schedule.end, "%Y-%m-%d") else {
            return true;
        };

        let today = chrono::Utc::now().date_naive();
        end_date > today
    }

    pub fn is_lts(&self, major: u32) -> bool {
        self.versions
            .get(&major)
            .map(|s| s.lts.is_some() || s.codename.is_some())
            .unwrap_or(false)
    }

    pub fn codename(&self, major: u32) -> Option<&str> {
        self.versions
            .get(&major)
            .and_then(|s| s.codename.as_deref())
    }

    pub fn active_versions(&self) -> Vec<u32> {
        self.versions
            .keys()
            .filter(|&&major| self.is_active(major))
            .copied()
            .collect()
    }

    pub fn active_lts_versions(&self) -> Vec<u32> {
        self.versions
            .keys()
            .filter(|&&major| self.is_active(major) && self.is_lts(major))
            .copied()
            .collect()
    }
}

pub async fn fetch_release_schedule() -> Result<ReleaseSchedule, String> {
    let response = reqwest::get(SCHEDULE_URL)
        .await
        .map_err(|e| format!("Failed to fetch release schedule: {}", e))?;

    let raw: HashMap<String, VersionSchedule> = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse release schedule: {}", e))?;

    let versions: HashMap<u32, VersionSchedule> = raw
        .into_iter()
        .filter_map(|(key, value)| {
            let major = key.trim_start_matches('v').parse().ok()?;
            Some((major, value))
        })
        .collect();

    Ok(ReleaseSchedule { versions })
}
