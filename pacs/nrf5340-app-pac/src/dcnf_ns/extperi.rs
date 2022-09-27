#[doc = r"Register block"]
#[repr(C)]
pub struct EXTPERI {
    #[doc = "0x00 - Description cluster: Control access for master connected to AMLI master port EXTPERI\\[n\\]"]
    pub protect: PROTECT,
}
#[doc = "PROTECT (rw) register accessor: an alias for `Reg<PROTECT_SPEC>`"]
pub type PROTECT = crate::Reg<protect::PROTECT_SPEC>;
#[doc = "Description cluster: Control access for master connected to AMLI master port EXTPERI\\[n\\]"]
pub mod protect;
