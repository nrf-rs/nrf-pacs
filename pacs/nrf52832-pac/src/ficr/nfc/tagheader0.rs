#[doc = "Register `TAGHEADER0` reader"]
pub type R = crate::R<Tagheader0Spec>;
#[doc = "Field `MFGID` reader - Default Manufacturer ID: Nordic Semiconductor ASA has ICM 0x5F"]
pub type MfgidR = crate::FieldReader;
#[doc = "Field `UD1` reader - Unique identifier byte 1"]
pub type Ud1R = crate::FieldReader;
#[doc = "Field `UD2` reader - Unique identifier byte 2"]
pub type Ud2R = crate::FieldReader;
#[doc = "Field `UD3` reader - Unique identifier byte 3"]
pub type Ud3R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Default Manufacturer ID: Nordic Semiconductor ASA has ICM 0x5F"]
    #[inline(always)]
    pub fn mfgid(&self) -> MfgidR {
        MfgidR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Unique identifier byte 1"]
    #[inline(always)]
    pub fn ud1(&self) -> Ud1R {
        Ud1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Unique identifier byte 2"]
    #[inline(always)]
    pub fn ud2(&self) -> Ud2R {
        Ud2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Unique identifier byte 3"]
    #[inline(always)]
    pub fn ud3(&self) -> Ud3R {
        Ud3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Default header for NFC Tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST.\n\nYou can [`read`](crate::Reg::read) this register and get [`tagheader0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tagheader0Spec;
impl crate::RegisterSpec for Tagheader0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tagheader0::R`](R) reader structure"]
impl crate::Readable for Tagheader0Spec {}
#[doc = "`reset()` method sets TAGHEADER0 to value 0xffff_ff5f"]
impl crate::Resettable for Tagheader0Spec {
    const RESET_VALUE: u32 = 0xffff_ff5f;
}
