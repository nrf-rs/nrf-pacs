#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "SET")]
pub struct Set {
    way: [Way; 2],
}
impl Set {
    #[doc = "0x00..0x20 - Unspecified"]
    #[inline(always)]
    pub const fn way(&self, n: usize) -> &Way {
        &self.way[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x20 - Unspecified"]
    #[inline(always)]
    pub fn way_iter(&self) -> impl Iterator<Item = &Way> {
        self.way.iter()
    }
}
#[doc = "Unspecified"]
pub use self::way::Way;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod way;
