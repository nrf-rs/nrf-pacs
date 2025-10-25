#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cticontrol: Cticontrol,
    _reserved1: [u8; 0x0c],
    ctiintack: Ctiintack,
    ctiappset: Ctiappset,
    ctiappclear: Ctiappclear,
    ctiapppulse: Ctiapppulse,
    ctiinen: [Ctiinen; 8],
    _reserved6: [u8; 0x60],
    ctiouten: [Ctiouten; 8],
    _reserved7: [u8; 0x70],
    ctitriginstatus: Ctitriginstatus,
    ctitrigoutstatus: Ctitrigoutstatus,
    ctichinstatus: Ctichinstatus,
    _reserved10: [u8; 0x04],
    ctigate: Ctigate,
    _reserved11: [u8; 0x0e78],
    devarch: Devarch,
    _reserved12: [u8; 0x08],
    devid: Devid,
    devtype: Devtype,
    pidr4: Pidr4,
    pidr5: Pidr5,
    pidr6: Pidr6,
    pidr7: Pidr7,
    pidr0: Pidr0,
    pidr1: Pidr1,
    pidr2: Pidr2,
    pidr3: Pidr3,
    cidr0: Cidr0,
    cidr1: Cidr1,
    cidr2: Cidr2,
    cidr3: Cidr3,
}
impl RegisterBlock {
    #[doc = "0x00 - CTI Control register"]
    #[inline(always)]
    pub const fn cticontrol(&self) -> &Cticontrol {
        &self.cticontrol
    }
    #[doc = "0x10 - CTI Interrupt Acknowledge register"]
    #[inline(always)]
    pub const fn ctiintack(&self) -> &Ctiintack {
        &self.ctiintack
    }
    #[doc = "0x14 - CTI Application Trigger Set register"]
    #[inline(always)]
    pub const fn ctiappset(&self) -> &Ctiappset {
        &self.ctiappset
    }
    #[doc = "0x18 - CTI Application Trigger Clear register"]
    #[inline(always)]
    pub const fn ctiappclear(&self) -> &Ctiappclear {
        &self.ctiappclear
    }
    #[doc = "0x1c - CTI Application Pulse register"]
    #[inline(always)]
    pub const fn ctiapppulse(&self) -> &Ctiapppulse {
        &self.ctiapppulse
    }
    #[doc = "0x20..0x40 - Description collection: CTI Trigger input"]
    #[inline(always)]
    pub const fn ctiinen(&self, n: usize) -> &Ctiinen {
        &self.ctiinen[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x20..0x40 - Description collection: CTI Trigger input"]
    #[inline(always)]
    pub fn ctiinen_iter(&self) -> impl Iterator<Item = &Ctiinen> {
        self.ctiinen.iter()
    }
    #[doc = "0xa0..0xc0 - Description collection: CTI Trigger output"]
    #[inline(always)]
    pub const fn ctiouten(&self, n: usize) -> &Ctiouten {
        &self.ctiouten[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xa0..0xc0 - Description collection: CTI Trigger output"]
    #[inline(always)]
    pub fn ctiouten_iter(&self) -> impl Iterator<Item = &Ctiouten> {
        self.ctiouten.iter()
    }
    #[doc = "0x130 - CTI Trigger In Status register"]
    #[inline(always)]
    pub const fn ctitriginstatus(&self) -> &Ctitriginstatus {
        &self.ctitriginstatus
    }
    #[doc = "0x134 - CTI Trigger Out Status register"]
    #[inline(always)]
    pub const fn ctitrigoutstatus(&self) -> &Ctitrigoutstatus {
        &self.ctitrigoutstatus
    }
    #[doc = "0x138 - CTI Channel In Status register"]
    #[inline(always)]
    pub const fn ctichinstatus(&self) -> &Ctichinstatus {
        &self.ctichinstatus
    }
    #[doc = "0x140 - Enable CTI Channel Gate register"]
    #[inline(always)]
    pub const fn ctigate(&self) -> &Ctigate {
        &self.ctigate
    }
    #[doc = "0xfbc - Device Architecture register"]
    #[inline(always)]
    pub const fn devarch(&self) -> &Devarch {
        &self.devarch
    }
    #[doc = "0xfc8 - Device Configuration register"]
    #[inline(always)]
    pub const fn devid(&self) -> &Devid {
        &self.devid
    }
    #[doc = "0xfcc - Device Type Identifier register"]
    #[inline(always)]
    pub const fn devtype(&self) -> &Devtype {
        &self.devtype
    }
    #[doc = "0xfd0 - Peripheral ID4 Register"]
    #[inline(always)]
    pub const fn pidr4(&self) -> &Pidr4 {
        &self.pidr4
    }
    #[doc = "0xfd4 - Peripheral ID5 register"]
    #[inline(always)]
    pub const fn pidr5(&self) -> &Pidr5 {
        &self.pidr5
    }
    #[doc = "0xfd8 - Peripheral ID6 register"]
    #[inline(always)]
    pub const fn pidr6(&self) -> &Pidr6 {
        &self.pidr6
    }
    #[doc = "0xfdc - Peripheral ID7 register"]
    #[inline(always)]
    pub const fn pidr7(&self) -> &Pidr7 {
        &self.pidr7
    }
    #[doc = "0xfe0 - Peripheral ID0 Register"]
    #[inline(always)]
    pub const fn pidr0(&self) -> &Pidr0 {
        &self.pidr0
    }
    #[doc = "0xfe4 - Peripheral ID1 Register"]
    #[inline(always)]
    pub const fn pidr1(&self) -> &Pidr1 {
        &self.pidr1
    }
    #[doc = "0xfe8 - Peripheral ID2 Register"]
    #[inline(always)]
    pub const fn pidr2(&self) -> &Pidr2 {
        &self.pidr2
    }
    #[doc = "0xfec - Peripheral ID3 Register"]
    #[inline(always)]
    pub const fn pidr3(&self) -> &Pidr3 {
        &self.pidr3
    }
    #[doc = "0xff0 - Component ID0 Register"]
    #[inline(always)]
    pub const fn cidr0(&self) -> &Cidr0 {
        &self.cidr0
    }
    #[doc = "0xff4 - Component ID1 Register"]
    #[inline(always)]
    pub const fn cidr1(&self) -> &Cidr1 {
        &self.cidr1
    }
    #[doc = "0xff8 - Component ID2 Register"]
    #[inline(always)]
    pub const fn cidr2(&self) -> &Cidr2 {
        &self.cidr2
    }
    #[doc = "0xffc - Component ID3 Register"]
    #[inline(always)]
    pub const fn cidr3(&self) -> &Cidr3 {
        &self.cidr3
    }
}
#[doc = "CTICONTROL (rw) register accessor: CTI Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cticontrol::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cticontrol::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cticontrol`] module"]
#[doc(alias = "CTICONTROL")]
pub type Cticontrol = crate::Reg<cticontrol::CticontrolSpec>;
#[doc = "CTI Control register"]
pub mod cticontrol;
#[doc = "CTIINTACK (w) register accessor: CTI Interrupt Acknowledge register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctiintack::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctiintack`] module"]
#[doc(alias = "CTIINTACK")]
pub type Ctiintack = crate::Reg<ctiintack::CtiintackSpec>;
#[doc = "CTI Interrupt Acknowledge register"]
pub mod ctiintack;
#[doc = "CTIAPPSET (rw) register accessor: CTI Application Trigger Set register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctiappset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctiappset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctiappset`] module"]
#[doc(alias = "CTIAPPSET")]
pub type Ctiappset = crate::Reg<ctiappset::CtiappsetSpec>;
#[doc = "CTI Application Trigger Set register"]
pub mod ctiappset;
#[doc = "CTIAPPCLEAR (w) register accessor: CTI Application Trigger Clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctiappclear::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctiappclear`] module"]
#[doc(alias = "CTIAPPCLEAR")]
pub type Ctiappclear = crate::Reg<ctiappclear::CtiappclearSpec>;
#[doc = "CTI Application Trigger Clear register"]
pub mod ctiappclear;
#[doc = "CTIAPPPULSE (w) register accessor: CTI Application Pulse register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctiapppulse::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctiapppulse`] module"]
#[doc(alias = "CTIAPPPULSE")]
pub type Ctiapppulse = crate::Reg<ctiapppulse::CtiapppulseSpec>;
#[doc = "CTI Application Pulse register"]
pub mod ctiapppulse;
#[doc = "CTIINEN (rw) register accessor: Description collection: CTI Trigger input\n\nYou can [`read`](crate::Reg::read) this register and get [`ctiinen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctiinen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctiinen`] module"]
#[doc(alias = "CTIINEN")]
pub type Ctiinen = crate::Reg<ctiinen::CtiinenSpec>;
#[doc = "Description collection: CTI Trigger input"]
pub mod ctiinen;
#[doc = "CTIOUTEN (rw) register accessor: Description collection: CTI Trigger output\n\nYou can [`read`](crate::Reg::read) this register and get [`ctiouten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctiouten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctiouten`] module"]
#[doc(alias = "CTIOUTEN")]
pub type Ctiouten = crate::Reg<ctiouten::CtioutenSpec>;
#[doc = "Description collection: CTI Trigger output"]
pub mod ctiouten;
#[doc = "CTITRIGINSTATUS (r) register accessor: CTI Trigger In Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctitriginstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctitriginstatus`] module"]
#[doc(alias = "CTITRIGINSTATUS")]
pub type Ctitriginstatus = crate::Reg<ctitriginstatus::CtitriginstatusSpec>;
#[doc = "CTI Trigger In Status register"]
pub mod ctitriginstatus;
#[doc = "CTITRIGOUTSTATUS (r) register accessor: CTI Trigger Out Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctitrigoutstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctitrigoutstatus`] module"]
#[doc(alias = "CTITRIGOUTSTATUS")]
pub type Ctitrigoutstatus = crate::Reg<ctitrigoutstatus::CtitrigoutstatusSpec>;
#[doc = "CTI Trigger Out Status register"]
pub mod ctitrigoutstatus;
#[doc = "CTICHINSTATUS (r) register accessor: CTI Channel In Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctichinstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctichinstatus`] module"]
#[doc(alias = "CTICHINSTATUS")]
pub type Ctichinstatus = crate::Reg<ctichinstatus::CtichinstatusSpec>;
#[doc = "CTI Channel In Status register"]
pub mod ctichinstatus;
#[doc = "CTIGATE (rw) register accessor: Enable CTI Channel Gate register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctigate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctigate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctigate`] module"]
#[doc(alias = "CTIGATE")]
pub type Ctigate = crate::Reg<ctigate::CtigateSpec>;
#[doc = "Enable CTI Channel Gate register"]
pub mod ctigate;
#[doc = "DEVARCH (r) register accessor: Device Architecture register\n\nYou can [`read`](crate::Reg::read) this register and get [`devarch::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devarch`] module"]
#[doc(alias = "DEVARCH")]
pub type Devarch = crate::Reg<devarch::DevarchSpec>;
#[doc = "Device Architecture register"]
pub mod devarch;
#[doc = "DEVID (r) register accessor: Device Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`devid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devid`] module"]
#[doc(alias = "DEVID")]
pub type Devid = crate::Reg<devid::DevidSpec>;
#[doc = "Device Configuration register"]
pub mod devid;
#[doc = "DEVTYPE (r) register accessor: Device Type Identifier register\n\nYou can [`read`](crate::Reg::read) this register and get [`devtype::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devtype`] module"]
#[doc(alias = "DEVTYPE")]
pub type Devtype = crate::Reg<devtype::DevtypeSpec>;
#[doc = "Device Type Identifier register"]
pub mod devtype;
#[doc = "PIDR4 (r) register accessor: Peripheral ID4 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr4`] module"]
#[doc(alias = "PIDR4")]
pub type Pidr4 = crate::Reg<pidr4::Pidr4Spec>;
#[doc = "Peripheral ID4 Register"]
pub mod pidr4;
#[doc = "PIDR5 (r) register accessor: Peripheral ID5 register\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr5`] module"]
#[doc(alias = "PIDR5")]
pub type Pidr5 = crate::Reg<pidr5::Pidr5Spec>;
#[doc = "Peripheral ID5 register"]
pub mod pidr5;
#[doc = "PIDR6 (r) register accessor: Peripheral ID6 register\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr6`] module"]
#[doc(alias = "PIDR6")]
pub type Pidr6 = crate::Reg<pidr6::Pidr6Spec>;
#[doc = "Peripheral ID6 register"]
pub mod pidr6;
#[doc = "PIDR7 (r) register accessor: Peripheral ID7 register\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr7`] module"]
#[doc(alias = "PIDR7")]
pub type Pidr7 = crate::Reg<pidr7::Pidr7Spec>;
#[doc = "Peripheral ID7 register"]
pub mod pidr7;
#[doc = "PIDR0 (r) register accessor: Peripheral ID0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr0`] module"]
#[doc(alias = "PIDR0")]
pub type Pidr0 = crate::Reg<pidr0::Pidr0Spec>;
#[doc = "Peripheral ID0 Register"]
pub mod pidr0;
#[doc = "PIDR1 (r) register accessor: Peripheral ID1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr1`] module"]
#[doc(alias = "PIDR1")]
pub type Pidr1 = crate::Reg<pidr1::Pidr1Spec>;
#[doc = "Peripheral ID1 Register"]
pub mod pidr1;
#[doc = "PIDR2 (r) register accessor: Peripheral ID2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr2`] module"]
#[doc(alias = "PIDR2")]
pub type Pidr2 = crate::Reg<pidr2::Pidr2Spec>;
#[doc = "Peripheral ID2 Register"]
pub mod pidr2;
#[doc = "PIDR3 (r) register accessor: Peripheral ID3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr3`] module"]
#[doc(alias = "PIDR3")]
pub type Pidr3 = crate::Reg<pidr3::Pidr3Spec>;
#[doc = "Peripheral ID3 Register"]
pub mod pidr3;
#[doc = "CIDR0 (r) register accessor: Component ID0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cidr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cidr0`] module"]
#[doc(alias = "CIDR0")]
pub type Cidr0 = crate::Reg<cidr0::Cidr0Spec>;
#[doc = "Component ID0 Register"]
pub mod cidr0;
#[doc = "CIDR1 (r) register accessor: Component ID1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cidr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cidr1`] module"]
#[doc(alias = "CIDR1")]
pub type Cidr1 = crate::Reg<cidr1::Cidr1Spec>;
#[doc = "Component ID1 Register"]
pub mod cidr1;
#[doc = "CIDR2 (r) register accessor: Component ID2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cidr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cidr2`] module"]
#[doc(alias = "CIDR2")]
pub type Cidr2 = crate::Reg<cidr2::Cidr2Spec>;
#[doc = "Component ID2 Register"]
pub mod cidr2;
#[doc = "CIDR3 (r) register accessor: Component ID3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cidr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cidr3`] module"]
#[doc(alias = "CIDR3")]
pub type Cidr3 = crate::Reg<cidr3::Cidr3Spec>;
#[doc = "Component ID3 Register"]
pub mod cidr3;
