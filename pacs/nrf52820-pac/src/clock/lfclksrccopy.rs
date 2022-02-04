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
#[doc = "Clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SRC_A {
    #[doc = "0: 32.768 kHz RC oscillator (LFRC)"]
    RC = 0,
    #[doc = "1: 32.768 kHz crystal oscillator (LFXO)"]
    XTAL = 1,
    #[doc = "2: 32.768 kHz synthesized from HFCLK (LFSYNT)"]
    SYNTH = 2,
}
impl From<SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: SRC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SRC` reader - Clock source"]
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
            0 => Some(SRC_A::RC),
            1 => Some(SRC_A::XTAL),
            2 => Some(SRC_A::SYNTH),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RC`"]
    #[inline(always)]
    pub fn is_rc(&self) -> bool {
        **self == SRC_A::RC
    }
    #[doc = "Checks if the value of the field is `XTAL`"]
    #[inline(always)]
    pub fn is_xtal(&self) -> bool {
        **self == SRC_A::XTAL
    }
    #[doc = "Checks if the value of the field is `SYNTH`"]
    #[inline(always)]
    pub fn is_synth(&self) -> bool {
        **self == SRC_A::SYNTH
    }
}
impl core::ops::Deref for SRC_R {
    type Target = crate::FieldReader<u8, SRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:1 - Clock source"]
    #[inline(always)]
    pub fn src(&self) -> SRC_R {
        SRC_R::new((self.bits & 0x03) as u8)
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
#[doc = "`reset()` method sets LFCLKSRCCOPY to value 0"]
impl crate::Resettable for LFCLKSRCCOPY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
