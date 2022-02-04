#[doc = "Register `SEMSTAT` reader"]
pub struct R(crate::R<SEMSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEMSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEMSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEMSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Semaphore status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SEMSTAT_A {
    #[doc = "0: Semaphore is free"]
    FREE = 0,
    #[doc = "1: Semaphore is assigned to CPU"]
    CPU = 1,
    #[doc = "2: Semaphore is assigned to SPI slave"]
    SPIS = 2,
    #[doc = "3: Semaphore is assigned to SPI but a handover to the CPU is pending"]
    CPUPENDING = 3,
}
impl From<SEMSTAT_A> for u8 {
    #[inline(always)]
    fn from(variant: SEMSTAT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SEMSTAT` reader - Semaphore status"]
pub struct SEMSTAT_R(crate::FieldReader<u8, SEMSTAT_A>);
impl SEMSTAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SEMSTAT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEMSTAT_A {
        match self.bits {
            0 => SEMSTAT_A::FREE,
            1 => SEMSTAT_A::CPU,
            2 => SEMSTAT_A::SPIS,
            3 => SEMSTAT_A::CPUPENDING,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FREE`"]
    #[inline(always)]
    pub fn is_free(&self) -> bool {
        **self == SEMSTAT_A::FREE
    }
    #[doc = "Checks if the value of the field is `CPU`"]
    #[inline(always)]
    pub fn is_cpu(&self) -> bool {
        **self == SEMSTAT_A::CPU
    }
    #[doc = "Checks if the value of the field is `SPIS`"]
    #[inline(always)]
    pub fn is_spis(&self) -> bool {
        **self == SEMSTAT_A::SPIS
    }
    #[doc = "Checks if the value of the field is `CPUPENDING`"]
    #[inline(always)]
    pub fn is_cpupending(&self) -> bool {
        **self == SEMSTAT_A::CPUPENDING
    }
}
impl core::ops::Deref for SEMSTAT_R {
    type Target = crate::FieldReader<u8, SEMSTAT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:1 - Semaphore status"]
    #[inline(always)]
    pub fn semstat(&self) -> SEMSTAT_R {
        SEMSTAT_R::new((self.bits & 0x03) as u8)
    }
}
#[doc = "Semaphore status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [semstat](index.html) module"]
pub struct SEMSTAT_SPEC;
impl crate::RegisterSpec for SEMSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [semstat::R](R) reader structure"]
impl crate::Readable for SEMSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SEMSTAT to value 0x01"]
impl crate::Resettable for SEMSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
