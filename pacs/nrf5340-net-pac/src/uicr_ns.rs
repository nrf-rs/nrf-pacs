#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    approtect: Approtect,
    eraseprotect: Eraseprotect,
    _reserved2: [u8; 0x01f8],
    nrffw: [Nrffw; 32],
    _reserved3: [u8; 0x80],
    customer: [Customer; 32],
}
impl RegisterBlock {
    #[doc = "0x00 - Access port protection"]
    #[inline(always)]
    pub const fn approtect(&self) -> &Approtect {
        &self.approtect
    }
    #[doc = "0x04 - Erase protection"]
    #[inline(always)]
    pub const fn eraseprotect(&self) -> &Eraseprotect {
        &self.eraseprotect
    }
    #[doc = "0x200..0x280 - Description collection: Reserved for Nordic firmware design"]
    #[inline(always)]
    pub const fn nrffw(&self, n: usize) -> &Nrffw {
        &self.nrffw[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x200..0x280 - Description collection: Reserved for Nordic firmware design"]
    #[inline(always)]
    pub fn nrffw_iter(&self) -> impl Iterator<Item = &Nrffw> {
        self.nrffw.iter()
    }
    #[doc = "0x300..0x380 - Description collection: Reserved for customer"]
    #[inline(always)]
    pub const fn customer(&self, n: usize) -> &Customer {
        &self.customer[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x300..0x380 - Description collection: Reserved for customer"]
    #[inline(always)]
    pub fn customer_iter(&self) -> impl Iterator<Item = &Customer> {
        self.customer.iter()
    }
}
#[doc = "APPROTECT (rw) register accessor: Access port protection\n\nYou can [`read`](crate::Reg::read) this register and get [`approtect::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`approtect::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@approtect`] module"]
#[doc(alias = "APPROTECT")]
pub type Approtect = crate::Reg<approtect::ApprotectSpec>;
#[doc = "Access port protection"]
pub mod approtect;
#[doc = "ERASEPROTECT (rw) register accessor: Erase protection\n\nYou can [`read`](crate::Reg::read) this register and get [`eraseprotect::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eraseprotect::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eraseprotect`] module"]
#[doc(alias = "ERASEPROTECT")]
pub type Eraseprotect = crate::Reg<eraseprotect::EraseprotectSpec>;
#[doc = "Erase protection"]
pub mod eraseprotect;
#[doc = "NRFFW (rw) register accessor: Description collection: Reserved for Nordic firmware design\n\nYou can [`read`](crate::Reg::read) this register and get [`nrffw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nrffw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nrffw`] module"]
#[doc(alias = "NRFFW")]
pub type Nrffw = crate::Reg<nrffw::NrffwSpec>;
#[doc = "Description collection: Reserved for Nordic firmware design"]
pub mod nrffw;
#[doc = "CUSTOMER (rw) register accessor: Description collection: Reserved for customer\n\nYou can [`read`](crate::Reg::read) this register and get [`customer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`customer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@customer`] module"]
#[doc(alias = "CUSTOMER")]
pub type Customer = crate::Reg<customer::CustomerSpec>;
#[doc = "Description collection: Reserved for customer"]
pub mod customer;
