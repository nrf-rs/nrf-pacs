#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    codepagesize: Codepagesize,
    codesize: Codesize,
    _reserved2: [u8; 0x48],
    deviceid: [Deviceid; 2],
    _reserved3: [u8; 0x18],
    er: [Er; 4],
    ir: [Ir; 4],
    deviceaddrtype: Deviceaddrtype,
    deviceaddr: [Deviceaddr; 2],
    _reserved7: [u8; 0x54],
    info: Info,
    _reserved8: [u8; 0x02e4],
    temp: Temp,
    _reserved9: [u8; 0x08],
    nfc: Nfc,
}
impl RegisterBlock {
    #[doc = "0x10 - Code memory page size"]
    #[inline(always)]
    pub const fn codepagesize(&self) -> &Codepagesize {
        &self.codepagesize
    }
    #[doc = "0x14 - Code memory size"]
    #[inline(always)]
    pub const fn codesize(&self) -> &Codesize {
        &self.codesize
    }
    #[doc = "0x60..0x68 - Description collection\\[0\\]: Device identifier"]
    #[inline(always)]
    pub const fn deviceid(&self, n: usize) -> &Deviceid {
        &self.deviceid[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x60..0x68 - Description collection\\[0\\]: Device identifier"]
    #[inline(always)]
    pub fn deviceid_iter(&self) -> impl Iterator<Item = &Deviceid> {
        self.deviceid.iter()
    }
    #[doc = "0x80..0x90 - Description collection\\[0\\]: Encryption Root, word 0"]
    #[inline(always)]
    pub const fn er(&self, n: usize) -> &Er {
        &self.er[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0x90 - Description collection\\[0\\]: Encryption Root, word 0"]
    #[inline(always)]
    pub fn er_iter(&self) -> impl Iterator<Item = &Er> {
        self.er.iter()
    }
    #[doc = "0x90..0xa0 - Description collection\\[0\\]: Identity Root, word 0"]
    #[inline(always)]
    pub const fn ir(&self, n: usize) -> &Ir {
        &self.ir[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x90..0xa0 - Description collection\\[0\\]: Identity Root, word 0"]
    #[inline(always)]
    pub fn ir_iter(&self) -> impl Iterator<Item = &Ir> {
        self.ir.iter()
    }
    #[doc = "0xa0 - Device address type"]
    #[inline(always)]
    pub const fn deviceaddrtype(&self) -> &Deviceaddrtype {
        &self.deviceaddrtype
    }
    #[doc = "0xa4..0xac - Description collection\\[0\\]: Device address 0"]
    #[inline(always)]
    pub const fn deviceaddr(&self, n: usize) -> &Deviceaddr {
        &self.deviceaddr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xa4..0xac - Description collection\\[0\\]: Device address 0"]
    #[inline(always)]
    pub fn deviceaddr_iter(&self) -> impl Iterator<Item = &Deviceaddr> {
        self.deviceaddr.iter()
    }
    #[doc = "0x100..0x120 - Device info"]
    #[inline(always)]
    pub const fn info(&self) -> &Info {
        &self.info
    }
    #[doc = "0x404..0x448 - Registers storing factory TEMP module linearization coefficients"]
    #[inline(always)]
    pub const fn temp(&self) -> &Temp {
        &self.temp
    }
    #[doc = "0x450..0x460 - Unspecified"]
    #[inline(always)]
    pub const fn nfc(&self) -> &Nfc {
        &self.nfc
    }
}
#[doc = "CODEPAGESIZE (r) register accessor: Code memory page size\n\nYou can [`read`](crate::Reg::read) this register and get [`codepagesize::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@codepagesize`] module"]
#[doc(alias = "CODEPAGESIZE")]
pub type Codepagesize = crate::Reg<codepagesize::CodepagesizeSpec>;
#[doc = "Code memory page size"]
pub mod codepagesize;
#[doc = "CODESIZE (r) register accessor: Code memory size\n\nYou can [`read`](crate::Reg::read) this register and get [`codesize::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@codesize`] module"]
#[doc(alias = "CODESIZE")]
pub type Codesize = crate::Reg<codesize::CodesizeSpec>;
#[doc = "Code memory size"]
pub mod codesize;
#[doc = "DEVICEID (r) register accessor: Description collection\\[0\\]: Device identifier\n\nYou can [`read`](crate::Reg::read) this register and get [`deviceid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deviceid`] module"]
#[doc(alias = "DEVICEID")]
pub type Deviceid = crate::Reg<deviceid::DeviceidSpec>;
#[doc = "Description collection\\[0\\]: Device identifier"]
pub mod deviceid;
#[doc = "ER (r) register accessor: Description collection\\[0\\]: Encryption Root, word 0\n\nYou can [`read`](crate::Reg::read) this register and get [`er::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@er`] module"]
#[doc(alias = "ER")]
pub type Er = crate::Reg<er::ErSpec>;
#[doc = "Description collection\\[0\\]: Encryption Root, word 0"]
pub mod er;
#[doc = "IR (r) register accessor: Description collection\\[0\\]: Identity Root, word 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ir::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ir`] module"]
#[doc(alias = "IR")]
pub type Ir = crate::Reg<ir::IrSpec>;
#[doc = "Description collection\\[0\\]: Identity Root, word 0"]
pub mod ir;
#[doc = "DEVICEADDRTYPE (r) register accessor: Device address type\n\nYou can [`read`](crate::Reg::read) this register and get [`deviceaddrtype::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deviceaddrtype`] module"]
#[doc(alias = "DEVICEADDRTYPE")]
pub type Deviceaddrtype = crate::Reg<deviceaddrtype::DeviceaddrtypeSpec>;
#[doc = "Device address type"]
pub mod deviceaddrtype;
#[doc = "DEVICEADDR (r) register accessor: Description collection\\[0\\]: Device address 0\n\nYou can [`read`](crate::Reg::read) this register and get [`deviceaddr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deviceaddr`] module"]
#[doc(alias = "DEVICEADDR")]
pub type Deviceaddr = crate::Reg<deviceaddr::DeviceaddrSpec>;
#[doc = "Description collection\\[0\\]: Device address 0"]
pub mod deviceaddr;
#[doc = "Device info"]
pub use self::info::Info;
#[doc = r"Cluster"]
#[doc = "Device info"]
pub mod info;
#[doc = "Registers storing factory TEMP module linearization coefficients"]
pub use self::temp::Temp;
#[doc = r"Cluster"]
#[doc = "Registers storing factory TEMP module linearization coefficients"]
pub mod temp;
#[doc = "Unspecified"]
pub use self::nfc::Nfc;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod nfc;
