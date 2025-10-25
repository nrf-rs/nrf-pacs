#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "HALTED")]
pub struct Halted {
    epin: [Epin; 8],
    _reserved1: [u8; 0x04],
    epout: [Epout; 8],
}
impl Halted {
    #[doc = "0x00..0x20 - Description collection: IN endpoint halted status. Can be used as is as response to a GetStatus() request to endpoint."]
    #[inline(always)]
    pub const fn epin(&self, n: usize) -> &Epin {
        &self.epin[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x20 - Description collection: IN endpoint halted status. Can be used as is as response to a GetStatus() request to endpoint."]
    #[inline(always)]
    pub fn epin_iter(&self) -> impl Iterator<Item = &Epin> {
        self.epin.iter()
    }
    #[doc = "0x24..0x44 - Description collection: OUT endpoint halted status. Can be used as is as response to a GetStatus() request to endpoint."]
    #[inline(always)]
    pub const fn epout(&self, n: usize) -> &Epout {
        &self.epout[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x24..0x44 - Description collection: OUT endpoint halted status. Can be used as is as response to a GetStatus() request to endpoint."]
    #[inline(always)]
    pub fn epout_iter(&self) -> impl Iterator<Item = &Epout> {
        self.epout.iter()
    }
}
#[doc = "EPIN (r) register accessor: Description collection: IN endpoint halted status. Can be used as is as response to a GetStatus() request to endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`epin::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epin`] module"]
#[doc(alias = "EPIN")]
pub type Epin = crate::Reg<epin::EpinSpec>;
#[doc = "Description collection: IN endpoint halted status. Can be used as is as response to a GetStatus() request to endpoint."]
pub mod epin;
#[doc = "EPOUT (r) register accessor: Description collection: OUT endpoint halted status. Can be used as is as response to a GetStatus() request to endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`epout::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epout`] module"]
#[doc(alias = "EPOUT")]
pub type Epout = crate::Reg<epout::EpoutSpec>;
#[doc = "Description collection: OUT endpoint halted status. Can be used as is as response to a GetStatus() request to endpoint."]
pub mod epout;
