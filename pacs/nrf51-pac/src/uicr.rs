#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Length of code region 0."]
    pub clenr0: crate::Reg<clenr0::CLENR0_SPEC>,
    #[doc = "0x04 - Readback protection configuration."]
    pub rbpconf: crate::Reg<rbpconf::RBPCONF_SPEC>,
    #[doc = "0x08 - Reset value for CLOCK XTALFREQ register."]
    pub xtalfreq: crate::Reg<xtalfreq::XTALFREQ_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - Firmware ID."]
    pub fwid: crate::Reg<fwid::FWID_SPEC>,
    _reserved_4_nrffw: [u8; 0x3c],
    #[doc = "0x50..0x80 - Reserved for Nordic hardware design."]
    pub nrfhw: [crate::Reg<nrfhw::NRFHW_SPEC>; 12],
    #[doc = "0x80..0x100 - Reserved for customer."]
    pub customer: [crate::Reg<customer::CUSTOMER_SPEC>; 32],
}
impl RegisterBlock {
    #[doc = "0x14..0x50 - Reserved for Nordic firmware design."]
    #[inline(always)]
    pub fn nrffw(&self) -> &[crate::Reg<nrffw::NRFFW_SPEC>; 15] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(20usize)
                as *const [crate::Reg<nrffw::NRFFW_SPEC>; 15])
        }
    }
    #[doc = "0x14 - Bootloader start address."]
    #[inline(always)]
    pub fn bootloaderaddr(&self) -> &crate::Reg<bootloaderaddr::BOOTLOADERADDR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(20usize)
                as *const crate::Reg<bootloaderaddr::BOOTLOADERADDR_SPEC>)
        }
    }
}
#[doc = "CLENR0 register accessor: an alias for `Reg<CLENR0_SPEC>`"]
pub type CLENR0 = crate::Reg<clenr0::CLENR0_SPEC>;
#[doc = "Length of code region 0."]
pub mod clenr0;
#[doc = "RBPCONF register accessor: an alias for `Reg<RBPCONF_SPEC>`"]
pub type RBPCONF = crate::Reg<rbpconf::RBPCONF_SPEC>;
#[doc = "Readback protection configuration."]
pub mod rbpconf;
#[doc = "XTALFREQ register accessor: an alias for `Reg<XTALFREQ_SPEC>`"]
pub type XTALFREQ = crate::Reg<xtalfreq::XTALFREQ_SPEC>;
#[doc = "Reset value for CLOCK XTALFREQ register."]
pub mod xtalfreq;
#[doc = "FWID register accessor: an alias for `Reg<FWID_SPEC>`"]
pub type FWID = crate::Reg<fwid::FWID_SPEC>;
#[doc = "Firmware ID."]
pub mod fwid;
#[doc = "BOOTLOADERADDR register accessor: an alias for `Reg<BOOTLOADERADDR_SPEC>`"]
pub type BOOTLOADERADDR = crate::Reg<bootloaderaddr::BOOTLOADERADDR_SPEC>;
#[doc = "Bootloader start address."]
pub mod bootloaderaddr;
#[doc = "NRFFW register accessor: an alias for `Reg<NRFFW_SPEC>`"]
pub type NRFFW = crate::Reg<nrffw::NRFFW_SPEC>;
#[doc = "Reserved for Nordic firmware design."]
pub mod nrffw;
#[doc = "NRFHW register accessor: an alias for `Reg<NRFHW_SPEC>`"]
pub type NRFHW = crate::Reg<nrfhw::NRFHW_SPEC>;
#[doc = "Reserved for Nordic hardware design."]
pub mod nrfhw;
#[doc = "CUSTOMER register accessor: an alias for `Reg<CUSTOMER_SPEC>`"]
pub type CUSTOMER = crate::Reg<customer::CUSTOMER_SPEC>;
#[doc = "Reserved for customer."]
pub mod customer;
