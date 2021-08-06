#[doc = "KEY0 register accessor: an alias for `Reg<KEY0_SPEC>`"]
pub type KEY0 = crate::Reg<key0::KEY0_SPEC>;
#[doc = "Bits 31:0 of XIP AES KEY"]
pub mod key0;
#[doc = "KEY1 register accessor: an alias for `Reg<KEY1_SPEC>`"]
pub type KEY1 = crate::Reg<key1::KEY1_SPEC>;
#[doc = "Bits 63:32 of XIP AES KEY"]
pub mod key1;
#[doc = "KEY2 register accessor: an alias for `Reg<KEY2_SPEC>`"]
pub type KEY2 = crate::Reg<key2::KEY2_SPEC>;
#[doc = "Bits 95:64 of XIP AES KEY"]
pub mod key2;
#[doc = "KEY3 register accessor: an alias for `Reg<KEY3_SPEC>`"]
pub type KEY3 = crate::Reg<key3::KEY3_SPEC>;
#[doc = "Bits 127:96 of XIP AES KEY"]
pub mod key3;
#[doc = "NONCE0 register accessor: an alias for `Reg<NONCE0_SPEC>`"]
pub type NONCE0 = crate::Reg<nonce0::NONCE0_SPEC>;
#[doc = "Bits 31:0 of XIP NONCE"]
pub mod nonce0;
#[doc = "NONCE1 register accessor: an alias for `Reg<NONCE1_SPEC>`"]
pub type NONCE1 = crate::Reg<nonce1::NONCE1_SPEC>;
#[doc = "Bits 63:32 of XIP NONCE"]
pub mod nonce1;
#[doc = "NONCE2 register accessor: an alias for `Reg<NONCE2_SPEC>`"]
pub type NONCE2 = crate::Reg<nonce2::NONCE2_SPEC>;
#[doc = "Bits 95:64 of XIP NONCE"]
pub mod nonce2;
#[doc = "ENABLE register accessor: an alias for `Reg<ENABLE_SPEC>`"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "Enable stream cipher for XIP"]
pub mod enable;
