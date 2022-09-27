#[doc = r"Register block"]
#[repr(C)]
pub struct WAY {
    #[doc = "0x00 - Description cluster: Cache data bits \\[31:0\\]
of SET\\[n\\], WAY\\[o\\]."]
    pub data0: DATA0,
    #[doc = "0x04 - Description cluster: Cache data bits \\[63:32\\]
of SET\\[n\\], WAY\\[o\\]."]
    pub data1: DATA1,
    #[doc = "0x08 - Description cluster: Cache data bits \\[95:64\\]
of SET\\[n\\], WAY\\[o\\]."]
    pub data2: DATA2,
    #[doc = "0x0c - Description cluster: Cache data bits \\[127:96\\]
of SET\\[n\\], WAY\\[o\\]."]
    pub data3: DATA3,
}
#[doc = "DATA0 (rw) register accessor: an alias for `Reg<DATA0_SPEC>`"]
pub type DATA0 = crate::Reg<data0::DATA0_SPEC>;
#[doc = "Description cluster: Cache data bits \\[31:0\\]
of SET\\[n\\], WAY\\[o\\]."]
pub mod data0;
#[doc = "DATA1 (rw) register accessor: an alias for `Reg<DATA1_SPEC>`"]
pub type DATA1 = crate::Reg<data1::DATA1_SPEC>;
#[doc = "Description cluster: Cache data bits \\[63:32\\]
of SET\\[n\\], WAY\\[o\\]."]
pub mod data1;
#[doc = "DATA2 (rw) register accessor: an alias for `Reg<DATA2_SPEC>`"]
pub type DATA2 = crate::Reg<data2::DATA2_SPEC>;
#[doc = "Description cluster: Cache data bits \\[95:64\\]
of SET\\[n\\], WAY\\[o\\]."]
pub mod data2;
#[doc = "DATA3 (rw) register accessor: an alias for `Reg<DATA3_SPEC>`"]
pub type DATA3 = crate::Reg<data3::DATA3_SPEC>;
#[doc = "Description cluster: Cache data bits \\[127:96\\]
of SET\\[n\\], WAY\\[o\\]."]
pub mod data3;
