#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0100],
    #[doc = "0x100..0x120 - Peripheral events."]
    pub events_region: [EVENTS_REGION; 4],
    _reserved1: [u8; 0x40],
    #[doc = "0x160..0x170 - Peripheral events."]
    pub events_pregion: [EVENTS_PREGION; 2],
    _reserved2: [u8; 0x0190],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: crate::Reg<inten::INTEN_SPEC>,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
    _reserved5: [u8; 0x14],
    #[doc = "0x320 - Enable or disable interrupt"]
    pub nmien: crate::Reg<nmien::NMIEN_SPEC>,
    #[doc = "0x324 - Enable interrupt"]
    pub nmienset: crate::Reg<nmienset::NMIENSET_SPEC>,
    #[doc = "0x328 - Disable interrupt"]
    pub nmienclr: crate::Reg<nmienclr::NMIENCLR_SPEC>,
    _reserved8: [u8; 0xd4],
    #[doc = "0x400..0x410 - Unspecified"]
    pub perregion: [PERREGION; 2],
    _reserved9: [u8; 0x0100],
    #[doc = "0x510 - Enable/disable regions watch"]
    pub regionen: crate::Reg<regionen::REGIONEN_SPEC>,
    #[doc = "0x514 - Enable regions watch"]
    pub regionenset: crate::Reg<regionenset::REGIONENSET_SPEC>,
    #[doc = "0x518 - Disable regions watch"]
    pub regionenclr: crate::Reg<regionenclr::REGIONENCLR_SPEC>,
    _reserved12: [u8; 0xe4],
    #[doc = "0x600..0x608 - Unspecified"]
    pub region0: REGION,
    _reserved13: [u8; 0x08],
    #[doc = "0x610..0x618 - Unspecified"]
    pub region1: REGION,
    _reserved14: [u8; 0x08],
    #[doc = "0x620..0x628 - Unspecified"]
    pub region2: REGION,
    _reserved15: [u8; 0x08],
    #[doc = "0x630..0x638 - Unspecified"]
    pub region3: REGION,
    _reserved16: [u8; 0x88],
    #[doc = "0x6c0..0x6cc - Unspecified"]
    pub pregion0: PREGION,
    _reserved17: [u8; 0x04],
    #[doc = "0x6d0..0x6dc - Unspecified"]
    pub pregion1: PREGION,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct EVENTS_REGION {
    #[doc = "0x00 - Description cluster: Write access to region n detected"]
    pub wa: crate::Reg<self::events_region::wa::WA_SPEC>,
    #[doc = "0x04 - Description cluster: Read access to region n detected"]
    pub ra: crate::Reg<self::events_region::ra::RA_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Peripheral events."]
pub mod events_region;
#[doc = r"Register block"]
#[repr(C)]
pub struct EVENTS_PREGION {
    #[doc = "0x00 - Description cluster: Write access to peripheral region n detected"]
    pub wa: crate::Reg<self::events_pregion::wa::WA_SPEC>,
    #[doc = "0x04 - Description cluster: Read access to peripheral region n detected"]
    pub ra: crate::Reg<self::events_pregion::ra::RA_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Peripheral events."]
pub mod events_pregion;
#[doc = r"Register block"]
#[repr(C)]
pub struct PERREGION {
    #[doc = "0x00 - Description cluster: Source of event/interrupt in region n, write access detected while corresponding subregion was enabled for watching"]
    pub substatwa: crate::Reg<self::perregion::substatwa::SUBSTATWA_SPEC>,
    #[doc = "0x04 - Description cluster: Source of event/interrupt in region n, read access detected while corresponding subregion was enabled for watching"]
    pub substatra: crate::Reg<self::perregion::substatra::SUBSTATRA_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod perregion;
#[doc = r"Register block"]
#[repr(C)]
pub struct REGION {
    #[doc = "0x00 - Description cluster: Start address for region n"]
    pub start: crate::Reg<self::region::start::START_SPEC>,
    #[doc = "0x04 - Description cluster: End address of region n"]
    pub end: crate::Reg<self::region::end::END_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod region;
#[doc = r"Register block"]
#[repr(C)]
pub struct PREGION {
    #[doc = "0x00 - Description cluster: Reserved for future use"]
    pub start: crate::Reg<self::pregion::start::START_SPEC>,
    #[doc = "0x04 - Description cluster: Reserved for future use"]
    pub end: crate::Reg<self::pregion::end::END_SPEC>,
    #[doc = "0x08 - Description cluster: Subregions of region n"]
    pub subs: crate::Reg<self::pregion::subs::SUBS_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod pregion;
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
#[doc = "NMIEN register accessor: an alias for `Reg<NMIEN_SPEC>`"]
pub type NMIEN = crate::Reg<nmien::NMIEN_SPEC>;
#[doc = "Enable or disable interrupt"]
pub mod nmien;
#[doc = "NMIENSET register accessor: an alias for `Reg<NMIENSET_SPEC>`"]
pub type NMIENSET = crate::Reg<nmienset::NMIENSET_SPEC>;
#[doc = "Enable interrupt"]
pub mod nmienset;
#[doc = "NMIENCLR register accessor: an alias for `Reg<NMIENCLR_SPEC>`"]
pub type NMIENCLR = crate::Reg<nmienclr::NMIENCLR_SPEC>;
#[doc = "Disable interrupt"]
pub mod nmienclr;
#[doc = "REGIONEN register accessor: an alias for `Reg<REGIONEN_SPEC>`"]
pub type REGIONEN = crate::Reg<regionen::REGIONEN_SPEC>;
#[doc = "Enable/disable regions watch"]
pub mod regionen;
#[doc = "REGIONENSET register accessor: an alias for `Reg<REGIONENSET_SPEC>`"]
pub type REGIONENSET = crate::Reg<regionenset::REGIONENSET_SPEC>;
#[doc = "Enable regions watch"]
pub mod regionenset;
#[doc = "REGIONENCLR register accessor: an alias for `Reg<REGIONENCLR_SPEC>`"]
pub type REGIONENCLR = crate::Reg<regionenclr::REGIONENCLR_SPEC>;
#[doc = "Disable regions watch"]
pub mod regionenclr;
