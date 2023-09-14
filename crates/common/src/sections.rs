pub const SECTIONS: [&str; 5] = ["fresh", "pantry", "protein", "dairy", "freezer"];

pub struct Section(String);

impl Section {
    pub fn new(sec: impl Into<String>) -> Self {
        Self(sec.into())
    }
}
