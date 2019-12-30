pub struct True;
pub struct False;

impl True {
    pub const fn value(self) -> bool {
        true
    }
}

impl False {
    pub const fn value(self) -> bool {
        false
    }
}
