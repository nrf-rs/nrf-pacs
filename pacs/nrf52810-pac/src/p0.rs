#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0504],
    #[doc = "0x504 - Write GPIO port"]
    pub out: crate::Reg<out::OUT_SPEC>,
    #[doc = "0x508 - Set individual bits in GPIO port"]
    pub outset: crate::Reg<outset::OUTSET_SPEC>,
    #[doc = "0x50c - Clear individual bits in GPIO port"]
    pub outclr: crate::Reg<outclr::OUTCLR_SPEC>,
    #[doc = "0x510 - Read GPIO port"]
    pub in_: crate::Reg<in_::IN_SPEC>,
    #[doc = "0x514 - Direction of GPIO pins"]
    pub dir: crate::Reg<dir::DIR_SPEC>,
    #[doc = "0x518 - DIR set register"]
    pub dirset: crate::Reg<dirset::DIRSET_SPEC>,
    #[doc = "0x51c - DIR clear register"]
    pub dirclr: crate::Reg<dirclr::DIRCLR_SPEC>,
    #[doc = "0x520 - Latch register indicating what GPIO pins that have met the criteria set in the PIN_CNF\\[n\\].SENSE registers"]
    pub latch: crate::Reg<latch::LATCH_SPEC>,
    #[doc = "0x524 - Select between default DETECT signal behaviour and LDETECT mode"]
    pub detectmode: crate::Reg<detectmode::DETECTMODE_SPEC>,
    _reserved9: [u8; 0x01d8],
    #[doc = "0x700..0x780 - Description collection: Configuration of GPIO pins"]
    pub pin_cnf: [crate::Reg<pin_cnf::PIN_CNF_SPEC>; 32],
}
#[doc = "OUT register accessor: an alias for `Reg<OUT_SPEC>`"]
pub type OUT = crate::Reg<out::OUT_SPEC>;
#[doc = "Write GPIO port"]
pub mod out;
#[doc = "OUTSET register accessor: an alias for `Reg<OUTSET_SPEC>`"]
pub type OUTSET = crate::Reg<outset::OUTSET_SPEC>;
#[doc = "Set individual bits in GPIO port"]
pub mod outset;
#[doc = "OUTCLR register accessor: an alias for `Reg<OUTCLR_SPEC>`"]
pub type OUTCLR = crate::Reg<outclr::OUTCLR_SPEC>;
#[doc = "Clear individual bits in GPIO port"]
pub mod outclr;
#[doc = "IN register accessor: an alias for `Reg<IN_SPEC>`"]
pub type IN = crate::Reg<in_::IN_SPEC>;
#[doc = "Read GPIO port"]
pub mod in_;
#[doc = "DIR register accessor: an alias for `Reg<DIR_SPEC>`"]
pub type DIR = crate::Reg<dir::DIR_SPEC>;
#[doc = "Direction of GPIO pins"]
pub mod dir;
#[doc = "DIRSET register accessor: an alias for `Reg<DIRSET_SPEC>`"]
pub type DIRSET = crate::Reg<dirset::DIRSET_SPEC>;
#[doc = "DIR set register"]
pub mod dirset;
#[doc = "DIRCLR register accessor: an alias for `Reg<DIRCLR_SPEC>`"]
pub type DIRCLR = crate::Reg<dirclr::DIRCLR_SPEC>;
#[doc = "DIR clear register"]
pub mod dirclr;
#[doc = "LATCH register accessor: an alias for `Reg<LATCH_SPEC>`"]
pub type LATCH = crate::Reg<latch::LATCH_SPEC>;
#[doc = "Latch register indicating what GPIO pins that have met the criteria set in the PIN_CNF\\[n\\].SENSE registers"]
pub mod latch;
#[doc = "DETECTMODE register accessor: an alias for `Reg<DETECTMODE_SPEC>`"]
pub type DETECTMODE = crate::Reg<detectmode::DETECTMODE_SPEC>;
#[doc = "Select between default DETECT signal behaviour and LDETECT mode"]
pub mod detectmode;
#[doc = "PIN_CNF register accessor: an alias for `Reg<PIN_CNF_SPEC>`"]
pub type PIN_CNF = crate::Reg<pin_cnf::PIN_CNF_SPEC>;
#[doc = "Description collection: Configuration of GPIO pins"]
pub mod pin_cnf;
