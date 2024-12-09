use num::Integer;

#[derive(Debug, Clone)]
pub struct Span1D<T: Integer + Copy + TryFrom<usize>>
where
    <T as TryFrom<usize>>::Error: std::fmt::Debug,
{
    pub start: T,
    pub len: usize,
}

impl<T: Integer + Copy + TryFrom<usize>> Span1D<T>
where
    <T as TryFrom<usize>>::Error: std::fmt::Debug,
{
    pub fn new(start: T, len: usize) -> Self
    where
        <T as TryFrom<usize>>::Error: std::fmt::Debug,
    {
        Self { start, len }
    }

    pub fn increment(&mut self) {
        self.len += 1
    }

    pub fn end(&self) -> T {
        self.start + T::try_from(self.len).unwrap()
    }
}
