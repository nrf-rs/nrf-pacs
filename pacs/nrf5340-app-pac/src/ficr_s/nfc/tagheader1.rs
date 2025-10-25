#[doc = "Register `TAGHEADER1` reader"]
pub type R = crate::R<Tagheader1Spec>;
#[doc = "Field `UD4` reader - Unique identifier byte 4"]
pub type Ud4R = crate::FieldReader;
#[doc = "Field `UD5` reader - Unique identifier byte 5"]
pub type Ud5R = crate::FieldReader;
#[doc = "Field `UD6` reader - Unique identifier byte 6"]
pub type Ud6R = crate::FieldReader;
#[doc = "Field `UD7` reader - Unique identifier byte 7"]
pub type Ud7R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Unique identifier byte 4"]
    #[inline(always)]
    pub fn ud4(&self) -> Ud4R {
        Ud4R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Unique identifier byte 5"]
    #[inline(always)]
    pub fn ud5(&self) -> Ud5R {
        Ud5R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Unique identifier byte 6"]
    #[inline(always)]
    pub fn ud6(&self) -> Ud6R {
        Ud6R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Unique identifier byte 7"]
    #[inline(always)]
    pub fn ud7(&self) -> Ud7R {
        Ud7R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Default header for NFC Tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST.\n\nYou can [`read`](crate::Reg::read) this register and get [`tagheader1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tagheader1Spec;
impl crate::RegisterSpec for Tagheader1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tagheader1::R`](R) reader structure"]
impl crate::Readable for Tagheader1Spec {}
#[doc = "`reset()` method sets TAGHEADER1 to value 0xffff_ffff"]
impl crate::Resettable for Tagheader1Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
