pub trait Iterable {
    fn iter<'a>(&'a self) -> std::vec::IntoIter<(&'static str, &'a dyn std::any::Any)>;
    fn iter_mut<'a>(&'a mut self) -> std::vec::IntoIter<(&'static str, &'a mut dyn std::any::Any)>;
}
