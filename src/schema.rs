#[derive(Debug)]
pub struct Placeholder {
    source: String,
    label: String,
    version: Option<String>
}

trait Lookup {
    fn lookup(&self) -> String
}