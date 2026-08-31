use super::IMapper;

pub struct TransformMapper<TFrom, TTo> {
    map: Box<dyn Fn(TFrom) -> TTo>,
    reverse_map: Box<dyn Fn(TTo) -> TFrom>,
}

impl<TFrom, TTo> TransformMapper<TFrom, TTo> {
    pub fn new(
        map: impl Fn(TFrom) -> TTo + 'static,
        reverse_map: impl Fn(TTo) -> TFrom + 'static,
    ) -> Self {
        Self {
            map: Box::new(map),
            reverse_map: Box::new(reverse_map),
        }
    }
}

impl<TFrom, TTo> IMapper<TFrom, TTo> for TransformMapper<TFrom, TTo> {
    fn map(&self, value: TFrom) -> TTo {
        (self.map)(value)
    }

    fn reverse_map(&self, value: TTo) -> TFrom {
        (self.reverse_map)(value)
    }
}
