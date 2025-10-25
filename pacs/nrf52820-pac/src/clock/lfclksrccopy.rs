#[doc = "Register `LFCLKSRCCOPY` reader"]
pub type R = crate::R<LfclksrccopySpec>;
#[doc = "Clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Src {
    #[doc = "0: 32.768 kHz RC oscillator (LFRC)"]
    Rc = 0,
    #[doc = "1: 32.768 kHz crystal oscillator (LFXO)"]
    Xtal = 1,
    #[doc = "2: 32.768 kHz synthesized from HFCLK (LFSYNT)"]
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
#[doc = "Field `SRC` reader - Clock source"]
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
    #[doc = "32.768 kHz RC oscillator (LFRC)"]
    #[inline(always)]
    pub fn is_rc(&self) -> bool {
        *self == Src::Rc
    }
    #[doc = "32.768 kHz crystal oscillator (LFXO)"]
    #[inline(always)]
    pub fn is_xtal(&self) -> bool {
        *self == Src::Xtal
    }
    #[doc = "32.768 kHz synthesized from HFCLK (LFSYNT)"]
    #[inline(always)]
    pub fn is_synth(&self) -> bool {
        *self == Src::Synth
    }
}
impl R {
    #[doc = "Bits 0:1 - Clock source"]
    #[inline(always)]
    pub fn src(&self) -> SrcR {
        SrcR::new((self.bits & 3) as u8)
    }
}
#[doc = "Copy of LFCLKSRC register, set when LFCLKSTART task was triggered\n\nYou can [`read`](crate::Reg::read) this register and get [`lfclksrccopy::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfclksrccopySpec;
impl crate::RegisterSpec for LfclksrccopySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfclksrccopy::R`](R) reader structure"]
impl crate::Readable for LfclksrccopySpec {}
#[doc = "`reset()` method sets LFCLKSRCCOPY to value 0"]
impl crate::Resettable for LfclksrccopySpec {}
