#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Unspecified"]
    pub unused0: UNUSED0,
    #[doc = "0x04 - Unspecified"]
    pub unused1: UNUSED1,
    #[doc = "0x08 - Unspecified"]
    pub unused2: UNUSED2,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - Unspecified"]
    pub unused3: UNUSED3,
    #[doc = "0x14..0x50 - Description collection\\[0\\]: Reserved for Nordic firmware design"]
    pub nrffw: [NRFFW; 15],
    #[doc = "0x50..0x80 - Description collection\\[0\\]: Reserved for Nordic hardware design"]
    pub nrfhw: [NRFHW; 12],
    #[doc = "0x80..0x100 - Description collection\\[0\\]: Reserved for customer"]
    pub customer: [CUSTOMER; 32],
    _reserved7: [u8; 0x0100],
    #[doc = "0x200..0x208 - Description collection\\[0\\]: Mapping of the nRESET function (see POWER chapter for details)"]
    pub pselreset: [PSELRESET; 2],
    #[doc = "0x208 - Access Port protection"]
    pub approtect: APPROTECT,
    #[doc = "0x20c - Setting of pins dedicated to NFC functionality: NFC antenna or GPIO"]
    pub nfcpins: NFCPINS,
}
#[doc = "UNUSED0 (rw) register accessor: an alias for `Reg<UNUSED0_SPEC>`"]
pub type UNUSED0 = crate::Reg<unused0::UNUSED0_SPEC>;
#[doc = "Unspecified"]
pub mod unused0;
#[doc = "UNUSED1 (rw) register accessor: an alias for `Reg<UNUSED1_SPEC>`"]
pub type UNUSED1 = crate::Reg<unused1::UNUSED1_SPEC>;
#[doc = "Unspecified"]
pub mod unused1;
#[doc = "UNUSED2 (rw) register accessor: an alias for `Reg<UNUSED2_SPEC>`"]
pub type UNUSED2 = crate::Reg<unused2::UNUSED2_SPEC>;
#[doc = "Unspecified"]
pub mod unused2;
#[doc = "UNUSED3 (rw) register accessor: an alias for `Reg<UNUSED3_SPEC>`"]
pub type UNUSED3 = crate::Reg<unused3::UNUSED3_SPEC>;
#[doc = "Unspecified"]
pub mod unused3;
#[doc = "NRFFW (rw) register accessor: an alias for `Reg<NRFFW_SPEC>`"]
pub type NRFFW = crate::Reg<nrffw::NRFFW_SPEC>;
#[doc = "Description collection\\[0\\]: Reserved for Nordic firmware design"]
pub mod nrffw;
#[doc = "NRFHW (rw) register accessor: an alias for `Reg<NRFHW_SPEC>`"]
pub type NRFHW = crate::Reg<nrfhw::NRFHW_SPEC>;
#[doc = "Description collection\\[0\\]: Reserved for Nordic hardware design"]
pub mod nrfhw;
#[doc = "CUSTOMER (rw) register accessor: an alias for `Reg<CUSTOMER_SPEC>`"]
pub type CUSTOMER = crate::Reg<customer::CUSTOMER_SPEC>;
#[doc = "Description collection\\[0\\]: Reserved for customer"]
pub mod customer;
#[doc = "PSELRESET (rw) register accessor: an alias for `Reg<PSELRESET_SPEC>`"]
pub type PSELRESET = crate::Reg<pselreset::PSELRESET_SPEC>;
#[doc = "Description collection\\[0\\]: Mapping of the nRESET function (see POWER chapter for details)"]
pub mod pselreset;
#[doc = "APPROTECT (rw) register accessor: an alias for `Reg<APPROTECT_SPEC>`"]
pub type APPROTECT = crate::Reg<approtect::APPROTECT_SPEC>;
#[doc = "Access Port protection"]
pub mod approtect;
#[doc = "NFCPINS (rw) register accessor: an alias for `Reg<NFCPINS_SPEC>`"]
pub type NFCPINS = crate::Reg<nfcpins::NFCPINS_SPEC>;
#[doc = "Setting of pins dedicated to NFC functionality: NFC antenna or GPIO"]
pub mod nfcpins;
