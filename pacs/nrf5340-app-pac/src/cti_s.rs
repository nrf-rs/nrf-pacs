#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CTI Control register"]
    pub cticontrol: crate::Reg<cticontrol::CTICONTROL_SPEC>,
    _reserved1: [u8; 0x0c],
    #[doc = "0x10 - CTI Interrupt Acknowledge register"]
    pub ctiintack: crate::Reg<ctiintack::CTIINTACK_SPEC>,
    #[doc = "0x14 - CTI Application Trigger Set register"]
    pub ctiappset: crate::Reg<ctiappset::CTIAPPSET_SPEC>,
    #[doc = "0x18 - CTI Application Trigger Clear register"]
    pub ctiappclear: crate::Reg<ctiappclear::CTIAPPCLEAR_SPEC>,
    #[doc = "0x1c - CTI Application Pulse register"]
    pub ctiapppulse: crate::Reg<ctiapppulse::CTIAPPPULSE_SPEC>,
    #[doc = "0x20..0x40 - Description collection: CTI Trigger input"]
    pub ctiinen: [crate::Reg<ctiinen::CTIINEN_SPEC>; 8],
    _reserved6: [u8; 0x60],
    #[doc = "0xa0..0xc0 - Description collection: CTI Trigger output"]
    pub ctiouten: [crate::Reg<ctiouten::CTIOUTEN_SPEC>; 8],
    _reserved7: [u8; 0x70],
    #[doc = "0x130 - CTI Trigger In Status register"]
    pub ctitriginstatus: crate::Reg<ctitriginstatus::CTITRIGINSTATUS_SPEC>,
    #[doc = "0x134 - CTI Trigger Out Status register"]
    pub ctitrigoutstatus: crate::Reg<ctitrigoutstatus::CTITRIGOUTSTATUS_SPEC>,
    #[doc = "0x138 - CTI Channel In Status register"]
    pub ctichinstatus: crate::Reg<ctichinstatus::CTICHINSTATUS_SPEC>,
    _reserved10: [u8; 0x04],
    #[doc = "0x140 - Enable CTI Channel Gate register"]
    pub ctigate: crate::Reg<ctigate::CTIGATE_SPEC>,
    _reserved11: [u8; 0x0e78],
    #[doc = "0xfbc - Device Architecture register"]
    pub devarch: crate::Reg<devarch::DEVARCH_SPEC>,
    _reserved12: [u8; 0x08],
    #[doc = "0xfc8 - Device Configuration register"]
    pub devid: crate::Reg<devid::DEVID_SPEC>,
    #[doc = "0xfcc - Device Type Identifier register"]
    pub devtype: crate::Reg<devtype::DEVTYPE_SPEC>,
    #[doc = "0xfd0 - Peripheral ID4 Register"]
    pub pidr4: crate::Reg<pidr4::PIDR4_SPEC>,
    #[doc = "0xfd4 - Peripheral ID5 register"]
    pub pidr5: crate::Reg<pidr5::PIDR5_SPEC>,
    #[doc = "0xfd8 - Peripheral ID6 register"]
    pub pidr6: crate::Reg<pidr6::PIDR6_SPEC>,
    #[doc = "0xfdc - Peripheral ID7 register"]
    pub pidr7: crate::Reg<pidr7::PIDR7_SPEC>,
    #[doc = "0xfe0 - Peripheral ID0 Register"]
    pub pidr0: crate::Reg<pidr0::PIDR0_SPEC>,
    #[doc = "0xfe4 - Peripheral ID1 Register"]
    pub pidr1: crate::Reg<pidr1::PIDR1_SPEC>,
    #[doc = "0xfe8 - Peripheral ID2 Register"]
    pub pidr2: crate::Reg<pidr2::PIDR2_SPEC>,
    #[doc = "0xfec - Peripheral ID3 Register"]
    pub pidr3: crate::Reg<pidr3::PIDR3_SPEC>,
    #[doc = "0xff0 - Component ID0 Register"]
    pub cidr0: crate::Reg<cidr0::CIDR0_SPEC>,
    #[doc = "0xff4 - Component ID1 Register"]
    pub cidr1: crate::Reg<cidr1::CIDR1_SPEC>,
    #[doc = "0xff8 - Component ID2 Register"]
    pub cidr2: crate::Reg<cidr2::CIDR2_SPEC>,
    #[doc = "0xffc - Component ID3 Register"]
    pub cidr3: crate::Reg<cidr3::CIDR3_SPEC>,
}
#[doc = "CTICONTROL register accessor: an alias for `Reg<CTICONTROL_SPEC>`"]
pub type CTICONTROL = crate::Reg<cticontrol::CTICONTROL_SPEC>;
#[doc = "CTI Control register"]
pub mod cticontrol;
#[doc = "CTIINTACK register accessor: an alias for `Reg<CTIINTACK_SPEC>`"]
pub type CTIINTACK = crate::Reg<ctiintack::CTIINTACK_SPEC>;
#[doc = "CTI Interrupt Acknowledge register"]
pub mod ctiintack;
#[doc = "CTIAPPSET register accessor: an alias for `Reg<CTIAPPSET_SPEC>`"]
pub type CTIAPPSET = crate::Reg<ctiappset::CTIAPPSET_SPEC>;
#[doc = "CTI Application Trigger Set register"]
pub mod ctiappset;
#[doc = "CTIAPPCLEAR register accessor: an alias for `Reg<CTIAPPCLEAR_SPEC>`"]
pub type CTIAPPCLEAR = crate::Reg<ctiappclear::CTIAPPCLEAR_SPEC>;
#[doc = "CTI Application Trigger Clear register"]
pub mod ctiappclear;
#[doc = "CTIAPPPULSE register accessor: an alias for `Reg<CTIAPPPULSE_SPEC>`"]
pub type CTIAPPPULSE = crate::Reg<ctiapppulse::CTIAPPPULSE_SPEC>;
#[doc = "CTI Application Pulse register"]
pub mod ctiapppulse;
#[doc = "CTIINEN register accessor: an alias for `Reg<CTIINEN_SPEC>`"]
pub type CTIINEN = crate::Reg<ctiinen::CTIINEN_SPEC>;
#[doc = "Description collection: CTI Trigger input"]
pub mod ctiinen;
#[doc = "CTIOUTEN register accessor: an alias for `Reg<CTIOUTEN_SPEC>`"]
pub type CTIOUTEN = crate::Reg<ctiouten::CTIOUTEN_SPEC>;
#[doc = "Description collection: CTI Trigger output"]
pub mod ctiouten;
#[doc = "CTITRIGINSTATUS register accessor: an alias for `Reg<CTITRIGINSTATUS_SPEC>`"]
pub type CTITRIGINSTATUS = crate::Reg<ctitriginstatus::CTITRIGINSTATUS_SPEC>;
#[doc = "CTI Trigger In Status register"]
pub mod ctitriginstatus;
#[doc = "CTITRIGOUTSTATUS register accessor: an alias for `Reg<CTITRIGOUTSTATUS_SPEC>`"]
pub type CTITRIGOUTSTATUS = crate::Reg<ctitrigoutstatus::CTITRIGOUTSTATUS_SPEC>;
#[doc = "CTI Trigger Out Status register"]
pub mod ctitrigoutstatus;
#[doc = "CTICHINSTATUS register accessor: an alias for `Reg<CTICHINSTATUS_SPEC>`"]
pub type CTICHINSTATUS = crate::Reg<ctichinstatus::CTICHINSTATUS_SPEC>;
#[doc = "CTI Channel In Status register"]
pub mod ctichinstatus;
#[doc = "CTIGATE register accessor: an alias for `Reg<CTIGATE_SPEC>`"]
pub type CTIGATE = crate::Reg<ctigate::CTIGATE_SPEC>;
#[doc = "Enable CTI Channel Gate register"]
pub mod ctigate;
#[doc = "DEVARCH register accessor: an alias for `Reg<DEVARCH_SPEC>`"]
pub type DEVARCH = crate::Reg<devarch::DEVARCH_SPEC>;
#[doc = "Device Architecture register"]
pub mod devarch;
#[doc = "DEVID register accessor: an alias for `Reg<DEVID_SPEC>`"]
pub type DEVID = crate::Reg<devid::DEVID_SPEC>;
#[doc = "Device Configuration register"]
pub mod devid;
#[doc = "DEVTYPE register accessor: an alias for `Reg<DEVTYPE_SPEC>`"]
pub type DEVTYPE = crate::Reg<devtype::DEVTYPE_SPEC>;
#[doc = "Device Type Identifier register"]
pub mod devtype;
#[doc = "PIDR4 register accessor: an alias for `Reg<PIDR4_SPEC>`"]
pub type PIDR4 = crate::Reg<pidr4::PIDR4_SPEC>;
#[doc = "Peripheral ID4 Register"]
pub mod pidr4;
#[doc = "PIDR5 register accessor: an alias for `Reg<PIDR5_SPEC>`"]
pub type PIDR5 = crate::Reg<pidr5::PIDR5_SPEC>;
#[doc = "Peripheral ID5 register"]
pub mod pidr5;
#[doc = "PIDR6 register accessor: an alias for `Reg<PIDR6_SPEC>`"]
pub type PIDR6 = crate::Reg<pidr6::PIDR6_SPEC>;
#[doc = "Peripheral ID6 register"]
pub mod pidr6;
#[doc = "PIDR7 register accessor: an alias for `Reg<PIDR7_SPEC>`"]
pub type PIDR7 = crate::Reg<pidr7::PIDR7_SPEC>;
#[doc = "Peripheral ID7 register"]
pub mod pidr7;
#[doc = "PIDR0 register accessor: an alias for `Reg<PIDR0_SPEC>`"]
pub type PIDR0 = crate::Reg<pidr0::PIDR0_SPEC>;
#[doc = "Peripheral ID0 Register"]
pub mod pidr0;
#[doc = "PIDR1 register accessor: an alias for `Reg<PIDR1_SPEC>`"]
pub type PIDR1 = crate::Reg<pidr1::PIDR1_SPEC>;
#[doc = "Peripheral ID1 Register"]
pub mod pidr1;
#[doc = "PIDR2 register accessor: an alias for `Reg<PIDR2_SPEC>`"]
pub type PIDR2 = crate::Reg<pidr2::PIDR2_SPEC>;
#[doc = "Peripheral ID2 Register"]
pub mod pidr2;
#[doc = "PIDR3 register accessor: an alias for `Reg<PIDR3_SPEC>`"]
pub type PIDR3 = crate::Reg<pidr3::PIDR3_SPEC>;
#[doc = "Peripheral ID3 Register"]
pub mod pidr3;
#[doc = "CIDR0 register accessor: an alias for `Reg<CIDR0_SPEC>`"]
pub type CIDR0 = crate::Reg<cidr0::CIDR0_SPEC>;
#[doc = "Component ID0 Register"]
pub mod cidr0;
#[doc = "CIDR1 register accessor: an alias for `Reg<CIDR1_SPEC>`"]
pub type CIDR1 = crate::Reg<cidr1::CIDR1_SPEC>;
#[doc = "Component ID1 Register"]
pub mod cidr1;
#[doc = "CIDR2 register accessor: an alias for `Reg<CIDR2_SPEC>`"]
pub type CIDR2 = crate::Reg<cidr2::CIDR2_SPEC>;
#[doc = "Component ID2 Register"]
pub mod cidr2;
#[doc = "CIDR3 register accessor: an alias for `Reg<CIDR3_SPEC>`"]
pub type CIDR3 = crate::Reg<cidr3::CIDR3_SPEC>;
#[doc = "Component ID3 Register"]
pub mod cidr3;
