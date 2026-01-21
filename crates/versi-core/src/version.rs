use versi_backend::{InstalledVersion, RemoteVersion};

pub fn parse_installed_versions(output: &str) -> Vec<InstalledVersion> {
    output
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() {
                return None;
            }

            if line == "system" || line == "* system" {
                return None;
            }

            let is_default = line.contains("default");

            let version_str = line.split_whitespace().find(|s| s.starts_with('v'))?;

            let version = version_str.parse().ok()?;

            Some(InstalledVersion {
                version,
                is_default,
                lts_codename: None,
                install_date: None,
                disk_size: None,
            })
        })
        .collect()
}

pub fn parse_remote_versions(output: &str) -> Vec<RemoteVersion> {
    output
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() {
                return None;
            }

            let parts: Vec<&str> = line.splitn(2, ' ').collect();
            let version_str = parts[0].trim();
            let version = version_str.parse().ok()?;

            let lts_codename = if parts.len() > 1 {
                let rest = parts[1].trim();
                if rest.starts_with('(') && rest.ends_with(')') {
                    Some(rest[1..rest.len() - 1].to_string())
                } else {
                    None
                }
            } else {
                None
            };

            Some(RemoteVersion {
                version,
                lts_codename,
                is_latest: false,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_installed_versions() {
        let output = "* v20.11.0 default\nv18.19.1\nv16.20.2";
        let versions = parse_installed_versions(output);
        assert_eq!(versions.len(), 3);
        assert!(versions[0].is_default);
    }
}
