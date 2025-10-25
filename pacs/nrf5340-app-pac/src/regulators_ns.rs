#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0428],
    mainregstatus: Mainregstatus,
    _reserved1: [u8; 0xd4],
    systemoff: Systemoff,
    _reserved2: [u8; 0x0c],
    pofcon: Pofcon,
    _reserved3: [u8; 0x01f0],
    vregmain: Vregmain,
    _reserved4: [u8; 0x01f8],
    vregradio: Vregradio,
    _reserved5: [u8; 0x01f8],
    vregh: Vregh,
}
impl RegisterBlock {
    #[doc = "0x428 - Main supply status"]
    #[inline(always)]
    pub const fn mainregstatus(&self) -> &Mainregstatus {
        &self.mainregstatus
    }
    #[doc = "0x500 - System OFF register"]
    #[inline(always)]
    pub const fn systemoff(&self) -> &Systemoff {
        &self.systemoff
    }
    #[doc = "0x510 - Power-fail comparator configuration"]
    #[inline(always)]
    pub const fn pofcon(&self) -> &Pofcon {
        &self.pofcon
    }
    #[doc = "0x704 - Unspecified"]
    #[inline(always)]
    pub const fn vregmain(&self) -> &Vregmain {
        &self.vregmain
    }
    #[doc = "0x900..0x908 - Unspecified"]
    #[inline(always)]
    pub const fn vregradio(&self) -> &Vregradio {
        &self.vregradio
    }
    #[doc = "0xb00 - Unspecified"]
    #[inline(always)]
    pub const fn vregh(&self) -> &Vregh {
        &self.vregh
    }
}
#[doc = "MAINREGSTATUS (r) register accessor: Main supply status\n\nYou can [`read`](crate::Reg::read) this register and get [`mainregstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mainregstatus`] module"]
#[doc(alias = "MAINREGSTATUS")]
pub type Mainregstatus = crate::Reg<mainregstatus::MainregstatusSpec>;
#[doc = "Main supply status"]
pub mod mainregstatus;
#[doc = "SYSTEMOFF (w) register accessor: System OFF register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`systemoff::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@systemoff`] module"]
#[doc(alias = "SYSTEMOFF")]
pub type Systemoff = crate::Reg<systemoff::SystemoffSpec>;
#[doc = "System OFF register"]
pub mod systemoff;
#[doc = "POFCON (rw) register accessor: Power-fail comparator configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`pofcon::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pofcon::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pofcon`] module"]
#[doc(alias = "POFCON")]
pub type Pofcon = crate::Reg<pofcon::PofconSpec>;
#[doc = "Power-fail comparator configuration"]
pub mod pofcon;
#[doc = "Unspecified"]
pub use self::vregmain::Vregmain;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod vregmain;
#[doc = "Unspecified"]
pub use self::vregradio::Vregradio;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod vregradio;
#[doc = "Unspecified"]
pub use self::vregh::Vregh;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod vregh;
