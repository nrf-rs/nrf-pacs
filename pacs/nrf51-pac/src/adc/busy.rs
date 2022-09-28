#[doc = "Register `BUSY` reader"]
pub struct R(crate::R<BUSY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUSY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUSY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUSY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BUSY` reader - ADC busy register."]
pub type BUSY_R = crate::BitReader<BUSY_A>;
#[doc = "ADC busy register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSY_A {
    #[doc = "0: No ongoing ADC conversion is taking place. ADC is ready."]
    READY = 0,
    #[doc = "1: An ADC conversion is taking place. ADC is busy."]
    BUSY = 1,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY_A {
        match self.bits {
            false => BUSY_A::READY,
            true => BUSY_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == BUSY_A::READY
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY_A::BUSY
    }
}
impl R {
    #[doc = "Bit 0 - ADC busy register."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 1) != 0)
    }
}
#[doc = "ADC busy register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [busy](index.html) module"]
pub struct BUSY_SPEC;
impl crate::RegisterSpec for BUSY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [busy::R](R) reader structure"]
impl crate::Readable for BUSY_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BUSY to value 0"]
impl crate::Resettable for BUSY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
