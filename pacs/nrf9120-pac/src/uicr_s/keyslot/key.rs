#[doc = r"Register block"]
#[repr(C)]
pub struct KEY {
    #[doc = "0x00..0x10 - Description collection: Define bits \\[31+o*32:0+o*32\\]
of value assigned to KMU key slot."]
    pub value: [VALUE; 4],
}
#[doc = "VALUE (rw) register accessor: an alias for `Reg<VALUE_SPEC>`"]
pub type VALUE = crate::Reg<value::VALUE_SPEC>;
#[doc = "Description collection: Define bits \\[31+o*32:0+o*32\\]
of value assigned to KMU key slot."]
pub mod value;
