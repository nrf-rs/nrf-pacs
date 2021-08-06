#[doc = "PTR register accessor: an alias for `Reg<PTR_SPEC>`"]
pub type PTR = crate::Reg<ptr::PTR_SPEC>;
#[doc = "Description cluster: Beginning address in RAM of this sequence"]
pub mod ptr;
#[doc = "CNT register accessor: an alias for `Reg<CNT_SPEC>`"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "Description cluster: Number of values (duty cycles) in this sequence"]
pub mod cnt;
#[doc = "REFRESH register accessor: an alias for `Reg<REFRESH_SPEC>`"]
pub type REFRESH = crate::Reg<refresh::REFRESH_SPEC>;
#[doc = "Description cluster: Number of additional PWM periods between samples loaded into compare register"]
pub mod refresh;
#[doc = "ENDDELAY register accessor: an alias for `Reg<ENDDELAY_SPEC>`"]
pub type ENDDELAY = crate::Reg<enddelay::ENDDELAY_SPEC>;
#[doc = "Description cluster: Time added after the sequence"]
pub mod enddelay;
