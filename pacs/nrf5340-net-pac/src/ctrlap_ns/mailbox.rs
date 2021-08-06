#[doc = "RXDATA register accessor: an alias for `Reg<RXDATA_SPEC>`"]
pub type RXDATA = crate::Reg<rxdata::RXDATA_SPEC>;
#[doc = "Data sent from the debugger to the CPU."]
pub mod rxdata;
#[doc = "RXSTATUS register accessor: an alias for `Reg<RXSTATUS_SPEC>`"]
pub type RXSTATUS = crate::Reg<rxstatus::RXSTATUS_SPEC>;
#[doc = "This register shows a status that indicates if data sent from the debugger to the CPU has been read."]
pub mod rxstatus;
#[doc = "TXDATA register accessor: an alias for `Reg<TXDATA_SPEC>`"]
pub type TXDATA = crate::Reg<txdata::TXDATA_SPEC>;
#[doc = "Data sent from the CPU to the debugger."]
pub mod txdata;
#[doc = "TXSTATUS register accessor: an alias for `Reg<TXSTATUS_SPEC>`"]
pub type TXSTATUS = crate::Reg<txstatus::TXSTATUS_SPEC>;
#[doc = "This register shows a status that indicates if the data sent from the CPU to the debugger has been read."]
pub mod txstatus;
