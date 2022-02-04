#[doc = "Register `TAGHEADER2` reader"]
pub struct R(crate::R<TAGHEADER2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAGHEADER2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAGHEADER2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAGHEADER2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `UD8` reader - Unique identifier byte 8"]
pub struct UD8_R(crate::FieldReader<u8, u8>);
impl UD8_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        UD8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UD8_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UD9` reader - Unique identifier byte 9"]
pub struct UD9_R(crate::FieldReader<u8, u8>);
impl UD9_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        UD9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UD9_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UD10` reader - Unique identifier byte 10"]
pub struct UD10_R(crate::FieldReader<u8, u8>);
impl UD10_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        UD10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UD10_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UD11` reader - Unique identifier byte 11"]
pub struct UD11_R(crate::FieldReader<u8, u8>);
impl UD11_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        UD11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UD11_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Unique identifier byte 8"]
    #[inline(always)]
    pub fn ud8(&self) -> UD8_R {
        UD8_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Unique identifier byte 9"]
    #[inline(always)]
    pub fn ud9(&self) -> UD9_R {
        UD9_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Unique identifier byte 10"]
    #[inline(always)]
    pub fn ud10(&self) -> UD10_R {
        UD10_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Unique identifier byte 11"]
    #[inline(always)]
    pub fn ud11(&self) -> UD11_R {
        UD11_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Default header for NFC tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST, and NFCID1_LAST.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tagheader2](index.html) module"]
pub struct TAGHEADER2_SPEC;
impl crate::RegisterSpec for TAGHEADER2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tagheader2::R](R) reader structure"]
impl crate::Readable for TAGHEADER2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TAGHEADER2 to value 0xffff_ffff"]
impl crate::Resettable for TAGHEADER2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
