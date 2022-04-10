#[repr(align(4), C)]
pub struct Align4<T>(T);

impl<T> core::ops::Deref for Align4<T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &T {
        &self.0
    }
}
