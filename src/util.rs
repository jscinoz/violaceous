// From git2-rs's util module
// XXX: Maybe just depend on git2 and use their module?

pub trait IsNull {
    fn is_ptr_null(&self) -> bool;
}

pub trait Binding : Sized  {
    type Raw;

    // Should be Self::Raw?
    fn from_raw(raw: Self::Raw) -> Self;
    // Consumes self
    fn to_raw(self) -> Self::Raw;

    fn from_raw_opt<T>(raw: T) -> Option<Self>
        where T: Copy + IsNull, Self: Binding<Raw=T>
    {
        if raw.is_ptr_null() {
            None
        } else {
            Some(Self::from_raw(raw))
        }
    }
}
