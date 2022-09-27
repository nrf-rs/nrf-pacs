#[doc = r"Register block"]
#[repr(C)]
pub struct HALTED {
    #[doc = "0x00..0x20 - Description collection\\[n\\]: IN endpoint halted status. Can be used as is as response to a GetStatus() request to endpoint."]
    pub epin: [EPIN; 8],
    _reserved1: [u8; 0x04],
    #[doc = "0x24..0x44 - Description collection\\[n\\]: OUT endpoint halted status. Can be used as is as response to a GetStatus() request to endpoint."]
    pub epout: [EPOUT; 8],
}
#[doc = "EPIN (r) register accessor: an alias for `Reg<EPIN_SPEC>`"]
pub type EPIN = crate::Reg<epin::EPIN_SPEC>;
#[doc = "Description collection\\[n\\]: IN endpoint halted status. Can be used as is as response to a GetStatus() request to endpoint."]
pub mod epin;
#[doc = "EPOUT (r) register accessor: an alias for `Reg<EPOUT_SPEC>`"]
pub type EPOUT = crate::Reg<epout::EPOUT_SPEC>;
#[doc = "Description collection\\[n\\]: OUT endpoint halted status. Can be used as is as response to a GetStatus() request to endpoint."]
pub mod epout;
