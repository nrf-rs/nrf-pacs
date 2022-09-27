#[doc = r"Register block"]
#[repr(C)]
pub struct SET {
    #[doc = "0x00..0x08 - Description collection: Cache information for SET\\[n\\], WAY\\[o\\]."]
    pub way: [WAY; 2],
}
#[doc = "WAY (rw) register accessor: an alias for `Reg<WAY_SPEC>`"]
pub type WAY = crate::Reg<way::WAY_SPEC>;
#[doc = "Description collection: Cache information for SET\\[n\\], WAY\\[o\\]."]
pub mod way;
