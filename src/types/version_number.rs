/// Version number type
pub(crate) struct VersionNumber {
    major: u32,
    minor: u32,
    patch: u32,
}

impl VersionNumber {
    /// Create a new version number with all parts set to 0
    pub fn new() -> VersionNumber {
        VersionNumber {
            major: 0,
            minor: 0,
            patch: 0,
        }
    }

    /// Create a new version number from a string
    pub fn from_string(version: &str) -> VersionNumber {
        let mut version_number = VersionNumber::new();
        version_number.parse_from_string(version);
        version_number
    }

    /// Parse a version number from a string and return if it was successful
    pub fn parse_from_string(&mut self, version: &str) -> bool {
        let parts: Vec<&str> = version.split('.').collect();

        if parts.len() != 3 {
            return false;
        }
        // cehck if all parts are numbers
        if parts.iter().any(|part| !part.chars().all(char::is_numeric)) {
            return false;
        }
        self.major = parts[0].parse().unwrap();
        self.minor = parts[1].parse().unwrap();
        self.patch = parts[2].parse().unwrap();
        true
    }

    /// Check if this version number is newer than the other
    pub fn is_newer(&self, other: &VersionNumber) -> bool {
        if self.major > other.major {
            return true;
        }
        if self.major == other.major && self.minor > other.minor {
            return true;
        }
        if self.major == other.major && self.minor == other.minor && self.patch > other.patch {
            return true;
        }
        false
    }

    /// Check if this version number is older than the other
    pub fn is_older(&self, other: &VersionNumber) -> bool {
        if self.major < other.major {
            return true;
        }
        if self.major == other.major && self.minor < other.minor {
            return true;
        }
        if self.major == other.major && self.minor == other.minor && self.patch < other.patch {
            return true;
        }
        false
    }

    pub fn is_major_update(&self, other: &VersionNumber) -> bool {
        if self.major != other.major {
            return true;
        }
        return false;
    }
}
