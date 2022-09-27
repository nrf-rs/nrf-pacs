#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0504],
    #[doc = "0x504 - Write GPIO port."]
    pub out: OUT,
    #[doc = "0x508 - Set individual bits in GPIO port."]
    pub outset: OUTSET,
    #[doc = "0x50c - Clear individual bits in GPIO port."]
    pub outclr: OUTCLR,
    #[doc = "0x510 - Read GPIO port."]
    pub in_: IN,
    #[doc = "0x514 - Direction of GPIO pins."]
    pub dir: DIR,
    #[doc = "0x518 - DIR set register."]
    pub dirset: DIRSET,
    #[doc = "0x51c - DIR clear register."]
    pub dirclr: DIRCLR,
    _reserved7: [u8; 0x01e0],
    #[doc = "0x700..0x780 - Configuration of GPIO pins."]
    pub pin_cnf: [PIN_CNF; 32],
}
#[doc = "OUT (rw) register accessor: an alias for `Reg<OUT_SPEC>`"]
pub type OUT = crate::Reg<out::OUT_SPEC>;
#[doc = "Write GPIO port."]
pub mod out;
#[doc = "OUTSET (rw) register accessor: an alias for `Reg<OUTSET_SPEC>`"]
pub type OUTSET = crate::Reg<outset::OUTSET_SPEC>;
#[doc = "Set individual bits in GPIO port."]
pub mod outset;
#[doc = "OUTCLR (rw) register accessor: an alias for `Reg<OUTCLR_SPEC>`"]
pub type OUTCLR = crate::Reg<outclr::OUTCLR_SPEC>;
#[doc = "Clear individual bits in GPIO port."]
pub mod outclr;
#[doc = "IN (r) register accessor: an alias for `Reg<IN_SPEC>`"]
pub type IN = crate::Reg<in_::IN_SPEC>;
#[doc = "Read GPIO port."]
pub mod in_;
#[doc = "DIR (rw) register accessor: an alias for `Reg<DIR_SPEC>`"]
pub type DIR = crate::Reg<dir::DIR_SPEC>;
#[doc = "Direction of GPIO pins."]
pub mod dir;
#[doc = "DIRSET (rw) register accessor: an alias for `Reg<DIRSET_SPEC>`"]
pub type DIRSET = crate::Reg<dirset::DIRSET_SPEC>;
#[doc = "DIR set register."]
pub mod dirset;
#[doc = "DIRCLR (rw) register accessor: an alias for `Reg<DIRCLR_SPEC>`"]
pub type DIRCLR = crate::Reg<dirclr::DIRCLR_SPEC>;
#[doc = "DIR clear register."]
pub mod dirclr;
#[doc = "PIN_CNF (rw) register accessor: an alias for `Reg<PIN_CNF_SPEC>`"]
pub type PIN_CNF = crate::Reg<pin_cnf::PIN_CNF_SPEC>;
#[doc = "Configuration of GPIO pins."]
pub mod pin_cnf;
