#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Deep power-down mode (DPM) status of external flash.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPM_A {
    #[doc = "0: External flash is not in DPM."]
    DISABLED = 0,
    #[doc = "1: External flash is in DPM."]
    ENABLED = 1,
}
impl From<DPM_A> for bool {
    #[inline(always)]
    fn from(variant: DPM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DPM` reader - Deep power-down mode (DPM) status of external flash."]
pub struct DPM_R(crate::FieldReader<bool, DPM_A>);
impl DPM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DPM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPM_A {
        match self.bits {
            false => DPM_A::DISABLED,
            true => DPM_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DPM_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DPM_A::ENABLED
    }
}
impl core::ops::Deref for DPM_R {
    type Target = crate::FieldReader<bool, DPM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Ready status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READY_A {
    #[doc = "1: QSPI peripheral is ready. It is allowed to trigger new tasks, writing custom instructions or enter/exit DPM."]
    READY = 1,
    #[doc = "0: QSPI peripheral is busy. It is not allowed to trigger any new tasks, writing custom instructions or enter/exit DPM."]
    BUSY = 0,
}
impl From<READY_A> for bool {
    #[inline(always)]
    fn from(variant: READY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READY` reader - Ready status."]
pub struct READY_R(crate::FieldReader<bool, READY_A>);
impl READY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        READY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> READY_A {
        match self.bits {
            true => READY_A::READY,
            false => READY_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        **self == READY_A::READY
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        **self == READY_A::BUSY
    }
}
impl core::ops::Deref for READY_R {
    type Target = crate::FieldReader<bool, READY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SREG` reader - Value of external flash device Status Register. When the external flash has two bytes status register this field includes the value of the low byte."]
pub struct SREG_R(crate::FieldReader<u8, u8>);
impl SREG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SREG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SREG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 2 - Deep power-down mode (DPM) status of external flash."]
    #[inline(always)]
    pub fn dpm(&self) -> DPM_R {
        DPM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Ready status."]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 24:31 - Value of external flash device Status Register. When the external flash has two bytes status register this field includes the value of the low byte."]
    #[inline(always)]
    pub fn sreg(&self) -> SREG_R {
        SREG_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Status register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
