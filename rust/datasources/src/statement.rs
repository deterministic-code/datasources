pub trait Statement: Send + Sync {
    fn get_text(&self) -> String;
}
