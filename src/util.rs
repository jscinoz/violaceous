// From git2-rs's util module
// XXX: Maybe just depend on git2 and use their module?

pub trait IsNull {
    fn is_ptr_null(&self) -> bool;
}

pub trait Binding : Sized  {
    type NativeType;

    // Should be Self::NativeType?
    fn from_native(native: Self::NativeType) -> Self;
    fn to_native(&self) -> Self::NativeType;

    fn from_native_opt<T>(native: T) -> Option<Self>
        where T: Copy + IsNull, Self: Binding<NativeType=T>
    {
        if native.is_ptr_null() {
            None
        } else {
            Some(Self::from_native(native))
        }
    }
}
