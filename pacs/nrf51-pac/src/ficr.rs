#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    #[doc = "0x10 - Code memory page size in bytes."]
    pub codepagesize: CODEPAGESIZE,
    #[doc = "0x14 - Code memory size in pages."]
    pub codesize: CODESIZE,
    _reserved2: [u8; 0x10],
    #[doc = "0x28 - Length of code region 0 in bytes."]
    pub clenr0: CLENR0,
    #[doc = "0x2c - Pre-programmed factory code present."]
    pub ppfc: PPFC,
    _reserved4: [u8; 0x04],
    #[doc = "0x34 - Number of individualy controllable RAM blocks."]
    pub numramblock: NUMRAMBLOCK,
    _reserved_5_sizeramblock: [u8; 0x10],
    _reserved6: [u8; 0x14],
    #[doc = "0x5c - Configuration identifier."]
    pub configid: CONFIGID,
    #[doc = "0x60..0x68 - Device identifier."]
    pub deviceid: [DEVICEID; 2],
    _reserved8: [u8; 0x18],
    #[doc = "0x80..0x90 - Encryption root."]
    pub er: [ER; 4],
    #[doc = "0x90..0xa0 - Identity root."]
    pub ir: [IR; 4],
    #[doc = "0xa0 - Device address type."]
    pub deviceaddrtype: DEVICEADDRTYPE,
    #[doc = "0xa4..0xac - Device address."]
    pub deviceaddr: [DEVICEADDR; 2],
    #[doc = "0xac - Radio calibration override enable."]
    pub overrideen: OVERRIDEEN,
    #[doc = "0xb0..0xc4 - Override values for the OVERRIDEn registers in RADIO for NRF_1Mbit mode."]
    pub nrf_1mbit: [NRF_1MBIT; 5],
    _reserved14: [u8; 0x28],
    #[doc = "0xec..0x100 - Override values for the OVERRIDEn registers in RADIO for BLE_1Mbit mode."]
    pub ble_1mbit: [BLE_1MBIT; 5],
}
impl RegisterBlock {
    #[doc = "0x38..0x48 - Deprecated array of size of RAM block in bytes. This name is kept for backward compatinility purposes. Use SIZERAMBLOCKS instead."]
    #[inline(always)]
    pub fn sizeramblock(&self) -> &[SIZERAMBLOCK; 4] {
        unsafe { &*(((self as *const Self) as *const u8).add(56usize) as *const [SIZERAMBLOCK; 4]) }
    }
    #[doc = "0x38 - Size of RAM blocks in bytes."]
    #[inline(always)]
    pub fn sizeramblocks(&self) -> &SIZERAMBLOCKS {
        unsafe { &*(((self as *const Self) as *const u8).add(56usize) as *const SIZERAMBLOCKS) }
    }
}
#[doc = "CODEPAGESIZE (r) register accessor: an alias for `Reg<CODEPAGESIZE_SPEC>`"]
pub type CODEPAGESIZE = crate::Reg<codepagesize::CODEPAGESIZE_SPEC>;
#[doc = "Code memory page size in bytes."]
pub mod codepagesize;
#[doc = "CODESIZE (r) register accessor: an alias for `Reg<CODESIZE_SPEC>`"]
pub type CODESIZE = crate::Reg<codesize::CODESIZE_SPEC>;
#[doc = "Code memory size in pages."]
pub mod codesize;
#[doc = "CLENR0 (r) register accessor: an alias for `Reg<CLENR0_SPEC>`"]
pub type CLENR0 = crate::Reg<clenr0::CLENR0_SPEC>;
#[doc = "Length of code region 0 in bytes."]
pub mod clenr0;
#[doc = "PPFC (r) register accessor: an alias for `Reg<PPFC_SPEC>`"]
pub type PPFC = crate::Reg<ppfc::PPFC_SPEC>;
#[doc = "Pre-programmed factory code present."]
pub mod ppfc;
#[doc = "NUMRAMBLOCK (r) register accessor: an alias for `Reg<NUMRAMBLOCK_SPEC>`"]
pub type NUMRAMBLOCK = crate::Reg<numramblock::NUMRAMBLOCK_SPEC>;
#[doc = "Number of individualy controllable RAM blocks."]
pub mod numramblock;
#[doc = "SIZERAMBLOCKS (r) register accessor: an alias for `Reg<SIZERAMBLOCKS_SPEC>`"]
pub type SIZERAMBLOCKS = crate::Reg<sizeramblocks::SIZERAMBLOCKS_SPEC>;
#[doc = "Size of RAM blocks in bytes."]
pub mod sizeramblocks;
#[doc = "SIZERAMBLOCK (r) register accessor: an alias for `Reg<SIZERAMBLOCK_SPEC>`"]
pub type SIZERAMBLOCK = crate::Reg<sizeramblock::SIZERAMBLOCK_SPEC>;
#[doc = "Deprecated array of size of RAM block in bytes. This name is kept for backward compatinility purposes. Use SIZERAMBLOCKS instead."]
pub mod sizeramblock;
#[doc = "CONFIGID (r) register accessor: an alias for `Reg<CONFIGID_SPEC>`"]
pub type CONFIGID = crate::Reg<configid::CONFIGID_SPEC>;
#[doc = "Configuration identifier."]
pub mod configid;
#[doc = "DEVICEID (r) register accessor: an alias for `Reg<DEVICEID_SPEC>`"]
pub type DEVICEID = crate::Reg<deviceid::DEVICEID_SPEC>;
#[doc = "Device identifier."]
pub mod deviceid;
#[doc = "ER (r) register accessor: an alias for `Reg<ER_SPEC>`"]
pub type ER = crate::Reg<er::ER_SPEC>;
#[doc = "Encryption root."]
pub mod er;
#[doc = "IR (r) register accessor: an alias for `Reg<IR_SPEC>`"]
pub type IR = crate::Reg<ir::IR_SPEC>;
#[doc = "Identity root."]
pub mod ir;
#[doc = "DEVICEADDRTYPE (r) register accessor: an alias for `Reg<DEVICEADDRTYPE_SPEC>`"]
pub type DEVICEADDRTYPE = crate::Reg<deviceaddrtype::DEVICEADDRTYPE_SPEC>;
#[doc = "Device address type."]
pub mod deviceaddrtype;
#[doc = "DEVICEADDR (r) register accessor: an alias for `Reg<DEVICEADDR_SPEC>`"]
pub type DEVICEADDR = crate::Reg<deviceaddr::DEVICEADDR_SPEC>;
#[doc = "Device address."]
pub mod deviceaddr;
#[doc = "OVERRIDEEN (r) register accessor: an alias for `Reg<OVERRIDEEN_SPEC>`"]
pub type OVERRIDEEN = crate::Reg<overrideen::OVERRIDEEN_SPEC>;
#[doc = "Radio calibration override enable."]
pub mod overrideen;
#[doc = "NRF_1MBIT (r) register accessor: an alias for `Reg<NRF_1MBIT_SPEC>`"]
pub type NRF_1MBIT = crate::Reg<nrf_1mbit::NRF_1MBIT_SPEC>;
#[doc = "Override values for the OVERRIDEn registers in RADIO for NRF_1Mbit mode."]
pub mod nrf_1mbit;
#[doc = "BLE_1MBIT (r) register accessor: an alias for `Reg<BLE_1MBIT_SPEC>`"]
pub type BLE_1MBIT = crate::Reg<ble_1mbit::BLE_1MBIT_SPEC>;
#[doc = "Override values for the OVERRIDEn registers in RADIO for BLE_1Mbit mode."]
pub mod ble_1mbit;
