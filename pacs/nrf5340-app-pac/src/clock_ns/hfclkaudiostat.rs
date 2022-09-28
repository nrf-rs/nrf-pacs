#[doc = "Register `HFCLKAUDIOSTAT` reader"]
pub struct R(crate::R<HFCLKAUDIOSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HFCLKAUDIOSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HFCLKAUDIOSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HFCLKAUDIOSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ALWAYSRUNNING` reader - ALWAYSRUN activated"]
pub type ALWAYSRUNNING_R = crate::BitReader<ALWAYSRUNNING_A>;
#[doc = "ALWAYSRUN activated\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALWAYSRUNNING_A {
    #[doc = "0: Automatic clock control enabled"]
    NOT_RUNNING = 0,
    #[doc = "1: Oscillator is always running"]
    RUNNING = 1,
}
impl From<ALWAYSRUNNING_A> for bool {
    #[inline(always)]
    fn from(variant: ALWAYSRUNNING_A) -> Self {
        variant as u8 != 0
    }
}
impl ALWAYSRUNNING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALWAYSRUNNING_A {
        match self.bits {
            false => ALWAYSRUNNING_A::NOT_RUNNING,
            true => ALWAYSRUNNING_A::RUNNING,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_RUNNING`"]
    #[inline(always)]
    pub fn is_not_running(&self) -> bool {
        *self == ALWAYSRUNNING_A::NOT_RUNNING
    }
    #[doc = "Checks if the value of the field is `RUNNING`"]
    #[inline(always)]
    pub fn is_running(&self) -> bool {
        *self == ALWAYSRUNNING_A::RUNNING
    }
}
#[doc = "Field `STATE` reader - HFCLKAUDIO state"]
pub type STATE_R = crate::BitReader<STATE_A>;
#[doc = "HFCLKAUDIO state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STATE_A {
    #[doc = "0: HFCLKAUDIO not running"]
    NOT_RUNNING = 0,
    #[doc = "1: HFCLKAUDIO running"]
    RUNNING = 1,
}
impl From<STATE_A> for bool {
    #[inline(always)]
    fn from(variant: STATE_A) -> Self {
        variant as u8 != 0
    }
}
impl STATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STATE_A {
        match self.bits {
            false => STATE_A::NOT_RUNNING,
            true => STATE_A::RUNNING,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_RUNNING`"]
    #[inline(always)]
    pub fn is_not_running(&self) -> bool {
        *self == STATE_A::NOT_RUNNING
    }
    #[doc = "Checks if the value of the field is `RUNNING`"]
    #[inline(always)]
    pub fn is_running(&self) -> bool {
        *self == STATE_A::RUNNING
    }
}
impl R {
    #[doc = "Bit 4 - ALWAYSRUN activated"]
    #[inline(always)]
    pub fn alwaysrunning(&self) -> ALWAYSRUNNING_R {
        ALWAYSRUNNING_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - HFCLKAUDIO state"]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "Status indicating which HFCLKAUDIO source is running\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfclkaudiostat](index.html) module"]
pub struct HFCLKAUDIOSTAT_SPEC;
impl crate::RegisterSpec for HFCLKAUDIOSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hfclkaudiostat::R](R) reader structure"]
impl crate::Readable for HFCLKAUDIOSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HFCLKAUDIOSTAT to value 0"]
impl crate::Resettable for HFCLKAUDIOSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
