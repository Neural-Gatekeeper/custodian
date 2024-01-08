pub trait ValueObject<T: PartialEq<T> + PartialEq<Self>> {
    fn value(&self) -> T;

    fn equals(&self, other: &Self) -> bool {
        self.value().eq(other)
    }

    fn equals_value(&self, value: T) -> bool {
        self.value().eq(&value)
    }
}