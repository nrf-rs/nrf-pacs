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
#[doc = "ALWAYSRUN activated\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALWAYSRUNNING_A {
    #[doc = "0: Automatic clock control enabled"]
    NOTRUNNING = 0,
    #[doc = "1: Oscillator is always running"]
    RUNNING = 1,
}
impl From<ALWAYSRUNNING_A> for bool {
    #[inline(always)]
    fn from(variant: ALWAYSRUNNING_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALWAYSRUNNING` reader - ALWAYSRUN activated"]
pub struct ALWAYSRUNNING_R(crate::FieldReader<bool, ALWAYSRUNNING_A>);
impl ALWAYSRUNNING_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALWAYSRUNNING_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALWAYSRUNNING_A {
        match self.bits {
            false => ALWAYSRUNNING_A::NOTRUNNING,
            true => ALWAYSRUNNING_A::RUNNING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTRUNNING`"]
    #[inline(always)]
    pub fn is_not_running(&self) -> bool {
        **self == ALWAYSRUNNING_A::NOTRUNNING
    }
    #[doc = "Checks if the value of the field is `RUNNING`"]
    #[inline(always)]
    pub fn is_running(&self) -> bool {
        **self == ALWAYSRUNNING_A::RUNNING
    }
}
impl core::ops::Deref for ALWAYSRUNNING_R {
    type Target = crate::FieldReader<bool, ALWAYSRUNNING_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "HFCLKAUDIO state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STATE_A {
    #[doc = "0: HFCLKAUDIO not running"]
    NOTRUNNING = 0,
    #[doc = "1: HFCLKAUDIO running"]
    RUNNING = 1,
}
impl From<STATE_A> for bool {
    #[inline(always)]
    fn from(variant: STATE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STATE` reader - HFCLKAUDIO state"]
pub struct STATE_R(crate::FieldReader<bool, STATE_A>);
impl STATE_R {
    pub(crate) fn new(bits: bool) -> Self {
        STATE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STATE_A {
        match self.bits {
            false => STATE_A::NOTRUNNING,
            true => STATE_A::RUNNING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTRUNNING`"]
    #[inline(always)]
    pub fn is_not_running(&self) -> bool {
        **self == STATE_A::NOTRUNNING
    }
    #[doc = "Checks if the value of the field is `RUNNING`"]
    #[inline(always)]
    pub fn is_running(&self) -> bool {
        **self == STATE_A::RUNNING
    }
}
impl core::ops::Deref for STATE_R {
    type Target = crate::FieldReader<bool, STATE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 4 - ALWAYSRUN activated"]
    #[inline(always)]
    pub fn alwaysrunning(&self) -> ALWAYSRUNNING_R {
        ALWAYSRUNNING_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 16 - HFCLKAUDIO state"]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new(((self.bits >> 16) & 0x01) != 0)
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
