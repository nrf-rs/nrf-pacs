#[doc = "Register `ISOOUT` reader"]
pub struct R(crate::R<ISOOUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISOOUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISOOUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISOOUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SIZE` reader - Number of bytes received last on this ISO OUT data endpoint"]
pub struct SIZE_R(crate::FieldReader<u16, u16>);
impl SIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIZE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Zero-length data packet received\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ZERO_A {
    #[doc = "0: No zero-length data received, use value in SIZE"]
    NORMAL = 0,
    #[doc = "1: Zero-length data received, ignore value in SIZE"]
    ZERODATA = 1,
}
impl From<ZERO_A> for bool {
    #[inline(always)]
    fn from(variant: ZERO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ZERO` reader - Zero-length data packet received"]
pub struct ZERO_R(crate::FieldReader<bool, ZERO_A>);
impl ZERO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ZERO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ZERO_A {
        match self.bits {
            false => ZERO_A::NORMAL,
            true => ZERO_A::ZERODATA,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == ZERO_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `ZERODATA`"]
    #[inline(always)]
    pub fn is_zero_data(&self) -> bool {
        **self == ZERO_A::ZERODATA
    }
}
impl core::ops::Deref for ZERO_R {
    type Target = crate::FieldReader<bool, ZERO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:9 - Number of bytes received last on this ISO OUT data endpoint"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 16 - Zero-length data packet received"]
    #[inline(always)]
    pub fn zero(&self) -> ZERO_R {
        ZERO_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
#[doc = "Number of bytes received last on this ISO OUT data endpoint\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isoout](index.html) module"]
pub struct ISOOUT_SPEC;
impl crate::RegisterSpec for ISOOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isoout::R](R) reader structure"]
impl crate::Readable for ISOOUT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ISOOUT to value 0x0001_0000"]
impl crate::Resettable for ISOOUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0001_0000
    }
}
