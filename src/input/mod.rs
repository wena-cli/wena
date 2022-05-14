pub mod null;

pub trait Input {
    fn argument(&self, string: &str) -> Result<String, String> ;
}
