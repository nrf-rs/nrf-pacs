#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0200],
    info: Info,
    _reserved1: [u8; 0x54],
    er: [Er; 4],
    ir: [Ir; 4],
    deviceaddrtype: Deviceaddrtype,
    deviceaddr: [Deviceaddr; 2],
    _reserved5: [u8; 0x54],
    trimcnf: [Trimcnf; 32],
}
impl RegisterBlock {
    #[doc = "0x200..0x22c - Device info"]
    #[inline(always)]
    pub const fn info(&self) -> &Info {
        &self.info
    }
    #[doc = "0x280..0x290 - Description collection: Encryption Root, word n"]
    #[inline(always)]
    pub const fn er(&self, n: usize) -> &Er {
        &self.er[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x280..0x290 - Description collection: Encryption Root, word n"]
    #[inline(always)]
    pub fn er_iter(&self) -> impl Iterator<Item = &Er> {
        self.er.iter()
    }
    #[doc = "0x290..0x2a0 - Description collection: Identity Root, word n"]
    #[inline(always)]
    pub const fn ir(&self, n: usize) -> &Ir {
        &self.ir[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x290..0x2a0 - Description collection: Identity Root, word n"]
    #[inline(always)]
    pub fn ir_iter(&self) -> impl Iterator<Item = &Ir> {
        self.ir.iter()
    }
    #[doc = "0x2a0 - Device address type"]
    #[inline(always)]
    pub const fn deviceaddrtype(&self) -> &Deviceaddrtype {
        &self.deviceaddrtype
    }
    #[doc = "0x2a4..0x2ac - Description collection: Device address n"]
    #[inline(always)]
    pub const fn deviceaddr(&self, n: usize) -> &Deviceaddr {
        &self.deviceaddr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2a4..0x2ac - Description collection: Device address n"]
    #[inline(always)]
    pub fn deviceaddr_iter(&self) -> impl Iterator<Item = &Deviceaddr> {
        self.deviceaddr.iter()
    }
    #[doc = "0x300..0x400 - Unspecified"]
    #[inline(always)]
    pub const fn trimcnf(&self, n: usize) -> &Trimcnf {
        &self.trimcnf[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x300..0x400 - Unspecified"]
    #[inline(always)]
    pub fn trimcnf_iter(&self) -> impl Iterator<Item = &Trimcnf> {
        self.trimcnf.iter()
    }
}
#[doc = "Device info"]
pub use self::info::Info;
#[doc = r"Cluster"]
#[doc = "Device info"]
pub mod info;
#[doc = "ER (r) register accessor: Description collection: Encryption Root, word n\n\nYou can [`read`](crate::Reg::read) this register and get [`er::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@er`] module"]
#[doc(alias = "ER")]
pub type Er = crate::Reg<er::ErSpec>;
#[doc = "Description collection: Encryption Root, word n"]
pub mod er;
#[doc = "IR (r) register accessor: Description collection: Identity Root, word n\n\nYou can [`read`](crate::Reg::read) this register and get [`ir::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ir`] module"]
#[doc(alias = "IR")]
pub type Ir = crate::Reg<ir::IrSpec>;
#[doc = "Description collection: Identity Root, word n"]
pub mod ir;
#[doc = "DEVICEADDRTYPE (r) register accessor: Device address type\n\nYou can [`read`](crate::Reg::read) this register and get [`deviceaddrtype::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deviceaddrtype`] module"]
#[doc(alias = "DEVICEADDRTYPE")]
pub type Deviceaddrtype = crate::Reg<deviceaddrtype::DeviceaddrtypeSpec>;
#[doc = "Device address type"]
pub mod deviceaddrtype;
#[doc = "DEVICEADDR (r) register accessor: Description collection: Device address n\n\nYou can [`read`](crate::Reg::read) this register and get [`deviceaddr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deviceaddr`] module"]
#[doc(alias = "DEVICEADDR")]
pub type Deviceaddr = crate::Reg<deviceaddr::DeviceaddrSpec>;
#[doc = "Description collection: Device address n"]
pub mod deviceaddr;
#[doc = "Unspecified"]
pub use self::trimcnf::Trimcnf;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod trimcnf;
