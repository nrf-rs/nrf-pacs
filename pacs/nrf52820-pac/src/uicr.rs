#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x14],
    nrffw: [Nrffw; 13],
    _reserved1: [u8; 0x08],
    nrfhw: [Nrfhw; 12],
    customer: [Customer; 32],
    _reserved3: [u8; 0x0100],
    pselreset: [Pselreset; 2],
    approtect: Approtect,
    _reserved5: [u8; 0x04],
    debugctrl: Debugctrl,
    _reserved6: [u8; 0xf0],
    regout0: Regout0,
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
    #[doc = "0x210 - Processor debug control"]
    #[inline(always)]
    pub const fn debugctrl(&self) -> &Debugctrl {
        &self.debugctrl
    }
    #[doc = "0x304 - Output voltage from REG0 regulator stage. The maximum output voltage from this stage is given as VDDH - V_VDDH-VDD."]
    #[inline(always)]
    pub const fn regout0(&self) -> &Regout0 {
        &self.regout0
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
#[doc = "DEBUGCTRL (rw) register accessor: Processor debug control\n\nYou can [`read`](crate::Reg::read) this register and get [`debugctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debugctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debugctrl`] module"]
#[doc(alias = "DEBUGCTRL")]
pub type Debugctrl = crate::Reg<debugctrl::DebugctrlSpec>;
#[doc = "Processor debug control"]
pub mod debugctrl;
#[doc = "REGOUT0 (rw) register accessor: Output voltage from REG0 regulator stage. The maximum output voltage from this stage is given as VDDH - V_VDDH-VDD.\n\nYou can [`read`](crate::Reg::read) this register and get [`regout0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`regout0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regout0`] module"]
#[doc(alias = "REGOUT0")]
pub type Regout0 = crate::Reg<regout0::Regout0Spec>;
#[doc = "Output voltage from REG0 regulator stage. The maximum output voltage from this stage is given as VDDH - V_VDDH-VDD."]
pub mod regout0;
