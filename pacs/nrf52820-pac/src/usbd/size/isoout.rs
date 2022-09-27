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
pub type SIZE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ZERO` reader - Zero-length data packet received"]
pub type ZERO_R = crate::BitReader<ZERO_A>;
#[doc = "Zero-length data packet received\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ZERO_A {
    #[doc = "0: No zero-length data received, use value in SIZE"]
    NORMAL = 0,
    #[doc = "1: Zero-length data received, ignore value in SIZE"]
    ZERO_DATA = 1,
}
impl From<ZERO_A> for bool {
    #[inline(always)]
    fn from(variant: ZERO_A) -> Self {
        variant as u8 != 0
    }
}
impl ZERO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ZERO_A {
        match self.bits {
            false => ZERO_A::NORMAL,
            true => ZERO_A::ZERO_DATA,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == ZERO_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `ZERO_DATA`"]
    #[inline(always)]
    pub fn is_zero_data(&self) -> bool {
        *self == ZERO_A::ZERO_DATA
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
        ZERO_R::new(((self.bits >> 16) & 1) != 0)
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
