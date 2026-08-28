pub trait ConvertSchema {
    fn schema() -> Vec<(&'static str, &'static str)>;
}

pub struct Cells;

impl ConvertSchema for Cells {
    fn schema() -> Vec<(&'static str, &'static str)> {
        Vec::new()
    }
}
