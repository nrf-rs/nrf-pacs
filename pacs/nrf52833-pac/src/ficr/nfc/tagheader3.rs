#[doc = "Register `TAGHEADER3` reader"]
pub struct R(crate::R<TAGHEADER3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAGHEADER3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAGHEADER3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAGHEADER3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `UD12` reader - Unique identifier byte 12"]
pub struct UD12_R(crate::FieldReader<u8, u8>);
impl UD12_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        UD12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UD12_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UD13` reader - Unique identifier byte 13"]
pub struct UD13_R(crate::FieldReader<u8, u8>);
impl UD13_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        UD13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UD13_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UD14` reader - Unique identifier byte 14"]
pub struct UD14_R(crate::FieldReader<u8, u8>);
impl UD14_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        UD14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UD14_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UD15` reader - Unique identifier byte 15"]
pub struct UD15_R(crate::FieldReader<u8, u8>);
impl UD15_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        UD15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UD15_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Unique identifier byte 12"]
    #[inline(always)]
    pub fn ud12(&self) -> UD12_R {
        UD12_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Unique identifier byte 13"]
    #[inline(always)]
    pub fn ud13(&self) -> UD13_R {
        UD13_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Unique identifier byte 14"]
    #[inline(always)]
    pub fn ud14(&self) -> UD14_R {
        UD14_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Unique identifier byte 15"]
    #[inline(always)]
    pub fn ud15(&self) -> UD15_R {
        UD15_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Default header for NFC tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST, and NFCID1_LAST.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tagheader3](index.html) module"]
pub struct TAGHEADER3_SPEC;
impl crate::RegisterSpec for TAGHEADER3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tagheader3::R](R) reader structure"]
impl crate::Readable for TAGHEADER3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TAGHEADER3 to value 0xffff_ffff"]
impl crate::Resettable for TAGHEADER3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
