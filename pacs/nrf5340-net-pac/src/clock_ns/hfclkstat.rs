#[doc = "Register `HFCLKSTAT` reader"]
pub struct R(crate::R<HFCLKSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HFCLKSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HFCLKSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HFCLKSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Active clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRC_A {
    #[doc = "0: Clock source: HFINT - 128 MHz on-chip oscillator"]
    HFINT = 0,
    #[doc = "1: Clock source: HFXO - 128 MHz clock derived from external 32 MHz crystal oscillator"]
    HFXO = 1,
}
impl From<SRC_A> for bool {
    #[inline(always)]
    fn from(variant: SRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRC` reader - Active clock source"]
pub struct SRC_R(crate::FieldReader<bool, SRC_A>);
impl SRC_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRC_A {
        match self.bits {
            false => SRC_A::HFINT,
            true => SRC_A::HFXO,
        }
    }
    #[doc = "Checks if the value of the field is `HFINT`"]
    #[inline(always)]
    pub fn is_hfint(&self) -> bool {
        **self == SRC_A::HFINT
    }
    #[doc = "Checks if the value of the field is `HFXO`"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        **self == SRC_A::HFXO
    }
}
impl core::ops::Deref for SRC_R {
    type Target = crate::FieldReader<bool, SRC_A>;
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
#[doc = "HFCLK state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STATE_A {
    #[doc = "0: HFCLK not running"]
    NOTRUNNING = 0,
    #[doc = "1: HFCLK running"]
    RUNNING = 1,
}
impl From<STATE_A> for bool {
    #[inline(always)]
    fn from(variant: STATE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STATE` reader - HFCLK state"]
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
    #[doc = "Bit 0 - Active clock source"]
    #[inline(always)]
    pub fn src(&self) -> SRC_R {
        SRC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - ALWAYSRUN activated"]
    #[inline(always)]
    pub fn alwaysrunning(&self) -> ALWAYSRUNNING_R {
        ALWAYSRUNNING_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 16 - HFCLK state"]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
#[doc = "Status indicating which HFCLK128M/HFCLK64M source is running This register value in any CLOCK instance reflects status only due to configurations/actions in that CLOCK instance.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfclkstat](index.html) module"]
pub struct HFCLKSTAT_SPEC;
impl crate::RegisterSpec for HFCLKSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hfclkstat::R](R) reader structure"]
impl crate::Readable for HFCLKSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HFCLKSTAT to value 0"]
impl crate::Resettable for HFCLKSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
