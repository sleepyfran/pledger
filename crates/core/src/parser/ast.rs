type Year = u32;

/// Defines the internal representation of a journal once parsed from a file.
#[derive(Debug)]
pub struct Journal {
    /// Optional year that applies to the whole journal.
    pub year: Option<Year>,
}
