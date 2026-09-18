use super::Mapper;

pub struct TransformMapper<TFrom, TTo> {
    map: Box<dyn Fn(TFrom) -> TTo + Send + Sync>,
    reverse_map: Box<dyn Fn(TTo) -> TFrom + Send + Sync>,
}

impl<TFrom, TTo> TransformMapper<TFrom, TTo> {
    pub fn new(
        map: impl Fn(TFrom) -> TTo + Send + Sync + 'static,
        reverse_map: impl Fn(TTo) -> TFrom + Send + Sync + 'static,
    ) -> Self {
        Self {
            map: Box::new(map),
            reverse_map: Box::new(reverse_map),
        }
    }
}

impl<TFrom, TTo> Mapper<TFrom, TTo> for TransformMapper<TFrom, TTo> {
    fn map(&self, value: TFrom) -> TTo {
        (self.map)(value)
    }

    fn reverse_map(&self, value: TTo) -> TFrom {
        (self.reverse_map)(value)
    }
}
