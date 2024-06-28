#[doc = r"Register block"]
#[repr(C)]
pub struct CONFIG {
    #[doc = "0x00 - Description cluster: Destination address where content of the key value registers (KEYSLOT.KEYn.VALUE\\[0-3\\]) will be pushed by KMU. Note that this address must match that of a peripherals APB mapped write-only key registers, else the KMU can push this key value into an address range which the CPU can potentially read."]
    pub dest: DEST,
    #[doc = "0x04 - Description cluster: Define permissions for the key slot. Bits 0-15 and 16-31 can only be written when equal to 0xFFFF."]
    pub perm: PERM,
}
#[doc = "DEST (rw) register accessor: an alias for `Reg<DEST_SPEC>`"]
pub type DEST = crate::Reg<dest::DEST_SPEC>;
#[doc = "Description cluster: Destination address where content of the key value registers (KEYSLOT.KEYn.VALUE\\[0-3\\]) will be pushed by KMU. Note that this address must match that of a peripherals APB mapped write-only key registers, else the KMU can push this key value into an address range which the CPU can potentially read."]
pub mod dest;
#[doc = "PERM (rw) register accessor: an alias for `Reg<PERM_SPEC>`"]
pub type PERM = crate::Reg<perm::PERM_SPEC>;
#[doc = "Description cluster: Define permissions for the key slot. Bits 0-15 and 16-31 can only be written when equal to 0xFFFF."]
pub mod perm;
