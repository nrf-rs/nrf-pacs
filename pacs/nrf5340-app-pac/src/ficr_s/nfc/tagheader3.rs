#[doc = "Register `TAGHEADER3` reader"]
pub type R = crate::R<Tagheader3Spec>;
#[doc = "Field `UD12` reader - Unique identifier byte 12"]
pub type Ud12R = crate::FieldReader;
#[doc = "Field `UD13` reader - Unique identifier byte 13"]
pub type Ud13R = crate::FieldReader;
#[doc = "Field `UD14` reader - Unique identifier byte 14"]
pub type Ud14R = crate::FieldReader;
#[doc = "Field `UD15` reader - Unique identifier byte 15"]
pub type Ud15R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Unique identifier byte 12"]
    #[inline(always)]
    pub fn ud12(&self) -> Ud12R {
        Ud12R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Unique identifier byte 13"]
    #[inline(always)]
    pub fn ud13(&self) -> Ud13R {
        Ud13R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Unique identifier byte 14"]
    #[inline(always)]
    pub fn ud14(&self) -> Ud14R {
        Ud14R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Unique identifier byte 15"]
    #[inline(always)]
    pub fn ud15(&self) -> Ud15R {
        Ud15R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Default header for NFC Tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST.\n\nYou can [`read`](crate::Reg::read) this register and get [`tagheader3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tagheader3Spec;
impl crate::RegisterSpec for Tagheader3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tagheader3::R`](R) reader structure"]
impl crate::Readable for Tagheader3Spec {}
#[doc = "`reset()` method sets TAGHEADER3 to value 0xffff_ffff"]
impl crate::Resettable for Tagheader3Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
