#[doc = "Register `LFCLKSRCCOPY` reader"]
pub type R = crate::R<LfclksrccopySpec>;
#[doc = "Clock source for the LFCLK clock, set when task LKCLKSTART is triggered.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Src {
    #[doc = "0: Internal 32KiHz RC oscillator."]
    Rc = 0,
    #[doc = "1: External 32KiHz crystal."]
    Xtal = 1,
    #[doc = "2: Internal 32KiHz synthesizer from HFCLK system clock."]
    Synth = 2,
}
impl From<Src> for u8 {
    #[inline(always)]
    fn from(variant: Src) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Src {
    type Ux = u8;
}
impl crate::IsEnum for Src {}
#[doc = "Field `SRC` reader - Clock source for the LFCLK clock, set when task LKCLKSTART is triggered."]
pub type SrcR = crate::FieldReader<Src>;
impl SrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Src> {
        match self.bits {
            0 => Some(Src::Rc),
            1 => Some(Src::Xtal),
            2 => Some(Src::Synth),
            _ => None,
        }
    }
    #[doc = "Internal 32KiHz RC oscillator."]
    #[inline(always)]
    pub fn is_rc(&self) -> bool {
        *self == Src::Rc
    }
    #[doc = "External 32KiHz crystal."]
    #[inline(always)]
    pub fn is_xtal(&self) -> bool {
        *self == Src::Xtal
    }
    #[doc = "Internal 32KiHz synthesizer from HFCLK system clock."]
    #[inline(always)]
    pub fn is_synth(&self) -> bool {
        *self == Src::Synth
    }
}
impl R {
    #[doc = "Bits 0:1 - Clock source for the LFCLK clock, set when task LKCLKSTART is triggered."]
    #[inline(always)]
    pub fn src(&self) -> SrcR {
        SrcR::new((self.bits & 3) as u8)
    }
}
#[doc = "Clock source for the LFCLK clock, set when task LKCLKSTART is triggered.\n\nYou can [`read`](crate::Reg::read) this register and get [`lfclksrccopy::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfclksrccopySpec;
impl crate::RegisterSpec for LfclksrccopySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfclksrccopy::R`](R) reader structure"]
impl crate::Readable for LfclksrccopySpec {}
#[doc = "`reset()` method sets LFCLKSRCCOPY to value 0"]
impl crate::Resettable for LfclksrccopySpec {}
