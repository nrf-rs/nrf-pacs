#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start generation of keystream. This operation will stop by itself when completed."]
    pub tasks_ksgen: crate::Reg<tasks_ksgen::TASKS_KSGEN_SPEC>,
    #[doc = "0x04 - Start encryption/decryption. This operation will stop by itself when completed."]
    pub tasks_crypt: crate::Reg<tasks_crypt::TASKS_CRYPT_SPEC>,
    #[doc = "0x08 - Stop encryption/decryption"]
    pub tasks_stop: crate::Reg<tasks_stop::TASKS_STOP_SPEC>,
    #[doc = "0x0c - Override DATARATE setting in MODE register with the contents of the RATEOVERRIDE register for any ongoing encryption/decryption"]
    pub tasks_rateoverride: crate::Reg<tasks_rateoverride::TASKS_RATEOVERRIDE_SPEC>,
    _reserved4: [u8; 0x70],
    #[doc = "0x80 - Subscribe configuration for task KSGEN"]
    pub subscribe_ksgen: crate::Reg<subscribe_ksgen::SUBSCRIBE_KSGEN_SPEC>,
    #[doc = "0x84 - Subscribe configuration for task CRYPT"]
    pub subscribe_crypt: crate::Reg<subscribe_crypt::SUBSCRIBE_CRYPT_SPEC>,
    #[doc = "0x88 - Subscribe configuration for task STOP"]
    pub subscribe_stop: crate::Reg<subscribe_stop::SUBSCRIBE_STOP_SPEC>,
    #[doc = "0x8c - Subscribe configuration for task RATEOVERRIDE"]
    pub subscribe_rateoverride: crate::Reg<subscribe_rateoverride::SUBSCRIBE_RATEOVERRIDE_SPEC>,
    _reserved8: [u8; 0x70],
    #[doc = "0x100 - Keystream generation complete"]
    pub events_endksgen: crate::Reg<events_endksgen::EVENTS_ENDKSGEN_SPEC>,
    #[doc = "0x104 - Encrypt/decrypt complete"]
    pub events_endcrypt: crate::Reg<events_endcrypt::EVENTS_ENDCRYPT_SPEC>,
    #[doc = "0x108 - Deprecated register - CCM error event"]
    pub events_error: crate::Reg<events_error::EVENTS_ERROR_SPEC>,
    _reserved11: [u8; 0x74],
    #[doc = "0x180 - Publish configuration for event ENDKSGEN"]
    pub publish_endksgen: crate::Reg<publish_endksgen::PUBLISH_ENDKSGEN_SPEC>,
    #[doc = "0x184 - Publish configuration for event ENDCRYPT"]
    pub publish_endcrypt: crate::Reg<publish_endcrypt::PUBLISH_ENDCRYPT_SPEC>,
    #[doc = "0x188 - Deprecated register - Publish configuration for event ERROR"]
    pub publish_error: crate::Reg<publish_error::PUBLISH_ERROR_SPEC>,
    _reserved14: [u8; 0x74],
    #[doc = "0x200 - Shortcuts between local events and tasks"]
    pub shorts: crate::Reg<shorts::SHORTS_SPEC>,
    _reserved15: [u8; 0x0100],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
    _reserved17: [u8; 0xf4],
    #[doc = "0x400 - MIC check result"]
    pub micstatus: crate::Reg<micstatus::MICSTATUS_SPEC>,
    _reserved18: [u8; 0xfc],
    #[doc = "0x500 - Enable"]
    pub enable: crate::Reg<enable::ENABLE_SPEC>,
    #[doc = "0x504 - Operation mode"]
    pub mode: crate::Reg<mode::MODE_SPEC>,
    #[doc = "0x508 - Pointer to data structure holding the AES key and the NONCE vector"]
    pub cnfptr: crate::Reg<cnfptr::CNFPTR_SPEC>,
    #[doc = "0x50c - Input pointer"]
    pub inptr: crate::Reg<inptr::INPTR_SPEC>,
    #[doc = "0x510 - Output pointer"]
    pub outptr: crate::Reg<outptr::OUTPTR_SPEC>,
    #[doc = "0x514 - Pointer to data area used for temporary storage"]
    pub scratchptr: crate::Reg<scratchptr::SCRATCHPTR_SPEC>,
    #[doc = "0x518 - Length of keystream generated when MODE.LENGTH = Extended"]
    pub maxpacketsize: crate::Reg<maxpacketsize::MAXPACKETSIZE_SPEC>,
    #[doc = "0x51c - Data rate override setting."]
    pub rateoverride: crate::Reg<rateoverride::RATEOVERRIDE_SPEC>,
    #[doc = "0x520 - Header (S0) mask."]
    pub headermask: crate::Reg<headermask::HEADERMASK_SPEC>,
}
#[doc = "TASKS_KSGEN register accessor: an alias for `Reg<TASKS_KSGEN_SPEC>`"]
pub type TASKS_KSGEN = crate::Reg<tasks_ksgen::TASKS_KSGEN_SPEC>;
#[doc = "Start generation of keystream. This operation will stop by itself when completed."]
pub mod tasks_ksgen;
#[doc = "TASKS_CRYPT register accessor: an alias for `Reg<TASKS_CRYPT_SPEC>`"]
pub type TASKS_CRYPT = crate::Reg<tasks_crypt::TASKS_CRYPT_SPEC>;
#[doc = "Start encryption/decryption. This operation will stop by itself when completed."]
pub mod tasks_crypt;
#[doc = "TASKS_STOP register accessor: an alias for `Reg<TASKS_STOP_SPEC>`"]
pub type TASKS_STOP = crate::Reg<tasks_stop::TASKS_STOP_SPEC>;
#[doc = "Stop encryption/decryption"]
pub mod tasks_stop;
#[doc = "TASKS_RATEOVERRIDE register accessor: an alias for `Reg<TASKS_RATEOVERRIDE_SPEC>`"]
pub type TASKS_RATEOVERRIDE = crate::Reg<tasks_rateoverride::TASKS_RATEOVERRIDE_SPEC>;
#[doc = "Override DATARATE setting in MODE register with the contents of the RATEOVERRIDE register for any ongoing encryption/decryption"]
pub mod tasks_rateoverride;
#[doc = "SUBSCRIBE_KSGEN register accessor: an alias for `Reg<SUBSCRIBE_KSGEN_SPEC>`"]
pub type SUBSCRIBE_KSGEN = crate::Reg<subscribe_ksgen::SUBSCRIBE_KSGEN_SPEC>;
#[doc = "Subscribe configuration for task KSGEN"]
pub mod subscribe_ksgen;
#[doc = "SUBSCRIBE_CRYPT register accessor: an alias for `Reg<SUBSCRIBE_CRYPT_SPEC>`"]
pub type SUBSCRIBE_CRYPT = crate::Reg<subscribe_crypt::SUBSCRIBE_CRYPT_SPEC>;
#[doc = "Subscribe configuration for task CRYPT"]
pub mod subscribe_crypt;
#[doc = "SUBSCRIBE_STOP register accessor: an alias for `Reg<SUBSCRIBE_STOP_SPEC>`"]
pub type SUBSCRIBE_STOP = crate::Reg<subscribe_stop::SUBSCRIBE_STOP_SPEC>;
#[doc = "Subscribe configuration for task STOP"]
pub mod subscribe_stop;
#[doc = "SUBSCRIBE_RATEOVERRIDE register accessor: an alias for `Reg<SUBSCRIBE_RATEOVERRIDE_SPEC>`"]
pub type SUBSCRIBE_RATEOVERRIDE = crate::Reg<subscribe_rateoverride::SUBSCRIBE_RATEOVERRIDE_SPEC>;
#[doc = "Subscribe configuration for task RATEOVERRIDE"]
pub mod subscribe_rateoverride;
#[doc = "EVENTS_ENDKSGEN register accessor: an alias for `Reg<EVENTS_ENDKSGEN_SPEC>`"]
pub type EVENTS_ENDKSGEN = crate::Reg<events_endksgen::EVENTS_ENDKSGEN_SPEC>;
#[doc = "Keystream generation complete"]
pub mod events_endksgen;
#[doc = "EVENTS_ENDCRYPT register accessor: an alias for `Reg<EVENTS_ENDCRYPT_SPEC>`"]
pub type EVENTS_ENDCRYPT = crate::Reg<events_endcrypt::EVENTS_ENDCRYPT_SPEC>;
#[doc = "Encrypt/decrypt complete"]
pub mod events_endcrypt;
#[doc = "EVENTS_ERROR register accessor: an alias for `Reg<EVENTS_ERROR_SPEC>`"]
pub type EVENTS_ERROR = crate::Reg<events_error::EVENTS_ERROR_SPEC>;
#[doc = "Deprecated register - CCM error event"]
pub mod events_error;
#[doc = "PUBLISH_ENDKSGEN register accessor: an alias for `Reg<PUBLISH_ENDKSGEN_SPEC>`"]
pub type PUBLISH_ENDKSGEN = crate::Reg<publish_endksgen::PUBLISH_ENDKSGEN_SPEC>;
#[doc = "Publish configuration for event ENDKSGEN"]
pub mod publish_endksgen;
#[doc = "PUBLISH_ENDCRYPT register accessor: an alias for `Reg<PUBLISH_ENDCRYPT_SPEC>`"]
pub type PUBLISH_ENDCRYPT = crate::Reg<publish_endcrypt::PUBLISH_ENDCRYPT_SPEC>;
#[doc = "Publish configuration for event ENDCRYPT"]
pub mod publish_endcrypt;
#[doc = "PUBLISH_ERROR register accessor: an alias for `Reg<PUBLISH_ERROR_SPEC>`"]
pub type PUBLISH_ERROR = crate::Reg<publish_error::PUBLISH_ERROR_SPEC>;
#[doc = "Deprecated register - Publish configuration for event ERROR"]
pub mod publish_error;
#[doc = "SHORTS register accessor: an alias for `Reg<SHORTS_SPEC>`"]
pub type SHORTS = crate::Reg<shorts::SHORTS_SPEC>;
#[doc = "Shortcuts between local events and tasks"]
pub mod shorts;
#[doc = "INTENSET register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "INTENCLR register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "MICSTATUS register accessor: an alias for `Reg<MICSTATUS_SPEC>`"]
pub type MICSTATUS = crate::Reg<micstatus::MICSTATUS_SPEC>;
#[doc = "MIC check result"]
pub mod micstatus;
#[doc = "ENABLE register accessor: an alias for `Reg<ENABLE_SPEC>`"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "Enable"]
pub mod enable;
#[doc = "MODE register accessor: an alias for `Reg<MODE_SPEC>`"]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = "Operation mode"]
pub mod mode;
#[doc = "CNFPTR register accessor: an alias for `Reg<CNFPTR_SPEC>`"]
pub type CNFPTR = crate::Reg<cnfptr::CNFPTR_SPEC>;
#[doc = "Pointer to data structure holding the AES key and the NONCE vector"]
pub mod cnfptr;
#[doc = "INPTR register accessor: an alias for `Reg<INPTR_SPEC>`"]
pub type INPTR = crate::Reg<inptr::INPTR_SPEC>;
#[doc = "Input pointer"]
pub mod inptr;
#[doc = "OUTPTR register accessor: an alias for `Reg<OUTPTR_SPEC>`"]
pub type OUTPTR = crate::Reg<outptr::OUTPTR_SPEC>;
#[doc = "Output pointer"]
pub mod outptr;
#[doc = "SCRATCHPTR register accessor: an alias for `Reg<SCRATCHPTR_SPEC>`"]
pub type SCRATCHPTR = crate::Reg<scratchptr::SCRATCHPTR_SPEC>;
#[doc = "Pointer to data area used for temporary storage"]
pub mod scratchptr;
#[doc = "MAXPACKETSIZE register accessor: an alias for `Reg<MAXPACKETSIZE_SPEC>`"]
pub type MAXPACKETSIZE = crate::Reg<maxpacketsize::MAXPACKETSIZE_SPEC>;
#[doc = "Length of keystream generated when MODE.LENGTH = Extended"]
pub mod maxpacketsize;
#[doc = "RATEOVERRIDE register accessor: an alias for `Reg<RATEOVERRIDE_SPEC>`"]
pub type RATEOVERRIDE = crate::Reg<rateoverride::RATEOVERRIDE_SPEC>;
#[doc = "Data rate override setting."]
pub mod rateoverride;
#[doc = "HEADERMASK register accessor: an alias for `Reg<HEADERMASK_SPEC>`"]
pub type HEADERMASK = crate::Reg<headermask::HEADERMASK_SPEC>;
#[doc = "Header (S0) mask."]
pub mod headermask;
