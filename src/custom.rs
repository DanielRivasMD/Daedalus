////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct Replacement {
    pub old: String,
    pub new: String,
    pub mode: String, // "token" or "line"
}

////////////////////////////////////////////////////////////////////////////////////////////////////

impl std::str::FromStr for Replacement {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (old, rest) = s.split_once('=').ok_or("invalid replace pair")?;
        let (mode, new_val) = if let Some(stripped) = rest.strip_suffix(":line") {
            ("line", stripped)
        } else if let Some(stripped) = rest.strip_suffix(":token") {
            ("token", stripped)
        } else {
            ("token", rest)
        };

        Ok(Replacement {
            old: old.to_string(),
            new: new_val.to_string(),
            mode: mode.to_string(),
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
