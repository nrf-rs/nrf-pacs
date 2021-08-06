#[doc = r"Register block"]
#[repr(C)]
pub struct CONFIG {
    #[doc = "0x00 - Description cluster: Destination address where content of the key value registers (KEYSLOT.KEYn.VALUE\\[0-3\\]) will be pushed by KMU. Note that this address must match that of a peripherals APB mapped write-only key registers, else the KMU can push this key value into an address range which the CPU can potentially read."]
    pub dest: crate::Reg<self::config::dest::DEST_SPEC>,
    #[doc = "0x04 - Description cluster: Define permissions for the key slot. Bits 0-15 and 16-31 can only be written when equal to 0xFFFF."]
    pub perm: crate::Reg<self::config::perm::PERM_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod config;
#[doc = r"Register block"]
#[repr(C)]
pub struct KEY {
    #[doc = "0x00..0x10 - Description collection: Define bits \\[31+o*32:0+o*32\\]
of value assigned to KMU key slot."]
    pub value: [crate::Reg<self::key::value::VALUE_SPEC>; 4],
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod key;
