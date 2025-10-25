#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    unused: Unused,
}
impl RegisterBlock {
    #[doc = "0x00 - Unused."]
    #[inline(always)]
    pub const fn unused(&self) -> &Unused {
        &self.unused
    }
}
#[doc = "UNUSED (r) register accessor: Unused.\n\nYou can [`read`](crate::Reg::read) this register and get [`unused::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@unused`] module"]
#[doc(alias = "UNUSED")]
pub type Unused = crate::Reg<unused::UnusedSpec>;
#[doc = "Unused."]
pub mod unused;
