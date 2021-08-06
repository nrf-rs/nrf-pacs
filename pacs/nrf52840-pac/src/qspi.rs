#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Activate QSPI interface"]
    pub tasks_activate: crate::Reg<tasks_activate::TASKS_ACTIVATE_SPEC>,
    #[doc = "0x04 - Start transfer from external flash memory to internal RAM"]
    pub tasks_readstart: crate::Reg<tasks_readstart::TASKS_READSTART_SPEC>,
    #[doc = "0x08 - Start transfer from internal RAM to external flash memory"]
    pub tasks_writestart: crate::Reg<tasks_writestart::TASKS_WRITESTART_SPEC>,
    #[doc = "0x0c - Start external flash memory erase operation"]
    pub tasks_erasestart: crate::Reg<tasks_erasestart::TASKS_ERASESTART_SPEC>,
    #[doc = "0x10 - Deactivate QSPI interface"]
    pub tasks_deactivate: crate::Reg<tasks_deactivate::TASKS_DEACTIVATE_SPEC>,
    _reserved5: [u8; 0xec],
    #[doc = "0x100 - QSPI peripheral is ready. This event will be generated as a response to any QSPI task."]
    pub events_ready: crate::Reg<events_ready::EVENTS_READY_SPEC>,
    _reserved6: [u8; 0x01fc],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: crate::Reg<inten::INTEN_SPEC>,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
    _reserved9: [u8; 0x01f4],
    #[doc = "0x500 - Enable QSPI peripheral and acquire the pins selected in PSELn registers"]
    pub enable: crate::Reg<enable::ENABLE_SPEC>,
    #[doc = "0x504..0x510 - Unspecified"]
    pub read: READ,
    #[doc = "0x510..0x51c - Unspecified"]
    pub write: WRITE,
    #[doc = "0x51c..0x524 - Unspecified"]
    pub erase: ERASE,
    #[doc = "0x524..0x540 - Unspecified"]
    pub psel: PSEL,
    #[doc = "0x540 - Address offset into the external memory for Execute in Place operation."]
    pub xipoffset: crate::Reg<xipoffset::XIPOFFSET_SPEC>,
    #[doc = "0x544 - Interface configuration."]
    pub ifconfig0: crate::Reg<ifconfig0::IFCONFIG0_SPEC>,
    _reserved16: [u8; 0xb8],
    #[doc = "0x600 - Interface configuration."]
    pub ifconfig1: crate::Reg<ifconfig1::IFCONFIG1_SPEC>,
    #[doc = "0x604 - Status register."]
    pub status: crate::Reg<status::STATUS_SPEC>,
    _reserved18: [u8; 0x0c],
    #[doc = "0x614 - Set the duration required to enter/exit deep power-down mode (DPM)."]
    pub dpmdur: crate::Reg<dpmdur::DPMDUR_SPEC>,
    _reserved19: [u8; 0x0c],
    #[doc = "0x624 - Extended address configuration."]
    pub addrconf: crate::Reg<addrconf::ADDRCONF_SPEC>,
    _reserved20: [u8; 0x0c],
    #[doc = "0x634 - Custom instruction configuration register."]
    pub cinstrconf: crate::Reg<cinstrconf::CINSTRCONF_SPEC>,
    #[doc = "0x638 - Custom instruction data register 0."]
    pub cinstrdat0: crate::Reg<cinstrdat0::CINSTRDAT0_SPEC>,
    #[doc = "0x63c - Custom instruction data register 1."]
    pub cinstrdat1: crate::Reg<cinstrdat1::CINSTRDAT1_SPEC>,
    #[doc = "0x640 - SPI interface timing."]
    pub iftiming: crate::Reg<iftiming::IFTIMING_SPEC>,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct READ {
    #[doc = "0x00 - Flash memory source address"]
    pub src: crate::Reg<self::read::src::SRC_SPEC>,
    #[doc = "0x04 - RAM destination address"]
    pub dst: crate::Reg<self::read::dst::DST_SPEC>,
    #[doc = "0x08 - Read transfer length"]
    pub cnt: crate::Reg<self::read::cnt::CNT_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod read;
#[doc = r"Register block"]
#[repr(C)]
pub struct WRITE {
    #[doc = "0x00 - Flash destination address"]
    pub dst: crate::Reg<self::write::dst::DST_SPEC>,
    #[doc = "0x04 - RAM source address"]
    pub src: crate::Reg<self::write::src::SRC_SPEC>,
    #[doc = "0x08 - Write transfer length"]
    pub cnt: crate::Reg<self::write::cnt::CNT_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod write;
#[doc = r"Register block"]
#[repr(C)]
pub struct ERASE {
    #[doc = "0x00 - Start address of flash block to be erased"]
    pub ptr: crate::Reg<self::erase::ptr::PTR_SPEC>,
    #[doc = "0x04 - Size of block to be erased."]
    pub len: crate::Reg<self::erase::len::LEN_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod erase;
#[doc = r"Register block"]
#[repr(C)]
pub struct PSEL {
    #[doc = "0x00 - Pin select for serial clock SCK"]
    pub sck: crate::Reg<self::psel::sck::SCK_SPEC>,
    #[doc = "0x04 - Pin select for chip select signal CSN."]
    pub csn: crate::Reg<self::psel::csn::CSN_SPEC>,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - Pin select for serial data MOSI/IO0."]
    pub io0: crate::Reg<self::psel::io0::IO0_SPEC>,
    #[doc = "0x10 - Pin select for serial data MISO/IO1."]
    pub io1: crate::Reg<self::psel::io1::IO1_SPEC>,
    #[doc = "0x14 - Pin select for serial data IO2."]
    pub io2: crate::Reg<self::psel::io2::IO2_SPEC>,
    #[doc = "0x18 - Pin select for serial data IO3."]
    pub io3: crate::Reg<self::psel::io3::IO3_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod psel;
#[doc = "TASKS_ACTIVATE register accessor: an alias for `Reg<TASKS_ACTIVATE_SPEC>`"]
pub type TASKS_ACTIVATE = crate::Reg<tasks_activate::TASKS_ACTIVATE_SPEC>;
#[doc = "Activate QSPI interface"]
pub mod tasks_activate;
#[doc = "TASKS_READSTART register accessor: an alias for `Reg<TASKS_READSTART_SPEC>`"]
pub type TASKS_READSTART = crate::Reg<tasks_readstart::TASKS_READSTART_SPEC>;
#[doc = "Start transfer from external flash memory to internal RAM"]
pub mod tasks_readstart;
#[doc = "TASKS_WRITESTART register accessor: an alias for `Reg<TASKS_WRITESTART_SPEC>`"]
pub type TASKS_WRITESTART = crate::Reg<tasks_writestart::TASKS_WRITESTART_SPEC>;
#[doc = "Start transfer from internal RAM to external flash memory"]
pub mod tasks_writestart;
#[doc = "TASKS_ERASESTART register accessor: an alias for `Reg<TASKS_ERASESTART_SPEC>`"]
pub type TASKS_ERASESTART = crate::Reg<tasks_erasestart::TASKS_ERASESTART_SPEC>;
#[doc = "Start external flash memory erase operation"]
pub mod tasks_erasestart;
#[doc = "TASKS_DEACTIVATE register accessor: an alias for `Reg<TASKS_DEACTIVATE_SPEC>`"]
pub type TASKS_DEACTIVATE = crate::Reg<tasks_deactivate::TASKS_DEACTIVATE_SPEC>;
#[doc = "Deactivate QSPI interface"]
pub mod tasks_deactivate;
#[doc = "EVENTS_READY register accessor: an alias for `Reg<EVENTS_READY_SPEC>`"]
pub type EVENTS_READY = crate::Reg<events_ready::EVENTS_READY_SPEC>;
#[doc = "QSPI peripheral is ready. This event will be generated as a response to any QSPI task."]
pub mod events_ready;
#[doc = "INTEN register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "Enable or disable interrupt"]
pub mod inten;
#[doc = "INTENSET register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "INTENCLR register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "ENABLE register accessor: an alias for `Reg<ENABLE_SPEC>`"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "Enable QSPI peripheral and acquire the pins selected in PSELn registers"]
pub mod enable;
#[doc = "XIPOFFSET register accessor: an alias for `Reg<XIPOFFSET_SPEC>`"]
pub type XIPOFFSET = crate::Reg<xipoffset::XIPOFFSET_SPEC>;
#[doc = "Address offset into the external memory for Execute in Place operation."]
pub mod xipoffset;
#[doc = "IFCONFIG0 register accessor: an alias for `Reg<IFCONFIG0_SPEC>`"]
pub type IFCONFIG0 = crate::Reg<ifconfig0::IFCONFIG0_SPEC>;
#[doc = "Interface configuration."]
pub mod ifconfig0;
#[doc = "IFCONFIG1 register accessor: an alias for `Reg<IFCONFIG1_SPEC>`"]
pub type IFCONFIG1 = crate::Reg<ifconfig1::IFCONFIG1_SPEC>;
#[doc = "Interface configuration."]
pub mod ifconfig1;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status register."]
pub mod status;
#[doc = "DPMDUR register accessor: an alias for `Reg<DPMDUR_SPEC>`"]
pub type DPMDUR = crate::Reg<dpmdur::DPMDUR_SPEC>;
#[doc = "Set the duration required to enter/exit deep power-down mode (DPM)."]
pub mod dpmdur;
#[doc = "ADDRCONF register accessor: an alias for `Reg<ADDRCONF_SPEC>`"]
pub type ADDRCONF = crate::Reg<addrconf::ADDRCONF_SPEC>;
#[doc = "Extended address configuration."]
pub mod addrconf;
#[doc = "CINSTRCONF register accessor: an alias for `Reg<CINSTRCONF_SPEC>`"]
pub type CINSTRCONF = crate::Reg<cinstrconf::CINSTRCONF_SPEC>;
#[doc = "Custom instruction configuration register."]
pub mod cinstrconf;
#[doc = "CINSTRDAT0 register accessor: an alias for `Reg<CINSTRDAT0_SPEC>`"]
pub type CINSTRDAT0 = crate::Reg<cinstrdat0::CINSTRDAT0_SPEC>;
#[doc = "Custom instruction data register 0."]
pub mod cinstrdat0;
#[doc = "CINSTRDAT1 register accessor: an alias for `Reg<CINSTRDAT1_SPEC>`"]
pub type CINSTRDAT1 = crate::Reg<cinstrdat1::CINSTRDAT1_SPEC>;
#[doc = "Custom instruction data register 1."]
pub mod cinstrdat1;
#[doc = "IFTIMING register accessor: an alias for `Reg<IFTIMING_SPEC>`"]
pub type IFTIMING = crate::Reg<iftiming::IFTIMING_SPEC>;
#[doc = "SPI interface timing."]
pub mod iftiming;
