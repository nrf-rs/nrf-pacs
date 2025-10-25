#[doc = "Register `LFCLKSRC` reader"]
pub type R = crate::R<LfclksrcSpec>;
#[doc = "Register `LFCLKSRC` writer"]
pub type W = crate::W<LfclksrcSpec>;
#[doc = "Clock source.\n\nValue on reset: 0"]
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
#[doc = "Field `SRC` reader - Clock source."]
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
#[doc = "Field `SRC` writer - Clock source."]
pub type SrcW<'a, REG> = crate::FieldWriter<'a, REG, 2, Src>;
impl<'a, REG> SrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal 32KiHz RC oscillator."]
    #[inline(always)]
    pub fn rc(self) -> &'a mut crate::W<REG> {
        self.variant(Src::Rc)
    }
    #[doc = "External 32KiHz crystal."]
    #[inline(always)]
    pub fn xtal(self) -> &'a mut crate::W<REG> {
        self.variant(Src::Xtal)
    }
    #[doc = "Internal 32KiHz synthesizer from HFCLK system clock."]
    #[inline(always)]
    pub fn synth(self) -> &'a mut crate::W<REG> {
        self.variant(Src::Synth)
    }
}
impl R {
    #[doc = "Bits 0:1 - Clock source."]
    #[inline(always)]
    pub fn src(&self) -> SrcR {
        SrcR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Clock source."]
    #[inline(always)]
    pub fn src(&mut self) -> SrcW<'_, LfclksrcSpec> {
        SrcW::new(self, 0)
    }
}
#[doc = "Clock source for the LFCLK clock.\n\nYou can [`read`](crate::Reg::read) this register and get [`lfclksrc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfclksrc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfclksrcSpec;
impl crate::RegisterSpec for LfclksrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfclksrc::R`](R) reader structure"]
impl crate::Readable for LfclksrcSpec {}
#[doc = "`write(|w| ..)` method takes [`lfclksrc::W`](W) writer structure"]
impl crate::Writable for LfclksrcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFCLKSRC to value 0"]
impl crate::Resettable for LfclksrcSpec {}
