#[derive(new, Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub struct TagDefinition {
    pub name: String,
    pub title: String,
    pub style: u8,
}