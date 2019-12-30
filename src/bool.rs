#[derive(Clone, Copy)]
pub struct True;

#[derive(Clone, Copy)]
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
