#[doc = "Register `DFESTATUS` reader"]
pub struct R(crate::R<DFESTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFESTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFESTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFESTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Internal state of switching state machine\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SWITCHINGSTATE_A {
    #[doc = "0: Switching state Idle"]
    IDLE = 0,
    #[doc = "1: Switching state Offset"]
    OFFSET = 1,
    #[doc = "2: Switching state Guard"]
    GUARD = 2,
    #[doc = "3: Switching state Ref"]
    REF = 3,
    #[doc = "4: Switching state Switching"]
    SWITCHING = 4,
    #[doc = "5: Switching state Ending"]
    ENDING = 5,
}
impl From<SWITCHINGSTATE_A> for u8 {
    #[inline(always)]
    fn from(variant: SWITCHINGSTATE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SWITCHINGSTATE` reader - Internal state of switching state machine"]
pub struct SWITCHINGSTATE_R(crate::FieldReader<u8, SWITCHINGSTATE_A>);
impl SWITCHINGSTATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SWITCHINGSTATE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SWITCHINGSTATE_A> {
        match self.bits {
            0 => Some(SWITCHINGSTATE_A::IDLE),
            1 => Some(SWITCHINGSTATE_A::OFFSET),
            2 => Some(SWITCHINGSTATE_A::GUARD),
            3 => Some(SWITCHINGSTATE_A::REF),
            4 => Some(SWITCHINGSTATE_A::SWITCHING),
            5 => Some(SWITCHINGSTATE_A::ENDING),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        **self == SWITCHINGSTATE_A::IDLE
    }
    #[doc = "Checks if the value of the field is `OFFSET`"]
    #[inline(always)]
    pub fn is_offset(&self) -> bool {
        **self == SWITCHINGSTATE_A::OFFSET
    }
    #[doc = "Checks if the value of the field is `GUARD`"]
    #[inline(always)]
    pub fn is_guard(&self) -> bool {
        **self == SWITCHINGSTATE_A::GUARD
    }
    #[doc = "Checks if the value of the field is `REF`"]
    #[inline(always)]
    pub fn is_ref(&self) -> bool {
        **self == SWITCHINGSTATE_A::REF
    }
    #[doc = "Checks if the value of the field is `SWITCHING`"]
    #[inline(always)]
    pub fn is_switching(&self) -> bool {
        **self == SWITCHINGSTATE_A::SWITCHING
    }
    #[doc = "Checks if the value of the field is `ENDING`"]
    #[inline(always)]
    pub fn is_ending(&self) -> bool {
        **self == SWITCHINGSTATE_A::ENDING
    }
}
impl core::ops::Deref for SWITCHINGSTATE_R {
    type Target = crate::FieldReader<u8, SWITCHINGSTATE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Internal state of sampling state machine\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAMPLINGSTATE_A {
    #[doc = "0: Sampling state Idle"]
    IDLE = 0,
    #[doc = "1: Sampling state Sampling"]
    SAMPLING = 1,
}
impl From<SAMPLINGSTATE_A> for bool {
    #[inline(always)]
    fn from(variant: SAMPLINGSTATE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SAMPLINGSTATE` reader - Internal state of sampling state machine"]
pub struct SAMPLINGSTATE_R(crate::FieldReader<bool, SAMPLINGSTATE_A>);
impl SAMPLINGSTATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SAMPLINGSTATE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAMPLINGSTATE_A {
        match self.bits {
            false => SAMPLINGSTATE_A::IDLE,
            true => SAMPLINGSTATE_A::SAMPLING,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        **self == SAMPLINGSTATE_A::IDLE
    }
    #[doc = "Checks if the value of the field is `SAMPLING`"]
    #[inline(always)]
    pub fn is_sampling(&self) -> bool {
        **self == SAMPLINGSTATE_A::SAMPLING
    }
}
impl core::ops::Deref for SAMPLINGSTATE_R {
    type Target = crate::FieldReader<bool, SAMPLINGSTATE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:2 - Internal state of switching state machine"]
    #[inline(always)]
    pub fn switchingstate(&self) -> SWITCHINGSTATE_R {
        SWITCHINGSTATE_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 4 - Internal state of sampling state machine"]
    #[inline(always)]
    pub fn samplingstate(&self) -> SAMPLINGSTATE_R {
        SAMPLINGSTATE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
#[doc = "DFE status information\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfestatus](index.html) module"]
pub struct DFESTATUS_SPEC;
impl crate::RegisterSpec for DFESTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfestatus::R](R) reader structure"]
impl crate::Readable for DFESTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DFESTATUS to value 0"]
impl crate::Resettable for DFESTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
