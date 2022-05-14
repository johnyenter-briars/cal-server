use core::fmt;

struct Optional64(Option<i64>);

impl fmt::Display for Optional64 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.0.is_some() {
            write!(f, "{}", self.0.unwrap())
        } else {
            write!(f, "{}", "")
        }
    }
}