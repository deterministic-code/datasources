pub trait Where: Send + Sync {
    fn get_text(&self) -> String;
}
