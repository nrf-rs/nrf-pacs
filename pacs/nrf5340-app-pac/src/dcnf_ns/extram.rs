#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "EXTRAM")]
pub struct Extram {
    protect: Protect,
}
impl Extram {
    #[doc = "0x00 - Description cluster: Control access from master connected to AMLI master port EXTRAM\\[n\\]"]
    #[inline(always)]
    pub const fn protect(&self) -> &Protect {
        &self.protect
    }
}
#[doc = "PROTECT (rw) register accessor: Description cluster: Control access from master connected to AMLI master port EXTRAM\\[n\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`protect::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`protect::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@protect`] module"]
#[doc(alias = "PROTECT")]
pub type Protect = crate::Reg<protect::ProtectSpec>;
#[doc = "Description cluster: Control access from master connected to AMLI master port EXTRAM\\[n\\]"]
pub mod protect;
