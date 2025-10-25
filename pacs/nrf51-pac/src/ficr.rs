#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    codepagesize: Codepagesize,
    codesize: Codesize,
    _reserved2: [u8; 0x10],
    clenr0: Clenr0,
    ppfc: Ppfc,
    _reserved4: [u8; 0x04],
    numramblock: Numramblock,
    _reserved_5_sizeramblock: [u8; 0x10],
    _reserved6: [u8; 0x14],
    configid: Configid,
    deviceid: [Deviceid; 2],
    _reserved8: [u8; 0x18],
    er: [Er; 4],
    ir: [Ir; 4],
    deviceaddrtype: Deviceaddrtype,
    deviceaddr: [Deviceaddr; 2],
    overrideen: Overrideen,
    nrf_1mbit: [Nrf1mbit; 5],
    _reserved14: [u8; 0x28],
    ble_1mbit: [Ble1mbit; 5],
}
impl RegisterBlock {
    #[doc = "0x10 - Code memory page size in bytes."]
    #[inline(always)]
    pub const fn codepagesize(&self) -> &Codepagesize {
        &self.codepagesize
    }
    #[doc = "0x14 - Code memory size in pages."]
    #[inline(always)]
    pub const fn codesize(&self) -> &Codesize {
        &self.codesize
    }
    #[doc = "0x28 - Length of code region 0 in bytes."]
    #[inline(always)]
    pub const fn clenr0(&self) -> &Clenr0 {
        &self.clenr0
    }
    #[doc = "0x2c - Pre-programmed factory code present."]
    #[inline(always)]
    pub const fn ppfc(&self) -> &Ppfc {
        &self.ppfc
    }
    #[doc = "0x34 - Number of individualy controllable RAM blocks."]
    #[inline(always)]
    pub const fn numramblock(&self) -> &Numramblock {
        &self.numramblock
    }
    #[doc = "0x38..0x48 - Deprecated array of size of RAM block in bytes. This name is kept for backward compatinility purposes. Use SIZERAMBLOCKS instead."]
    #[inline(always)]
    pub const fn sizeramblock(&self, n: usize) -> &Sizeramblock {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(56)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x38..0x48 - Deprecated array of size of RAM block in bytes. This name is kept for backward compatinility purposes. Use SIZERAMBLOCKS instead."]
    #[inline(always)]
    pub fn sizeramblock_iter(&self) -> impl Iterator<Item = &Sizeramblock> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(56)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x38 - Size of RAM blocks in bytes."]
    #[inline(always)]
    pub const fn sizeramblocks(&self) -> &Sizeramblocks {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(56).cast() }
    }
    #[doc = "0x5c - Configuration identifier."]
    #[inline(always)]
    pub const fn configid(&self) -> &Configid {
        &self.configid
    }
    #[doc = "0x60..0x68 - Device identifier."]
    #[inline(always)]
    pub const fn deviceid(&self, n: usize) -> &Deviceid {
        &self.deviceid[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x60..0x68 - Device identifier."]
    #[inline(always)]
    pub fn deviceid_iter(&self) -> impl Iterator<Item = &Deviceid> {
        self.deviceid.iter()
    }
    #[doc = "0x80..0x90 - Encryption root."]
    #[inline(always)]
    pub const fn er(&self, n: usize) -> &Er {
        &self.er[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0x90 - Encryption root."]
    #[inline(always)]
    pub fn er_iter(&self) -> impl Iterator<Item = &Er> {
        self.er.iter()
    }
    #[doc = "0x90..0xa0 - Identity root."]
    #[inline(always)]
    pub const fn ir(&self, n: usize) -> &Ir {
        &self.ir[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x90..0xa0 - Identity root."]
    #[inline(always)]
    pub fn ir_iter(&self) -> impl Iterator<Item = &Ir> {
        self.ir.iter()
    }
    #[doc = "0xa0 - Device address type."]
    #[inline(always)]
    pub const fn deviceaddrtype(&self) -> &Deviceaddrtype {
        &self.deviceaddrtype
    }
    #[doc = "0xa4..0xac - Device address."]
    #[inline(always)]
    pub const fn deviceaddr(&self, n: usize) -> &Deviceaddr {
        &self.deviceaddr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xa4..0xac - Device address."]
    #[inline(always)]
    pub fn deviceaddr_iter(&self) -> impl Iterator<Item = &Deviceaddr> {
        self.deviceaddr.iter()
    }
    #[doc = "0xac - Radio calibration override enable."]
    #[inline(always)]
    pub const fn overrideen(&self) -> &Overrideen {
        &self.overrideen
    }
    #[doc = "0xb0..0xc4 - Override values for the OVERRIDEn registers in RADIO for NRF_1Mbit mode."]
    #[inline(always)]
    pub const fn nrf_1mbit(&self, n: usize) -> &Nrf1mbit {
        &self.nrf_1mbit[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xb0..0xc4 - Override values for the OVERRIDEn registers in RADIO for NRF_1Mbit mode."]
    #[inline(always)]
    pub fn nrf_1mbit_iter(&self) -> impl Iterator<Item = &Nrf1mbit> {
        self.nrf_1mbit.iter()
    }
    #[doc = "0xec..0x100 - Override values for the OVERRIDEn registers in RADIO for BLE_1Mbit mode."]
    #[inline(always)]
    pub const fn ble_1mbit(&self, n: usize) -> &Ble1mbit {
        &self.ble_1mbit[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xec..0x100 - Override values for the OVERRIDEn registers in RADIO for BLE_1Mbit mode."]
    #[inline(always)]
    pub fn ble_1mbit_iter(&self) -> impl Iterator<Item = &Ble1mbit> {
        self.ble_1mbit.iter()
    }
}
#[doc = "CODEPAGESIZE (r) register accessor: Code memory page size in bytes.\n\nYou can [`read`](crate::Reg::read) this register and get [`codepagesize::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@codepagesize`] module"]
#[doc(alias = "CODEPAGESIZE")]
pub type Codepagesize = crate::Reg<codepagesize::CodepagesizeSpec>;
#[doc = "Code memory page size in bytes."]
pub mod codepagesize;
#[doc = "CODESIZE (r) register accessor: Code memory size in pages.\n\nYou can [`read`](crate::Reg::read) this register and get [`codesize::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@codesize`] module"]
#[doc(alias = "CODESIZE")]
pub type Codesize = crate::Reg<codesize::CodesizeSpec>;
#[doc = "Code memory size in pages."]
pub mod codesize;
#[doc = "CLENR0 (r) register accessor: Length of code region 0 in bytes.\n\nYou can [`read`](crate::Reg::read) this register and get [`clenr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clenr0`] module"]
#[doc(alias = "CLENR0")]
pub type Clenr0 = crate::Reg<clenr0::Clenr0Spec>;
#[doc = "Length of code region 0 in bytes."]
pub mod clenr0;
#[doc = "PPFC (r) register accessor: Pre-programmed factory code present.\n\nYou can [`read`](crate::Reg::read) this register and get [`ppfc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppfc`] module"]
#[doc(alias = "PPFC")]
pub type Ppfc = crate::Reg<ppfc::PpfcSpec>;
#[doc = "Pre-programmed factory code present."]
pub mod ppfc;
#[doc = "NUMRAMBLOCK (r) register accessor: Number of individualy controllable RAM blocks.\n\nYou can [`read`](crate::Reg::read) this register and get [`numramblock::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@numramblock`] module"]
#[doc(alias = "NUMRAMBLOCK")]
pub type Numramblock = crate::Reg<numramblock::NumramblockSpec>;
#[doc = "Number of individualy controllable RAM blocks."]
pub mod numramblock;
#[doc = "SIZERAMBLOCKS (r) register accessor: Size of RAM blocks in bytes.\n\nYou can [`read`](crate::Reg::read) this register and get [`sizeramblocks::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sizeramblocks`] module"]
#[doc(alias = "SIZERAMBLOCKS")]
pub type Sizeramblocks = crate::Reg<sizeramblocks::SizeramblocksSpec>;
#[doc = "Size of RAM blocks in bytes."]
pub mod sizeramblocks;
#[doc = "SIZERAMBLOCK (r) register accessor: Deprecated array of size of RAM block in bytes. This name is kept for backward compatinility purposes. Use SIZERAMBLOCKS instead.\n\nYou can [`read`](crate::Reg::read) this register and get [`sizeramblock::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sizeramblock`] module"]
#[doc(alias = "SIZERAMBLOCK")]
pub type Sizeramblock = crate::Reg<sizeramblock::SizeramblockSpec>;
#[doc = "Deprecated array of size of RAM block in bytes. This name is kept for backward compatinility purposes. Use SIZERAMBLOCKS instead."]
pub mod sizeramblock;
#[doc = "CONFIGID (r) register accessor: Configuration identifier.\n\nYou can [`read`](crate::Reg::read) this register and get [`configid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@configid`] module"]
#[doc(alias = "CONFIGID")]
pub type Configid = crate::Reg<configid::ConfigidSpec>;
#[doc = "Configuration identifier."]
pub mod configid;
#[doc = "DEVICEID (r) register accessor: Device identifier.\n\nYou can [`read`](crate::Reg::read) this register and get [`deviceid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deviceid`] module"]
#[doc(alias = "DEVICEID")]
pub type Deviceid = crate::Reg<deviceid::DeviceidSpec>;
#[doc = "Device identifier."]
pub mod deviceid;
#[doc = "ER (r) register accessor: Encryption root.\n\nYou can [`read`](crate::Reg::read) this register and get [`er::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@er`] module"]
#[doc(alias = "ER")]
pub type Er = crate::Reg<er::ErSpec>;
#[doc = "Encryption root."]
pub mod er;
#[doc = "IR (r) register accessor: Identity root.\n\nYou can [`read`](crate::Reg::read) this register and get [`ir::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ir`] module"]
#[doc(alias = "IR")]
pub type Ir = crate::Reg<ir::IrSpec>;
#[doc = "Identity root."]
pub mod ir;
#[doc = "DEVICEADDRTYPE (r) register accessor: Device address type.\n\nYou can [`read`](crate::Reg::read) this register and get [`deviceaddrtype::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deviceaddrtype`] module"]
#[doc(alias = "DEVICEADDRTYPE")]
pub type Deviceaddrtype = crate::Reg<deviceaddrtype::DeviceaddrtypeSpec>;
#[doc = "Device address type."]
pub mod deviceaddrtype;
#[doc = "DEVICEADDR (r) register accessor: Device address.\n\nYou can [`read`](crate::Reg::read) this register and get [`deviceaddr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deviceaddr`] module"]
#[doc(alias = "DEVICEADDR")]
pub type Deviceaddr = crate::Reg<deviceaddr::DeviceaddrSpec>;
#[doc = "Device address."]
pub mod deviceaddr;
#[doc = "OVERRIDEEN (r) register accessor: Radio calibration override enable.\n\nYou can [`read`](crate::Reg::read) this register and get [`overrideen::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@overrideen`] module"]
#[doc(alias = "OVERRIDEEN")]
pub type Overrideen = crate::Reg<overrideen::OverrideenSpec>;
#[doc = "Radio calibration override enable."]
pub mod overrideen;
#[doc = "NRF_1MBIT (r) register accessor: Override values for the OVERRIDEn registers in RADIO for NRF_1Mbit mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`nrf_1mbit::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nrf_1mbit`] module"]
#[doc(alias = "NRF_1MBIT")]
pub type Nrf1mbit = crate::Reg<nrf_1mbit::Nrf1mbitSpec>;
#[doc = "Override values for the OVERRIDEn registers in RADIO for NRF_1Mbit mode."]
pub mod nrf_1mbit;
#[doc = "BLE_1MBIT (r) register accessor: Override values for the OVERRIDEn registers in RADIO for BLE_1Mbit mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`ble_1mbit::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ble_1mbit`] module"]
#[doc(alias = "BLE_1MBIT")]
pub type Ble1mbit = crate::Reg<ble_1mbit::Ble1mbitSpec>;
#[doc = "Override values for the OVERRIDEn registers in RADIO for BLE_1Mbit mode."]
pub mod ble_1mbit;
