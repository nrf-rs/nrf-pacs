#[doc = "Register `MAINREGSTATUS` reader"]
pub struct R(crate::R<MAINREGSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAINREGSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAINREGSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAINREGSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VREGH` reader - VREGH status"]
pub type VREGH_R = crate::BitReader<VREGH_A>;
#[doc = "VREGH status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VREGH_A {
    #[doc = "0: Normal voltage mode. Voltage supplied on VDD and VDDH."]
    INACTIVE = 0,
    #[doc = "1: High voltage mode. Voltage supplied on VDDH."]
    ACTIVE = 1,
}
impl From<VREGH_A> for bool {
    #[inline(always)]
    fn from(variant: VREGH_A) -> Self {
        variant as u8 != 0
    }
}
impl VREGH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VREGH_A {
        match self.bits {
            false => VREGH_A::INACTIVE,
            true => VREGH_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == VREGH_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == VREGH_A::ACTIVE
    }
}
impl R {
    #[doc = "Bit 0 - VREGH status"]
    #[inline(always)]
    pub fn vregh(&self) -> VREGH_R {
        VREGH_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Main supply status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mainregstatus](index.html) module"]
pub struct MAINREGSTATUS_SPEC;
impl crate::RegisterSpec for MAINREGSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mainregstatus::R](R) reader structure"]
impl crate::Readable for MAINREGSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MAINREGSTATUS to value 0"]
impl crate::Resettable for MAINREGSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
