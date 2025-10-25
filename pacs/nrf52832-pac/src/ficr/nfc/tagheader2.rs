#[doc = "Register `TAGHEADER2` reader"]
pub type R = crate::R<Tagheader2Spec>;
#[doc = "Field `UD8` reader - Unique identifier byte 8"]
pub type Ud8R = crate::FieldReader;
#[doc = "Field `UD9` reader - Unique identifier byte 9"]
pub type Ud9R = crate::FieldReader;
#[doc = "Field `UD10` reader - Unique identifier byte 10"]
pub type Ud10R = crate::FieldReader;
#[doc = "Field `UD11` reader - Unique identifier byte 11"]
pub type Ud11R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Unique identifier byte 8"]
    #[inline(always)]
    pub fn ud8(&self) -> Ud8R {
        Ud8R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Unique identifier byte 9"]
    #[inline(always)]
    pub fn ud9(&self) -> Ud9R {
        Ud9R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Unique identifier byte 10"]
    #[inline(always)]
    pub fn ud10(&self) -> Ud10R {
        Ud10R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Unique identifier byte 11"]
    #[inline(always)]
    pub fn ud11(&self) -> Ud11R {
        Ud11R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Default header for NFC Tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST.\n\nYou can [`read`](crate::Reg::read) this register and get [`tagheader2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tagheader2Spec;
impl crate::RegisterSpec for Tagheader2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tagheader2::R`](R) reader structure"]
impl crate::Readable for Tagheader2Spec {}
#[doc = "`reset()` method sets TAGHEADER2 to value 0xffff_ffff"]
impl crate::Resettable for Tagheader2Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
