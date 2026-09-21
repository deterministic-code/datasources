pub trait Text: Send + Sync {
    fn get_text(&self) -> String;
}
