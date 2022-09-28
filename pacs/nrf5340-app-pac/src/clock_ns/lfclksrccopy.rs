#[doc = "Register `LFCLKSRCCOPY` reader"]
pub struct R(crate::R<LFCLKSRCCOPY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LFCLKSRCCOPY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LFCLKSRCCOPY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LFCLKSRCCOPY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SRC` reader - Clock source"]
pub type SRC_R = crate::FieldReader<u8, SRC_A>;
#[doc = "Clock source\n\nValue on reset: 1"]
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
impl SRC_R {
    #[doc = "Get enumerated values variant"]
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
        *self == SRC_A::LFRC
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == SRC_A::LFXO
    }
    #[doc = "Checks if the value of the field is `LFSYNT`"]
    #[inline(always)]
    pub fn is_lfsynt(&self) -> bool {
        *self == SRC_A::LFSYNT
    }
}
impl R {
    #[doc = "Bits 0:1 - Clock source"]
    #[inline(always)]
    pub fn src(&self) -> SRC_R {
        SRC_R::new((self.bits & 3) as u8)
    }
}
#[doc = "Copy of LFCLKSRC register, set when LFCLKSTART task was triggered\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lfclksrccopy](index.html) module"]
pub struct LFCLKSRCCOPY_SPEC;
impl crate::RegisterSpec for LFCLKSRCCOPY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lfclksrccopy::R](R) reader structure"]
impl crate::Readable for LFCLKSRCCOPY_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LFCLKSRCCOPY to value 0x01"]
impl crate::Resettable for LFCLKSRCCOPY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
