pub trait Query: Send + Sync {
    fn get_text(&self) -> String;
}
