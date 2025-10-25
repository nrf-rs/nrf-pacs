#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    clenr0: Clenr0,
    rbpconf: Rbpconf,
    xtalfreq: Xtalfreq,
    _reserved3: [u8; 0x04],
    fwid: Fwid,
    _reserved_4_nrffw: [u8; 0x3c],
    nrfhw: [Nrfhw; 12],
    customer: [Customer; 32],
}
impl RegisterBlock {
    #[doc = "0x00 - Length of code region 0."]
    #[inline(always)]
    pub const fn clenr0(&self) -> &Clenr0 {
        &self.clenr0
    }
    #[doc = "0x04 - Readback protection configuration."]
    #[inline(always)]
    pub const fn rbpconf(&self) -> &Rbpconf {
        &self.rbpconf
    }
    #[doc = "0x08 - Reset value for CLOCK XTALFREQ register."]
    #[inline(always)]
    pub const fn xtalfreq(&self) -> &Xtalfreq {
        &self.xtalfreq
    }
    #[doc = "0x10 - Firmware ID."]
    #[inline(always)]
    pub const fn fwid(&self) -> &Fwid {
        &self.fwid
    }
    #[doc = "0x14..0x50 - Reserved for Nordic firmware design."]
    #[inline(always)]
    pub const fn nrffw(&self, n: usize) -> &Nrffw {
        #[allow(clippy::no_effect)]
        [(); 15][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(20)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x14..0x50 - Reserved for Nordic firmware design."]
    #[inline(always)]
    pub fn nrffw_iter(&self) -> impl Iterator<Item = &Nrffw> {
        (0..15).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(20)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x14 - Bootloader start address."]
    #[inline(always)]
    pub const fn bootloaderaddr(&self) -> &Bootloaderaddr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(20).cast() }
    }
    #[doc = "0x50..0x80 - Reserved for Nordic hardware design."]
    #[inline(always)]
    pub const fn nrfhw(&self, n: usize) -> &Nrfhw {
        &self.nrfhw[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x50..0x80 - Reserved for Nordic hardware design."]
    #[inline(always)]
    pub fn nrfhw_iter(&self) -> impl Iterator<Item = &Nrfhw> {
        self.nrfhw.iter()
    }
    #[doc = "0x80..0x100 - Reserved for customer."]
    #[inline(always)]
    pub const fn customer(&self, n: usize) -> &Customer {
        &self.customer[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0x100 - Reserved for customer."]
    #[inline(always)]
    pub fn customer_iter(&self) -> impl Iterator<Item = &Customer> {
        self.customer.iter()
    }
}
#[doc = "CLENR0 (rw) register accessor: Length of code region 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`clenr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clenr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clenr0`] module"]
#[doc(alias = "CLENR0")]
pub type Clenr0 = crate::Reg<clenr0::Clenr0Spec>;
#[doc = "Length of code region 0."]
pub mod clenr0;
#[doc = "RBPCONF (rw) register accessor: Readback protection configuration.\n\nYou can [`read`](crate::Reg::read) this register and get [`rbpconf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rbpconf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rbpconf`] module"]
#[doc(alias = "RBPCONF")]
pub type Rbpconf = crate::Reg<rbpconf::RbpconfSpec>;
#[doc = "Readback protection configuration."]
pub mod rbpconf;
#[doc = "XTALFREQ (rw) register accessor: Reset value for CLOCK XTALFREQ register.\n\nYou can [`read`](crate::Reg::read) this register and get [`xtalfreq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtalfreq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xtalfreq`] module"]
#[doc(alias = "XTALFREQ")]
pub type Xtalfreq = crate::Reg<xtalfreq::XtalfreqSpec>;
#[doc = "Reset value for CLOCK XTALFREQ register."]
pub mod xtalfreq;
#[doc = "FWID (r) register accessor: Firmware ID.\n\nYou can [`read`](crate::Reg::read) this register and get [`fwid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fwid`] module"]
#[doc(alias = "FWID")]
pub type Fwid = crate::Reg<fwid::FwidSpec>;
#[doc = "Firmware ID."]
pub mod fwid;
#[doc = "BOOTLOADERADDR (rw) register accessor: Bootloader start address.\n\nYou can [`read`](crate::Reg::read) this register and get [`bootloaderaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootloaderaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bootloaderaddr`] module"]
#[doc(alias = "BOOTLOADERADDR")]
pub type Bootloaderaddr = crate::Reg<bootloaderaddr::BootloaderaddrSpec>;
#[doc = "Bootloader start address."]
pub mod bootloaderaddr;
#[doc = "NRFFW (rw) register accessor: Reserved for Nordic firmware design.\n\nYou can [`read`](crate::Reg::read) this register and get [`nrffw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nrffw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nrffw`] module"]
#[doc(alias = "NRFFW")]
pub type Nrffw = crate::Reg<nrffw::NrffwSpec>;
#[doc = "Reserved for Nordic firmware design."]
pub mod nrffw;
#[doc = "NRFHW (rw) register accessor: Reserved for Nordic hardware design.\n\nYou can [`read`](crate::Reg::read) this register and get [`nrfhw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nrfhw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nrfhw`] module"]
#[doc(alias = "NRFHW")]
pub type Nrfhw = crate::Reg<nrfhw::NrfhwSpec>;
#[doc = "Reserved for Nordic hardware design."]
pub mod nrfhw;
#[doc = "CUSTOMER (rw) register accessor: Reserved for customer.\n\nYou can [`read`](crate::Reg::read) this register and get [`customer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`customer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@customer`] module"]
#[doc(alias = "CUSTOMER")]
pub type Customer = crate::Reg<customer::CustomerSpec>;
#[doc = "Reserved for customer."]
pub mod customer;
