#[doc = "Register `LFCLKSTAT` reader"]
pub struct R(crate::R<LFCLKSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LFCLKSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LFCLKSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LFCLKSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Active clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SRC_A {
    #[doc = "1: 32.768 kHz RC oscillator"]
    LFRC = 1,
    #[doc = "2: 32.768 kHz crystal oscillator"]
    LFXO = 2,
    #[doc = "3: 32.768 kHz synthesized from HFCLK"]
    LFSYNT = 3,
}
impl From<SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: SRC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SRC` reader - Active clock source"]
pub struct SRC_R(crate::FieldReader<u8, SRC_A>);
impl SRC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SRC_A> {
        match self.bits {
            1 => Some(SRC_A::LFRC),
            2 => Some(SRC_A::LFXO),
            3 => Some(SRC_A::LFSYNT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LFRC`"]
    #[inline(always)]
    pub fn is_lfrc(&self) -> bool {
        **self == SRC_A::LFRC
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        **self == SRC_A::LFXO
    }
    #[doc = "Checks if the value of the field is `LFSYNT`"]
    #[inline(always)]
    pub fn is_lfsynt(&self) -> bool {
        **self == SRC_A::LFSYNT
    }
}
impl core::ops::Deref for SRC_R {
    type Target = crate::FieldReader<u8, SRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
    #[inline(always)]
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
#[doc = "LFCLK state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STATE_A {
    #[doc = "0: LFCLK not running"]
    NOTRUNNING = 0,
    #[doc = "1: LFCLK running"]
    RUNNING = 1,
}
impl From<STATE_A> for bool {
    #[inline(always)]
    fn from(variant: STATE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STATE` reader - LFCLK state"]
pub struct STATE_R(crate::FieldReader<bool, STATE_A>);
impl STATE_R {
    #[inline(always)]
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
    #[doc = "Bits 0:1 - Active clock source"]
    #[inline(always)]
    pub fn src(&self) -> SRC_R {
        SRC_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 4 - ALWAYSRUN activated"]
    #[inline(always)]
    pub fn alwaysrunning(&self) -> ALWAYSRUNNING_R {
        ALWAYSRUNNING_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 16 - LFCLK state"]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
#[doc = "Status indicating which LFCLK source is running This register value in any CLOCK instance reflects status only due to configurations/actions in that CLOCK instance.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lfclkstat](index.html) module"]
pub struct LFCLKSTAT_SPEC;
impl crate::RegisterSpec for LFCLKSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lfclkstat::R](R) reader structure"]
impl crate::Readable for LFCLKSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LFCLKSTAT to value 0"]
impl crate::Resettable for LFCLKSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
