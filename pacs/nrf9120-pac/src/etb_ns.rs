#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    rdp: Rdp,
    _reserved1: [u8; 0x04],
    sts: Sts,
    rrd: Rrd,
    rrp: Rrp,
    rwp: Rwp,
    trg: Trg,
    ctl: Ctl,
    rwd: Rwd,
    _reserved8: [u8; 0x02d8],
    ffsr: Ffsr,
    ffcr: Ffcr,
    _reserved10: [u8; 0x0bd8],
    itmiscop0: Itmiscop0,
    ittrflinack: Ittrflinack,
    ittrflin: Ittrflin,
    itatbdata0: Itatbdata0,
    itatbctr2: Itatbctr2,
    itatbctr1: Itatbctr1,
    itatbctr0: Itatbctr0,
    _reserved17: [u8; 0x04],
    itctrl: Itctrl,
    _reserved18: [u8; 0x9c],
    claimset: Claimset,
    claimclr: Claimclr,
    _reserved20: [u8; 0x08],
    lar: Lar,
    lsr: Lsr,
    authstatus: Authstatus,
    _reserved23: [u8; 0x0c],
    devid: Devid,
    devtype: Devtype,
    periphid4: Periphid4,
    _reserved26: [u8; 0x0c],
    periphid0: Periphid0,
    periphid1: Periphid1,
    periphid2: Periphid2,
    periphid3: Periphid3,
    compid0: Compid0,
    compid1: Compid1,
    compid2: Compid2,
    compid3: Compid3,
}
impl RegisterBlock {
    #[doc = "0x04 - ETB RAM Depth Register"]
    #[inline(always)]
    pub const fn rdp(&self) -> &Rdp {
        &self.rdp
    }
    #[doc = "0x0c - ETB Status Register"]
    #[inline(always)]
    pub const fn sts(&self) -> &Sts {
        &self.sts
    }
    #[doc = "0x10 - ETB RAM Read Data Register"]
    #[inline(always)]
    pub const fn rrd(&self) -> &Rrd {
        &self.rrd
    }
    #[doc = "0x14 - ETB RAM Read Pointer Register"]
    #[inline(always)]
    pub const fn rrp(&self) -> &Rrp {
        &self.rrp
    }
    #[doc = "0x18 - ETB RAM Write Pointer Register"]
    #[inline(always)]
    pub const fn rwp(&self) -> &Rwp {
        &self.rwp
    }
    #[doc = "0x1c - ETB Trigger Counter Register"]
    #[inline(always)]
    pub const fn trg(&self) -> &Trg {
        &self.trg
    }
    #[doc = "0x20 - ETB Control Register"]
    #[inline(always)]
    pub const fn ctl(&self) -> &Ctl {
        &self.ctl
    }
    #[doc = "0x24 - ETB RAM Write Data Register"]
    #[inline(always)]
    pub const fn rwd(&self) -> &Rwd {
        &self.rwd
    }
    #[doc = "0x300 - ETB Formatter and Flush Status Register"]
    #[inline(always)]
    pub const fn ffsr(&self) -> &Ffsr {
        &self.ffsr
    }
    #[doc = "0x304 - ETB Formatter and Flush Control Register"]
    #[inline(always)]
    pub const fn ffcr(&self) -> &Ffcr {
        &self.ffcr
    }
    #[doc = "0xee0 - Integration Test Miscellaneous Output Register 0"]
    #[inline(always)]
    pub const fn itmiscop0(&self) -> &Itmiscop0 {
        &self.itmiscop0
    }
    #[doc = "0xee4 - Integration Test Trigger In and Flush In Acknowledge Register"]
    #[inline(always)]
    pub const fn ittrflinack(&self) -> &Ittrflinack {
        &self.ittrflinack
    }
    #[doc = "0xee8 - Integration Test Trigger In and Flush In Register"]
    #[inline(always)]
    pub const fn ittrflin(&self) -> &Ittrflin {
        &self.ittrflin
    }
    #[doc = "0xeec - Integration Test ATB Data Register 0"]
    #[inline(always)]
    pub const fn itatbdata0(&self) -> &Itatbdata0 {
        &self.itatbdata0
    }
    #[doc = "0xef0 - Integration Test ATB Control Register 2"]
    #[inline(always)]
    pub const fn itatbctr2(&self) -> &Itatbctr2 {
        &self.itatbctr2
    }
    #[doc = "0xef4 - Integration Test ATB Control Register 1"]
    #[inline(always)]
    pub const fn itatbctr1(&self) -> &Itatbctr1 {
        &self.itatbctr1
    }
    #[doc = "0xef8 - Integration Test ATB Control Register 0"]
    #[inline(always)]
    pub const fn itatbctr0(&self) -> &Itatbctr0 {
        &self.itatbctr0
    }
    #[doc = "0xf00 - Integration Mode Control Register"]
    #[inline(always)]
    pub const fn itctrl(&self) -> &Itctrl {
        &self.itctrl
    }
    #[doc = "0xfa0 - Claim Tag Set Register"]
    #[inline(always)]
    pub const fn claimset(&self) -> &Claimset {
        &self.claimset
    }
    #[doc = "0xfa4 - Claim Tag Clear Register"]
    #[inline(always)]
    pub const fn claimclr(&self) -> &Claimclr {
        &self.claimclr
    }
    #[doc = "0xfb0 - Lock Access Register"]
    #[inline(always)]
    pub const fn lar(&self) -> &Lar {
        &self.lar
    }
    #[doc = "0xfb4 - Lock Status Register"]
    #[inline(always)]
    pub const fn lsr(&self) -> &Lsr {
        &self.lsr
    }
    #[doc = "0xfb8 - Authentication Status Register"]
    #[inline(always)]
    pub const fn authstatus(&self) -> &Authstatus {
        &self.authstatus
    }
    #[doc = "0xfc8 - Device Configuration Register"]
    #[inline(always)]
    pub const fn devid(&self) -> &Devid {
        &self.devid
    }
    #[doc = "0xfcc - Device Type Identifier Register"]
    #[inline(always)]
    pub const fn devtype(&self) -> &Devtype {
        &self.devtype
    }
    #[doc = "0xfd0 - Peripheral ID4 Register"]
    #[inline(always)]
    pub const fn periphid4(&self) -> &Periphid4 {
        &self.periphid4
    }
    #[doc = "0xfe0 - Peripheral ID0 Register"]
    #[inline(always)]
    pub const fn periphid0(&self) -> &Periphid0 {
        &self.periphid0
    }
    #[doc = "0xfe4 - Peripheral ID1 Register"]
    #[inline(always)]
    pub const fn periphid1(&self) -> &Periphid1 {
        &self.periphid1
    }
    #[doc = "0xfe8 - Peripheral ID2 Register"]
    #[inline(always)]
    pub const fn periphid2(&self) -> &Periphid2 {
        &self.periphid2
    }
    #[doc = "0xfec - Peripheral ID3 Register"]
    #[inline(always)]
    pub const fn periphid3(&self) -> &Periphid3 {
        &self.periphid3
    }
    #[doc = "0xff0 - Component ID0 Register"]
    #[inline(always)]
    pub const fn compid0(&self) -> &Compid0 {
        &self.compid0
    }
    #[doc = "0xff4 - Component ID1 Register"]
    #[inline(always)]
    pub const fn compid1(&self) -> &Compid1 {
        &self.compid1
    }
    #[doc = "0xff8 - Component ID2 Register"]
    #[inline(always)]
    pub const fn compid2(&self) -> &Compid2 {
        &self.compid2
    }
    #[doc = "0xffc - Component ID3 Register"]
    #[inline(always)]
    pub const fn compid3(&self) -> &Compid3 {
        &self.compid3
    }
}
#[doc = "RDP (r) register accessor: ETB RAM Depth Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rdp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdp`] module"]
#[doc(alias = "RDP")]
pub type Rdp = crate::Reg<rdp::RdpSpec>;
#[doc = "ETB RAM Depth Register"]
pub mod rdp;
#[doc = "STS (r) register accessor: ETB Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sts`] module"]
#[doc(alias = "STS")]
pub type Sts = crate::Reg<sts::StsSpec>;
#[doc = "ETB Status Register"]
pub mod sts;
#[doc = "RRD (r) register accessor: ETB RAM Read Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rrd::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rrd`] module"]
#[doc(alias = "RRD")]
pub type Rrd = crate::Reg<rrd::RrdSpec>;
#[doc = "ETB RAM Read Data Register"]
pub mod rrd;
#[doc = "RRP (rw) register accessor: ETB RAM Read Pointer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rrp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rrp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rrp`] module"]
#[doc(alias = "RRP")]
pub type Rrp = crate::Reg<rrp::RrpSpec>;
#[doc = "ETB RAM Read Pointer Register"]
pub mod rrp;
#[doc = "RWP (rw) register accessor: ETB RAM Write Pointer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rwp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rwp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rwp`] module"]
#[doc(alias = "RWP")]
pub type Rwp = crate::Reg<rwp::RwpSpec>;
#[doc = "ETB RAM Write Pointer Register"]
pub mod rwp;
#[doc = "TRG (rw) register accessor: ETB Trigger Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`trg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trg`] module"]
#[doc(alias = "TRG")]
pub type Trg = crate::Reg<trg::TrgSpec>;
#[doc = "ETB Trigger Counter Register"]
pub mod trg;
#[doc = "CTL (rw) register accessor: ETB Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`] module"]
#[doc(alias = "CTL")]
pub type Ctl = crate::Reg<ctl::CtlSpec>;
#[doc = "ETB Control Register"]
pub mod ctl;
#[doc = "RWD (rw) register accessor: ETB RAM Write Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rwd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rwd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rwd`] module"]
#[doc(alias = "RWD")]
pub type Rwd = crate::Reg<rwd::RwdSpec>;
#[doc = "ETB RAM Write Data Register"]
pub mod rwd;
#[doc = "FFSR (r) register accessor: ETB Formatter and Flush Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ffsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ffsr`] module"]
#[doc(alias = "FFSR")]
pub type Ffsr = crate::Reg<ffsr::FfsrSpec>;
#[doc = "ETB Formatter and Flush Status Register"]
pub mod ffsr;
#[doc = "FFCR (rw) register accessor: ETB Formatter and Flush Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ffcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ffcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ffcr`] module"]
#[doc(alias = "FFCR")]
pub type Ffcr = crate::Reg<ffcr::FfcrSpec>;
#[doc = "ETB Formatter and Flush Control Register"]
pub mod ffcr;
#[doc = "ITMISCOP0 (w) register accessor: Integration Test Miscellaneous Output Register 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itmiscop0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itmiscop0`] module"]
#[doc(alias = "ITMISCOP0")]
pub type Itmiscop0 = crate::Reg<itmiscop0::Itmiscop0Spec>;
#[doc = "Integration Test Miscellaneous Output Register 0"]
pub mod itmiscop0;
#[doc = "ITTRFLINACK (w) register accessor: Integration Test Trigger In and Flush In Acknowledge Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ittrflinack::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ittrflinack`] module"]
#[doc(alias = "ITTRFLINACK")]
pub type Ittrflinack = crate::Reg<ittrflinack::IttrflinackSpec>;
#[doc = "Integration Test Trigger In and Flush In Acknowledge Register"]
pub mod ittrflinack;
#[doc = "ITTRFLIN (r) register accessor: Integration Test Trigger In and Flush In Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ittrflin::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ittrflin`] module"]
#[doc(alias = "ITTRFLIN")]
pub type Ittrflin = crate::Reg<ittrflin::IttrflinSpec>;
#[doc = "Integration Test Trigger In and Flush In Register"]
pub mod ittrflin;
#[doc = "ITATBDATA0 (r) register accessor: Integration Test ATB Data Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`itatbdata0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itatbdata0`] module"]
#[doc(alias = "ITATBDATA0")]
pub type Itatbdata0 = crate::Reg<itatbdata0::Itatbdata0Spec>;
#[doc = "Integration Test ATB Data Register 0"]
pub mod itatbdata0;
#[doc = "ITATBCTR2 (w) register accessor: Integration Test ATB Control Register 2\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itatbctr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itatbctr2`] module"]
#[doc(alias = "ITATBCTR2")]
pub type Itatbctr2 = crate::Reg<itatbctr2::Itatbctr2Spec>;
#[doc = "Integration Test ATB Control Register 2"]
pub mod itatbctr2;
#[doc = "ITATBCTR1 (r) register accessor: Integration Test ATB Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`itatbctr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itatbctr1`] module"]
#[doc(alias = "ITATBCTR1")]
pub type Itatbctr1 = crate::Reg<itatbctr1::Itatbctr1Spec>;
#[doc = "Integration Test ATB Control Register 1"]
pub mod itatbctr1;
#[doc = "ITATBCTR0 (r) register accessor: Integration Test ATB Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`itatbctr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itatbctr0`] module"]
#[doc(alias = "ITATBCTR0")]
pub type Itatbctr0 = crate::Reg<itatbctr0::Itatbctr0Spec>;
#[doc = "Integration Test ATB Control Register 0"]
pub mod itatbctr0;
#[doc = "ITCTRL (rw) register accessor: Integration Mode Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`itctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itctrl`] module"]
#[doc(alias = "ITCTRL")]
pub type Itctrl = crate::Reg<itctrl::ItctrlSpec>;
#[doc = "Integration Mode Control Register"]
pub mod itctrl;
#[doc = "CLAIMSET (rw) register accessor: Claim Tag Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`claimset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`claimset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@claimset`] module"]
#[doc(alias = "CLAIMSET")]
pub type Claimset = crate::Reg<claimset::ClaimsetSpec>;
#[doc = "Claim Tag Set Register"]
pub mod claimset;
#[doc = "CLAIMCLR (rw) register accessor: Claim Tag Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`claimclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`claimclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@claimclr`] module"]
#[doc(alias = "CLAIMCLR")]
pub type Claimclr = crate::Reg<claimclr::ClaimclrSpec>;
#[doc = "Claim Tag Clear Register"]
pub mod claimclr;
#[doc = "LAR (w) register accessor: Lock Access Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lar::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lar`] module"]
#[doc(alias = "LAR")]
pub type Lar = crate::Reg<lar::LarSpec>;
#[doc = "Lock Access Register"]
pub mod lar;
#[doc = "LSR (r) register accessor: Lock Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lsr`] module"]
#[doc(alias = "LSR")]
pub type Lsr = crate::Reg<lsr::LsrSpec>;
#[doc = "Lock Status Register"]
pub mod lsr;
#[doc = "AUTHSTATUS (r) register accessor: Authentication Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`authstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@authstatus`] module"]
#[doc(alias = "AUTHSTATUS")]
pub type Authstatus = crate::Reg<authstatus::AuthstatusSpec>;
#[doc = "Authentication Status Register"]
pub mod authstatus;
#[doc = "DEVID (r) register accessor: Device Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`devid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devid`] module"]
#[doc(alias = "DEVID")]
pub type Devid = crate::Reg<devid::DevidSpec>;
#[doc = "Device Configuration Register"]
pub mod devid;
#[doc = "DEVTYPE (r) register accessor: Device Type Identifier Register\n\nYou can [`read`](crate::Reg::read) this register and get [`devtype::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devtype`] module"]
#[doc(alias = "DEVTYPE")]
pub type Devtype = crate::Reg<devtype::DevtypeSpec>;
#[doc = "Device Type Identifier Register"]
pub mod devtype;
#[doc = "PERIPHID4 (r) register accessor: Peripheral ID4 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`periphid4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@periphid4`] module"]
#[doc(alias = "PERIPHID4")]
pub type Periphid4 = crate::Reg<periphid4::Periphid4Spec>;
#[doc = "Peripheral ID4 Register"]
pub mod periphid4;
#[doc = "PERIPHID0 (r) register accessor: Peripheral ID0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`periphid0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@periphid0`] module"]
#[doc(alias = "PERIPHID0")]
pub type Periphid0 = crate::Reg<periphid0::Periphid0Spec>;
#[doc = "Peripheral ID0 Register"]
pub mod periphid0;
#[doc = "PERIPHID1 (r) register accessor: Peripheral ID1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`periphid1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@periphid1`] module"]
#[doc(alias = "PERIPHID1")]
pub type Periphid1 = crate::Reg<periphid1::Periphid1Spec>;
#[doc = "Peripheral ID1 Register"]
pub mod periphid1;
#[doc = "PERIPHID2 (r) register accessor: Peripheral ID2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`periphid2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@periphid2`] module"]
#[doc(alias = "PERIPHID2")]
pub type Periphid2 = crate::Reg<periphid2::Periphid2Spec>;
#[doc = "Peripheral ID2 Register"]
pub mod periphid2;
#[doc = "PERIPHID3 (r) register accessor: Peripheral ID3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`periphid3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@periphid3`] module"]
#[doc(alias = "PERIPHID3")]
pub type Periphid3 = crate::Reg<periphid3::Periphid3Spec>;
#[doc = "Peripheral ID3 Register"]
pub mod periphid3;
#[doc = "COMPID0 (r) register accessor: Component ID0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`compid0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@compid0`] module"]
#[doc(alias = "COMPID0")]
pub type Compid0 = crate::Reg<compid0::Compid0Spec>;
#[doc = "Component ID0 Register"]
pub mod compid0;
#[doc = "COMPID1 (r) register accessor: Component ID1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`compid1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@compid1`] module"]
#[doc(alias = "COMPID1")]
pub type Compid1 = crate::Reg<compid1::Compid1Spec>;
#[doc = "Component ID1 Register"]
pub mod compid1;
#[doc = "COMPID2 (r) register accessor: Component ID2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`compid2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@compid2`] module"]
#[doc(alias = "COMPID2")]
pub type Compid2 = crate::Reg<compid2::Compid2Spec>;
#[doc = "Component ID2 Register"]
pub mod compid2;
#[doc = "COMPID3 (r) register accessor: Component ID3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`compid3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@compid3`] module"]
#[doc(alias = "COMPID3")]
pub type Compid3 = crate::Reg<compid3::Compid3Spec>;
#[doc = "Component ID3 Register"]
pub mod compid3;
