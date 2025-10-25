#[doc = "Register `LFCLKSRCCOPY` reader"]
pub type R = crate::R<LfclksrccopySpec>;
#[doc = "Clock source\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Src {
    #[doc = "1: 32.768 kHz RC oscillator"]
    Lfrc = 1,
    #[doc = "2: 32.768 kHz crystal oscillator"]
    Lfxo = 2,
    #[doc = "3: 32.768 kHz synthesized from HFCLK"]
    Lfsynt = 3,
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
            1 => Some(Src::Lfrc),
            2 => Some(Src::Lfxo),
            3 => Some(Src::Lfsynt),
            _ => None,
        }
    }
    #[doc = "32.768 kHz RC oscillator"]
    #[inline(always)]
    pub fn is_lfrc(&self) -> bool {
        *self == Src::Lfrc
    }
    #[doc = "32.768 kHz crystal oscillator"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == Src::Lfxo
    }
    #[doc = "32.768 kHz synthesized from HFCLK"]
    #[inline(always)]
    pub fn is_lfsynt(&self) -> bool {
        *self == Src::Lfsynt
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
#[doc = "`reset()` method sets LFCLKSRCCOPY to value 0x01"]
impl crate::Resettable for LfclksrccopySpec {
    const RESET_VALUE: u32 = 0x01;
}
