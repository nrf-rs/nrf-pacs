#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    set: [Set; 256],
}
impl RegisterBlock {
    #[doc = "0x00..0x2000 - Unspecified"]
    #[inline(always)]
    pub const fn set(&self, n: usize) -> &Set {
        &self.set[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x2000 - Unspecified"]
    #[inline(always)]
    pub fn set_iter(&self) -> impl Iterator<Item = &Set> {
        self.set.iter()
    }
}
#[doc = "Unspecified"]
pub use self::set::Set;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod set;
