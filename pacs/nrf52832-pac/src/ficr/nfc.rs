#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "NFC")]
pub struct Nfc {
    tagheader0: Tagheader0,
    tagheader1: Tagheader1,
    tagheader2: Tagheader2,
    tagheader3: Tagheader3,
}
impl Nfc {
    #[doc = "0x00 - Default header for NFC Tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST."]
    #[inline(always)]
    pub const fn tagheader0(&self) -> &Tagheader0 {
        &self.tagheader0
    }
    #[doc = "0x04 - Default header for NFC Tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST."]
    #[inline(always)]
    pub const fn tagheader1(&self) -> &Tagheader1 {
        &self.tagheader1
    }
    #[doc = "0x08 - Default header for NFC Tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST."]
    #[inline(always)]
    pub const fn tagheader2(&self) -> &Tagheader2 {
        &self.tagheader2
    }
    #[doc = "0x0c - Default header for NFC Tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST."]
    #[inline(always)]
    pub const fn tagheader3(&self) -> &Tagheader3 {
        &self.tagheader3
    }
}
#[doc = "TAGHEADER0 (r) register accessor: Default header for NFC Tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST.\n\nYou can [`read`](crate::Reg::read) this register and get [`tagheader0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tagheader0`] module"]
#[doc(alias = "TAGHEADER0")]
pub type Tagheader0 = crate::Reg<tagheader0::Tagheader0Spec>;
#[doc = "Default header for NFC Tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST."]
pub mod tagheader0;
#[doc = "TAGHEADER1 (r) register accessor: Default header for NFC Tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST.\n\nYou can [`read`](crate::Reg::read) this register and get [`tagheader1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tagheader1`] module"]
#[doc(alias = "TAGHEADER1")]
pub type Tagheader1 = crate::Reg<tagheader1::Tagheader1Spec>;
#[doc = "Default header for NFC Tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST."]
pub mod tagheader1;
#[doc = "TAGHEADER2 (r) register accessor: Default header for NFC Tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST.\n\nYou can [`read`](crate::Reg::read) this register and get [`tagheader2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tagheader2`] module"]
#[doc(alias = "TAGHEADER2")]
pub type Tagheader2 = crate::Reg<tagheader2::Tagheader2Spec>;
#[doc = "Default header for NFC Tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST."]
pub mod tagheader2;
#[doc = "TAGHEADER3 (r) register accessor: Default header for NFC Tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST.\n\nYou can [`read`](crate::Reg::read) this register and get [`tagheader3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tagheader3`] module"]
#[doc(alias = "TAGHEADER3")]
pub type Tagheader3 = crate::Reg<tagheader3::Tagheader3Spec>;
#[doc = "Default header for NFC Tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST."]
pub mod tagheader3;
