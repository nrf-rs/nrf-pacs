#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "PSEL")]
pub struct Psel {
    scl: Scl,
    sda: Sda,
}
impl Psel {
    #[doc = "0x00 - Pin select for SCL signal"]
    #[inline(always)]
    pub const fn scl(&self) -> &Scl {
        &self.scl
    }
    #[doc = "0x04 - Pin select for SDA signal"]
    #[inline(always)]
    pub const fn sda(&self) -> &Sda {
        &self.sda
    }
}
#[doc = "SCL (rw) register accessor: Pin select for SCL signal\n\nYou can [`read`](crate::Reg::read) this register and get [`scl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scl`] module"]
#[doc(alias = "SCL")]
pub type Scl = crate::Reg<scl::SclSpec>;
#[doc = "Pin select for SCL signal"]
pub mod scl;
#[doc = "SDA (rw) register accessor: Pin select for SDA signal\n\nYou can [`read`](crate::Reg::read) this register and get [`sda::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sda::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sda`] module"]
#[doc(alias = "SDA")]
pub type Sda = crate::Reg<sda::SdaSpec>;
#[doc = "Pin select for SDA signal"]
pub mod sda;
