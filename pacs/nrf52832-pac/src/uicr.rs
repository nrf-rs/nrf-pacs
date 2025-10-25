#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    unused0: Unused0,
    unused1: Unused1,
    unused2: Unused2,
    _reserved3: [u8; 0x04],
    unused3: Unused3,
    nrffw: [Nrffw; 15],
    nrfhw: [Nrfhw; 12],
    customer: [Customer; 32],
    _reserved7: [u8; 0x0100],
    pselreset: [Pselreset; 2],
    approtect: Approtect,
    nfcpins: Nfcpins,
}
impl RegisterBlock {
    #[doc = "0x00 - Unspecified"]
    #[inline(always)]
    pub const fn unused0(&self) -> &Unused0 {
        &self.unused0
    }
    #[doc = "0x04 - Unspecified"]
    #[inline(always)]
    pub const fn unused1(&self) -> &Unused1 {
        &self.unused1
    }
    #[doc = "0x08 - Unspecified"]
    #[inline(always)]
    pub const fn unused2(&self) -> &Unused2 {
        &self.unused2
    }
    #[doc = "0x10 - Unspecified"]
    #[inline(always)]
    pub const fn unused3(&self) -> &Unused3 {
        &self.unused3
    }
    #[doc = "0x14..0x50 - Description collection\\[0\\]: Reserved for Nordic firmware design"]
    #[inline(always)]
    pub const fn nrffw(&self, n: usize) -> &Nrffw {
        &self.nrffw[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x14..0x50 - Description collection\\[0\\]: Reserved for Nordic firmware design"]
    #[inline(always)]
    pub fn nrffw_iter(&self) -> impl Iterator<Item = &Nrffw> {
        self.nrffw.iter()
    }
    #[doc = "0x50..0x80 - Description collection\\[0\\]: Reserved for Nordic hardware design"]
    #[inline(always)]
    pub const fn nrfhw(&self, n: usize) -> &Nrfhw {
        &self.nrfhw[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x50..0x80 - Description collection\\[0\\]: Reserved for Nordic hardware design"]
    #[inline(always)]
    pub fn nrfhw_iter(&self) -> impl Iterator<Item = &Nrfhw> {
        self.nrfhw.iter()
    }
    #[doc = "0x80..0x100 - Description collection\\[0\\]: Reserved for customer"]
    #[inline(always)]
    pub const fn customer(&self, n: usize) -> &Customer {
        &self.customer[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0x100 - Description collection\\[0\\]: Reserved for customer"]
    #[inline(always)]
    pub fn customer_iter(&self) -> impl Iterator<Item = &Customer> {
        self.customer.iter()
    }
    #[doc = "0x200..0x208 - Description collection\\[0\\]: Mapping of the nRESET function (see POWER chapter for details)"]
    #[inline(always)]
    pub const fn pselreset(&self, n: usize) -> &Pselreset {
        &self.pselreset[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x200..0x208 - Description collection\\[0\\]: Mapping of the nRESET function (see POWER chapter for details)"]
    #[inline(always)]
    pub fn pselreset_iter(&self) -> impl Iterator<Item = &Pselreset> {
        self.pselreset.iter()
    }
    #[doc = "0x208 - Access Port protection"]
    #[inline(always)]
    pub const fn approtect(&self) -> &Approtect {
        &self.approtect
    }
    #[doc = "0x20c - Setting of pins dedicated to NFC functionality: NFC antenna or GPIO"]
    #[inline(always)]
    pub const fn nfcpins(&self) -> &Nfcpins {
        &self.nfcpins
    }
}
#[doc = "UNUSED0 (rw) register accessor: Unspecified\n\nYou can [`read`](crate::Reg::read) this register and get [`unused0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`unused0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@unused0`] module"]
#[doc(alias = "UNUSED0")]
pub type Unused0 = crate::Reg<unused0::Unused0Spec>;
#[doc = "Unspecified"]
pub mod unused0;
#[doc = "UNUSED1 (rw) register accessor: Unspecified\n\nYou can [`read`](crate::Reg::read) this register and get [`unused1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`unused1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@unused1`] module"]
#[doc(alias = "UNUSED1")]
pub type Unused1 = crate::Reg<unused1::Unused1Spec>;
#[doc = "Unspecified"]
pub mod unused1;
#[doc = "UNUSED2 (rw) register accessor: Unspecified\n\nYou can [`read`](crate::Reg::read) this register and get [`unused2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`unused2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@unused2`] module"]
#[doc(alias = "UNUSED2")]
pub type Unused2 = crate::Reg<unused2::Unused2Spec>;
#[doc = "Unspecified"]
pub mod unused2;
#[doc = "UNUSED3 (rw) register accessor: Unspecified\n\nYou can [`read`](crate::Reg::read) this register and get [`unused3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`unused3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@unused3`] module"]
#[doc(alias = "UNUSED3")]
pub type Unused3 = crate::Reg<unused3::Unused3Spec>;
#[doc = "Unspecified"]
pub mod unused3;
#[doc = "NRFFW (rw) register accessor: Description collection\\[0\\]: Reserved for Nordic firmware design\n\nYou can [`read`](crate::Reg::read) this register and get [`nrffw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nrffw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nrffw`] module"]
#[doc(alias = "NRFFW")]
pub type Nrffw = crate::Reg<nrffw::NrffwSpec>;
#[doc = "Description collection\\[0\\]: Reserved for Nordic firmware design"]
pub mod nrffw;
#[doc = "NRFHW (rw) register accessor: Description collection\\[0\\]: Reserved for Nordic hardware design\n\nYou can [`read`](crate::Reg::read) this register and get [`nrfhw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nrfhw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nrfhw`] module"]
#[doc(alias = "NRFHW")]
pub type Nrfhw = crate::Reg<nrfhw::NrfhwSpec>;
#[doc = "Description collection\\[0\\]: Reserved for Nordic hardware design"]
pub mod nrfhw;
#[doc = "CUSTOMER (rw) register accessor: Description collection\\[0\\]: Reserved for customer\n\nYou can [`read`](crate::Reg::read) this register and get [`customer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`customer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@customer`] module"]
#[doc(alias = "CUSTOMER")]
pub type Customer = crate::Reg<customer::CustomerSpec>;
#[doc = "Description collection\\[0\\]: Reserved for customer"]
pub mod customer;
#[doc = "PSELRESET (rw) register accessor: Description collection\\[0\\]: Mapping of the nRESET function (see POWER chapter for details)\n\nYou can [`read`](crate::Reg::read) this register and get [`pselreset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pselreset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pselreset`] module"]
#[doc(alias = "PSELRESET")]
pub type Pselreset = crate::Reg<pselreset::PselresetSpec>;
#[doc = "Description collection\\[0\\]: Mapping of the nRESET function (see POWER chapter for details)"]
pub mod pselreset;
#[doc = "APPROTECT (rw) register accessor: Access Port protection\n\nYou can [`read`](crate::Reg::read) this register and get [`approtect::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`approtect::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@approtect`] module"]
#[doc(alias = "APPROTECT")]
pub type Approtect = crate::Reg<approtect::ApprotectSpec>;
#[doc = "Access Port protection"]
pub mod approtect;
#[doc = "NFCPINS (rw) register accessor: Setting of pins dedicated to NFC functionality: NFC antenna or GPIO\n\nYou can [`read`](crate::Reg::read) this register and get [`nfcpins::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nfcpins::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nfcpins`] module"]
#[doc(alias = "NFCPINS")]
pub type Nfcpins = crate::Reg<nfcpins::NfcpinsSpec>;
#[doc = "Setting of pins dedicated to NFC functionality: NFC antenna or GPIO"]
pub mod nfcpins;
