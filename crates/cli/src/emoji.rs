use std::fmt::{Display, Formatter, Result};

/// Encapsulates a string that holds an emoji. Doesn't check that the given string is a valid emoji,
/// but ensures that there's enough spaces around to not cause issues when displaying it.
pub struct Emoji(String);

impl Display for Emoji {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.0)
    }
}

/// Returns the assigned emoji for info.
pub fn for_success() -> Emoji {
    Emoji("✅".into())
}

/// Returns the assigned emoji for warnings.
pub fn for_warning() -> Emoji {
    Emoji("⚠️ ".into())
}

/// Returns the assigned emoji for errors.
pub fn for_error() -> Emoji {
    Emoji("❌".into())
}

pub fn for_search() -> Emoji {
    Emoji("🔎".into())
}
