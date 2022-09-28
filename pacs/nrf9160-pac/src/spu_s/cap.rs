#[doc = "Register `CAP` reader"]
pub struct R(crate::R<CAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TZM` reader - Show ARM TrustZone status"]
pub type TZM_R = crate::BitReader<TZM_A>;
#[doc = "Show ARM TrustZone status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TZM_A {
    #[doc = "0: ARM TrustZone support not available"]
    NOT_AVAILABLE = 0,
    #[doc = "1: ARM TrustZone support is available"]
    ENABLED = 1,
}
impl From<TZM_A> for bool {
    #[inline(always)]
    fn from(variant: TZM_A) -> Self {
        variant as u8 != 0
    }
}
impl TZM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TZM_A {
        match self.bits {
            false => TZM_A::NOT_AVAILABLE,
            true => TZM_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_AVAILABLE`"]
    #[inline(always)]
    pub fn is_not_available(&self) -> bool {
        *self == TZM_A::NOT_AVAILABLE
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TZM_A::ENABLED
    }
}
impl R {
    #[doc = "Bit 0 - Show ARM TrustZone status"]
    #[inline(always)]
    pub fn tzm(&self) -> TZM_R {
        TZM_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Show implemented features for the current device\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cap](index.html) module"]
pub struct CAP_SPEC;
impl crate::RegisterSpec for CAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cap::R](R) reader structure"]
impl crate::Readable for CAP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CAP to value 0x01"]
impl crate::Resettable for CAP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
