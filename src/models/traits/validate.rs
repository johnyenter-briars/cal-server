pub trait Validatable {
    fn time_is_populated(&self) -> (bool, String);
    fn end_after_start(&self) -> (bool, String);
}