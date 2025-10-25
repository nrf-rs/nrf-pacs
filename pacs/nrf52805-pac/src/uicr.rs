#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x14],
    nrffw: [Nrffw; 13],
    _reserved1: [u8; 0x08],
    nrfhw: [Nrfhw; 12],
    customer: [Customer; 32],
    nrfmdk: [Nrfmdk; 8],
    _reserved4: [u8; 0xe0],
    pselreset: [Pselreset; 2],
    approtect: Approtect,
}
impl RegisterBlock {
    #[doc = "0x14..0x48 - Description collection: Reserved for Nordic firmware design"]
    #[inline(always)]
    pub const fn nrffw(&self, n: usize) -> &Nrffw {
        &self.nrffw[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x14..0x48 - Description collection: Reserved for Nordic firmware design"]
    #[inline(always)]
    pub fn nrffw_iter(&self) -> impl Iterator<Item = &Nrffw> {
        self.nrffw.iter()
    }
    #[doc = "0x50..0x80 - Description collection: Reserved for Nordic hardware design"]
    #[inline(always)]
    pub const fn nrfhw(&self, n: usize) -> &Nrfhw {
        &self.nrfhw[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x50..0x80 - Description collection: Reserved for Nordic hardware design"]
    #[inline(always)]
    pub fn nrfhw_iter(&self) -> impl Iterator<Item = &Nrfhw> {
        self.nrfhw.iter()
    }
    #[doc = "0x80..0x100 - Description collection: Reserved for customer"]
    #[inline(always)]
    pub const fn customer(&self, n: usize) -> &Customer {
        &self.customer[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0x100 - Description collection: Reserved for customer"]
    #[inline(always)]
    pub fn customer_iter(&self) -> impl Iterator<Item = &Customer> {
        self.customer.iter()
    }
    #[doc = "0x100..0x120 - Description collection: Reserved for Nordic MDK"]
    #[inline(always)]
    pub const fn nrfmdk(&self, n: usize) -> &Nrfmdk {
        &self.nrfmdk[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x120 - Description collection: Reserved for Nordic MDK"]
    #[inline(always)]
    pub fn nrfmdk_iter(&self) -> impl Iterator<Item = &Nrfmdk> {
        self.nrfmdk.iter()
    }
    #[doc = "0x200..0x208 - Description collection: Mapping of the nRESET function (see POWER chapter for details)"]
    #[inline(always)]
    pub const fn pselreset(&self, n: usize) -> &Pselreset {
        &self.pselreset[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x200..0x208 - Description collection: Mapping of the nRESET function (see POWER chapter for details)"]
    #[inline(always)]
    pub fn pselreset_iter(&self) -> impl Iterator<Item = &Pselreset> {
        self.pselreset.iter()
    }
    #[doc = "0x208 - Access port protection"]
    #[inline(always)]
    pub const fn approtect(&self) -> &Approtect {
        &self.approtect
    }
}
#[doc = "NRFFW (rw) register accessor: Description collection: Reserved for Nordic firmware design\n\nYou can [`read`](crate::Reg::read) this register and get [`nrffw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nrffw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nrffw`] module"]
#[doc(alias = "NRFFW")]
pub type Nrffw = crate::Reg<nrffw::NrffwSpec>;
#[doc = "Description collection: Reserved for Nordic firmware design"]
pub mod nrffw;
#[doc = "NRFHW (rw) register accessor: Description collection: Reserved for Nordic hardware design\n\nYou can [`read`](crate::Reg::read) this register and get [`nrfhw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nrfhw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nrfhw`] module"]
#[doc(alias = "NRFHW")]
pub type Nrfhw = crate::Reg<nrfhw::NrfhwSpec>;
#[doc = "Description collection: Reserved for Nordic hardware design"]
pub mod nrfhw;
#[doc = "CUSTOMER (rw) register accessor: Description collection: Reserved for customer\n\nYou can [`read`](crate::Reg::read) this register and get [`customer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`customer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@customer`] module"]
#[doc(alias = "CUSTOMER")]
pub type Customer = crate::Reg<customer::CustomerSpec>;
#[doc = "Description collection: Reserved for customer"]
pub mod customer;
#[doc = "NRFMDK (rw) register accessor: Description collection: Reserved for Nordic MDK\n\nYou can [`read`](crate::Reg::read) this register and get [`nrfmdk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nrfmdk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nrfmdk`] module"]
#[doc(alias = "NRFMDK")]
pub type Nrfmdk = crate::Reg<nrfmdk::NrfmdkSpec>;
#[doc = "Description collection: Reserved for Nordic MDK"]
pub mod nrfmdk;
#[doc = "PSELRESET (rw) register accessor: Description collection: Mapping of the nRESET function (see POWER chapter for details)\n\nYou can [`read`](crate::Reg::read) this register and get [`pselreset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pselreset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pselreset`] module"]
#[doc(alias = "PSELRESET")]
pub type Pselreset = crate::Reg<pselreset::PselresetSpec>;
#[doc = "Description collection: Mapping of the nRESET function (see POWER chapter for details)"]
pub mod pselreset;
#[doc = "APPROTECT (rw) register accessor: Access port protection\n\nYou can [`read`](crate::Reg::read) this register and get [`approtect::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`approtect::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@approtect`] module"]
#[doc(alias = "APPROTECT")]
pub type Approtect = crate::Reg<approtect::ApprotectSpec>;
#[doc = "Access port protection"]
pub mod approtect;
