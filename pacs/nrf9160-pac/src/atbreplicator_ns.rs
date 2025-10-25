#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    idfilter0: Idfilter0,
    idfilter1: Idfilter1,
    _reserved2: [u8; 0x0ef0],
    itatbctr1: Itatbctr1,
    itatbctr0: Itatbctr0,
    itctrl: Itctrl,
    _reserved5: [u8; 0x9c],
    claimset: Claimset,
    claimclr: Claimclr,
    _reserved7: [u8; 0x08],
    lar: Lar,
    lsr: Lsr,
    authstatus: Authstatus,
    _reserved10: [u8; 0x0c],
    devid: Devid,
    devtype: Devtype,
    pidr4: Pidr4,
    _reserved13: [u8; 0x0c],
    pidr_0: Pidr0,
    pidr_1: Pidr1,
    pidr_2: Pidr2,
    pidr_3: Pidr3,
    cidr_0: Cidr0,
    cidr_1: Cidr1,
    cidr_2: Cidr2,
    cidr_3: Cidr3,
}
impl RegisterBlock {
    #[doc = "0x00 - The IDFILTER0 register enables the programming of ID filtering for master port 0."]
    #[inline(always)]
    pub const fn idfilter0(&self) -> &Idfilter0 {
        &self.idfilter0
    }
    #[doc = "0x04 - The IDFILTER1 register enables the programming of ID filtering for master port 1."]
    #[inline(always)]
    pub const fn idfilter1(&self) -> &Idfilter1 {
        &self.idfilter1
    }
    #[doc = "0xef8 - The ITATBCTR1 register returns the value of the atreadym0, atreadym1, and atvalids inputs in integration mode."]
    #[inline(always)]
    pub const fn itatbctr1(&self) -> &Itatbctr1 {
        &self.itatbctr1
    }
    #[doc = "0xefc - The ITATBCTR0 register controls the value of the atvalidm0, atvalidm1, and atreadys outputs in integration mode."]
    #[inline(always)]
    pub const fn itatbctr0(&self) -> &Itatbctr0 {
        &self.itatbctr0
    }
    #[doc = "0xf00 - The ITCTRL register enables the component to switch from a functional mode, which is the default behavior, to integration mode where the inputs and outputs of the component can be directly controlled for the purposes of integration testing and topology detection."]
    #[inline(always)]
    pub const fn itctrl(&self) -> &Itctrl {
        &self.itctrl
    }
    #[doc = "0xfa0 - Software can use the claim tag to coordinate application and debugger access to trace unit functionality. The claim tags have no effect on the operation of the component. The CLAIMSET register sets bits in the claim tag, and determines the number of claim bits implemented."]
    #[inline(always)]
    pub const fn claimset(&self) -> &Claimset {
        &self.claimset
    }
    #[doc = "0xfa4 - Software can use the claim tag to coordinate application and debugger access to trace unit functionality. The claim tags have no effect on the operation of the component. The CLAIMCLR register sets the bits in the claim tag to 0 and determines the current value of the claim tag."]
    #[inline(always)]
    pub const fn claimclr(&self) -> &Claimclr {
        &self.claimclr
    }
    #[doc = "0xfb0 - This is used to enable write access to device registers."]
    #[inline(always)]
    pub const fn lar(&self) -> &Lar {
        &self.lar
    }
    #[doc = "0xfb4 - This indicates the status of the lock control mechanism. This lock prevents accidental writes by code under debug. Accesses to the extended stimulus port registers are not affected by the lock mechanism. This register must always be present although there might not be any lock access control mechanism. The lock mechanism, where present and locked, must block write accesses to any control register, except the Lock Access Register. For most components this covers all registers except for the Lock Access Register."]
    #[inline(always)]
    pub const fn lsr(&self) -> &Lsr {
        &self.lsr
    }
    #[doc = "0xfb8 - Indicates the current level of tracing permitted by the system"]
    #[inline(always)]
    pub const fn authstatus(&self) -> &Authstatus {
        &self.authstatus
    }
    #[doc = "0xfc8 - Indicates the capabilities of the component."]
    #[inline(always)]
    pub const fn devid(&self) -> &Devid {
        &self.devid
    }
    #[doc = "0xfcc - The DEVTYPE register provides a debugger with information about the component when the Part Number field is not recognized. The debugger can then report this information."]
    #[inline(always)]
    pub const fn devtype(&self) -> &Devtype {
        &self.devtype
    }
    #[doc = "0xfd0 - Coresight peripheral identification registers."]
    #[inline(always)]
    pub const fn pidr4(&self) -> &Pidr4 {
        &self.pidr4
    }
    #[doc = "0xfe0 - Coresight peripheral identification registers."]
    #[inline(always)]
    pub const fn pidr_0(&self) -> &Pidr0 {
        &self.pidr_0
    }
    #[doc = "0xfe4 - Coresight peripheral identification registers."]
    #[inline(always)]
    pub const fn pidr_1(&self) -> &Pidr1 {
        &self.pidr_1
    }
    #[doc = "0xfe8 - Coresight peripheral identification registers."]
    #[inline(always)]
    pub const fn pidr_2(&self) -> &Pidr2 {
        &self.pidr_2
    }
    #[doc = "0xfec - Coresight peripheral identification registers."]
    #[inline(always)]
    pub const fn pidr_3(&self) -> &Pidr3 {
        &self.pidr_3
    }
    #[doc = "0xff0 - Coresight component identification registers."]
    #[inline(always)]
    pub const fn cidr_0(&self) -> &Cidr0 {
        &self.cidr_0
    }
    #[doc = "0xff4 - Coresight component identification registers."]
    #[inline(always)]
    pub const fn cidr_1(&self) -> &Cidr1 {
        &self.cidr_1
    }
    #[doc = "0xff8 - Coresight component identification registers."]
    #[inline(always)]
    pub const fn cidr_2(&self) -> &Cidr2 {
        &self.cidr_2
    }
    #[doc = "0xffc - Coresight component identification registers."]
    #[inline(always)]
    pub const fn cidr_3(&self) -> &Cidr3 {
        &self.cidr_3
    }
}
#[doc = "IDFILTER0 (rw) register accessor: The IDFILTER0 register enables the programming of ID filtering for master port 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`idfilter0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idfilter0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idfilter0`] module"]
#[doc(alias = "IDFILTER0")]
pub type Idfilter0 = crate::Reg<idfilter0::Idfilter0Spec>;
#[doc = "The IDFILTER0 register enables the programming of ID filtering for master port 0."]
pub mod idfilter0;
#[doc = "IDFILTER1 (rw) register accessor: The IDFILTER1 register enables the programming of ID filtering for master port 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`idfilter1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idfilter1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idfilter1`] module"]
#[doc(alias = "IDFILTER1")]
pub type Idfilter1 = crate::Reg<idfilter1::Idfilter1Spec>;
#[doc = "The IDFILTER1 register enables the programming of ID filtering for master port 1."]
pub mod idfilter1;
#[doc = "ITATBCTR1 (rw) register accessor: The ITATBCTR1 register returns the value of the atreadym0, atreadym1, and atvalids inputs in integration mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`itatbctr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itatbctr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itatbctr1`] module"]
#[doc(alias = "ITATBCTR1")]
pub type Itatbctr1 = crate::Reg<itatbctr1::Itatbctr1Spec>;
#[doc = "The ITATBCTR1 register returns the value of the atreadym0, atreadym1, and atvalids inputs in integration mode."]
pub mod itatbctr1;
#[doc = "ITATBCTR0 (rw) register accessor: The ITATBCTR0 register controls the value of the atvalidm0, atvalidm1, and atreadys outputs in integration mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`itatbctr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itatbctr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itatbctr0`] module"]
#[doc(alias = "ITATBCTR0")]
pub type Itatbctr0 = crate::Reg<itatbctr0::Itatbctr0Spec>;
#[doc = "The ITATBCTR0 register controls the value of the atvalidm0, atvalidm1, and atreadys outputs in integration mode."]
pub mod itatbctr0;
#[doc = "ITCTRL (rw) register accessor: The ITCTRL register enables the component to switch from a functional mode, which is the default behavior, to integration mode where the inputs and outputs of the component can be directly controlled for the purposes of integration testing and topology detection.\n\nYou can [`read`](crate::Reg::read) this register and get [`itctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itctrl`] module"]
#[doc(alias = "ITCTRL")]
pub type Itctrl = crate::Reg<itctrl::ItctrlSpec>;
#[doc = "The ITCTRL register enables the component to switch from a functional mode, which is the default behavior, to integration mode where the inputs and outputs of the component can be directly controlled for the purposes of integration testing and topology detection."]
pub mod itctrl;
#[doc = "CLAIMSET (rw) register accessor: Software can use the claim tag to coordinate application and debugger access to trace unit functionality. The claim tags have no effect on the operation of the component. The CLAIMSET register sets bits in the claim tag, and determines the number of claim bits implemented.\n\nYou can [`read`](crate::Reg::read) this register and get [`claimset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`claimset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@claimset`] module"]
#[doc(alias = "CLAIMSET")]
pub type Claimset = crate::Reg<claimset::ClaimsetSpec>;
#[doc = "Software can use the claim tag to coordinate application and debugger access to trace unit functionality. The claim tags have no effect on the operation of the component. The CLAIMSET register sets bits in the claim tag, and determines the number of claim bits implemented."]
pub mod claimset;
#[doc = "CLAIMCLR (rw) register accessor: Software can use the claim tag to coordinate application and debugger access to trace unit functionality. The claim tags have no effect on the operation of the component. The CLAIMCLR register sets the bits in the claim tag to 0 and determines the current value of the claim tag.\n\nYou can [`read`](crate::Reg::read) this register and get [`claimclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`claimclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@claimclr`] module"]
#[doc(alias = "CLAIMCLR")]
pub type Claimclr = crate::Reg<claimclr::ClaimclrSpec>;
#[doc = "Software can use the claim tag to coordinate application and debugger access to trace unit functionality. The claim tags have no effect on the operation of the component. The CLAIMCLR register sets the bits in the claim tag to 0 and determines the current value of the claim tag."]
pub mod claimclr;
#[doc = "LAR (rw) register accessor: This is used to enable write access to device registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lar`] module"]
#[doc(alias = "LAR")]
pub type Lar = crate::Reg<lar::LarSpec>;
#[doc = "This is used to enable write access to device registers."]
pub mod lar;
#[doc = "LSR (rw) register accessor: This indicates the status of the lock control mechanism. This lock prevents accidental writes by code under debug. Accesses to the extended stimulus port registers are not affected by the lock mechanism. This register must always be present although there might not be any lock access control mechanism. The lock mechanism, where present and locked, must block write accesses to any control register, except the Lock Access Register. For most components this covers all registers except for the Lock Access Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`lsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lsr`] module"]
#[doc(alias = "LSR")]
pub type Lsr = crate::Reg<lsr::LsrSpec>;
#[doc = "This indicates the status of the lock control mechanism. This lock prevents accidental writes by code under debug. Accesses to the extended stimulus port registers are not affected by the lock mechanism. This register must always be present although there might not be any lock access control mechanism. The lock mechanism, where present and locked, must block write accesses to any control register, except the Lock Access Register. For most components this covers all registers except for the Lock Access Register."]
pub mod lsr;
#[doc = "AUTHSTATUS (rw) register accessor: Indicates the current level of tracing permitted by the system\n\nYou can [`read`](crate::Reg::read) this register and get [`authstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`authstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@authstatus`] module"]
#[doc(alias = "AUTHSTATUS")]
pub type Authstatus = crate::Reg<authstatus::AuthstatusSpec>;
#[doc = "Indicates the current level of tracing permitted by the system"]
pub mod authstatus;
#[doc = "DEVID (r) register accessor: Indicates the capabilities of the component.\n\nYou can [`read`](crate::Reg::read) this register and get [`devid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devid`] module"]
#[doc(alias = "DEVID")]
pub type Devid = crate::Reg<devid::DevidSpec>;
#[doc = "Indicates the capabilities of the component."]
pub mod devid;
#[doc = "DEVTYPE (r) register accessor: The DEVTYPE register provides a debugger with information about the component when the Part Number field is not recognized. The debugger can then report this information.\n\nYou can [`read`](crate::Reg::read) this register and get [`devtype::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devtype`] module"]
#[doc(alias = "DEVTYPE")]
pub type Devtype = crate::Reg<devtype::DevtypeSpec>;
#[doc = "The DEVTYPE register provides a debugger with information about the component when the Part Number field is not recognized. The debugger can then report this information."]
pub mod devtype;
#[doc = "PIDR4 (rw) register accessor: Coresight peripheral identification registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pidr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr4`] module"]
#[doc(alias = "PIDR4")]
pub type Pidr4 = crate::Reg<pidr4::Pidr4Spec>;
#[doc = "Coresight peripheral identification registers."]
pub mod pidr4;
#[doc = "PIDR_0 (rw) register accessor: Coresight peripheral identification registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pidr_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr_0`] module"]
#[doc(alias = "PIDR_0")]
pub type Pidr0 = crate::Reg<pidr_0::Pidr0Spec>;
#[doc = "Coresight peripheral identification registers."]
pub mod pidr_0;
#[doc = "PIDR_1 (rw) register accessor: Coresight peripheral identification registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pidr_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr_1`] module"]
#[doc(alias = "PIDR_1")]
pub type Pidr1 = crate::Reg<pidr_1::Pidr1Spec>;
#[doc = "Coresight peripheral identification registers."]
pub mod pidr_1;
#[doc = "PIDR_2 (rw) register accessor: Coresight peripheral identification registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pidr_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr_2`] module"]
#[doc(alias = "PIDR_2")]
pub type Pidr2 = crate::Reg<pidr_2::Pidr2Spec>;
#[doc = "Coresight peripheral identification registers."]
pub mod pidr_2;
#[doc = "PIDR_3 (rw) register accessor: Coresight peripheral identification registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pidr_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr_3`] module"]
#[doc(alias = "PIDR_3")]
pub type Pidr3 = crate::Reg<pidr_3::Pidr3Spec>;
#[doc = "Coresight peripheral identification registers."]
pub mod pidr_3;
#[doc = "CIDR_0 (rw) register accessor: Coresight component identification registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`cidr_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cidr_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cidr_0`] module"]
#[doc(alias = "CIDR_0")]
pub type Cidr0 = crate::Reg<cidr_0::Cidr0Spec>;
#[doc = "Coresight component identification registers."]
pub mod cidr_0;
#[doc = "CIDR_1 (rw) register accessor: Coresight component identification registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`cidr_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cidr_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cidr_1`] module"]
#[doc(alias = "CIDR_1")]
pub type Cidr1 = crate::Reg<cidr_1::Cidr1Spec>;
#[doc = "Coresight component identification registers."]
pub mod cidr_1;
#[doc = "CIDR_2 (rw) register accessor: Coresight component identification registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`cidr_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cidr_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cidr_2`] module"]
#[doc(alias = "CIDR_2")]
pub type Cidr2 = crate::Reg<cidr_2::Cidr2Spec>;
#[doc = "Coresight component identification registers."]
pub mod cidr_2;
#[doc = "CIDR_3 (rw) register accessor: Coresight component identification registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`cidr_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cidr_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cidr_3`] module"]
#[doc(alias = "CIDR_3")]
pub type Cidr3 = crate::Reg<cidr_3::Cidr3Spec>;
#[doc = "Coresight component identification registers."]
pub mod cidr_3;
