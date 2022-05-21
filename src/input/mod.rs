pub mod null;

pub trait Input {
    fn argument(&self, string: &str) -> Result<String, String>;

    fn to_iter(&self) -> Box<dyn Iterator<Item = String>>;
}