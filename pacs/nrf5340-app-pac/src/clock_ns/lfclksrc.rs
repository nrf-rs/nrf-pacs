#[doc = "Register `LFCLKSRC` reader"]
pub type R = crate::R<LfclksrcSpec>;
#[doc = "Register `LFCLKSRC` writer"]
pub type W = crate::W<LfclksrcSpec>;
#[doc = "Select which LFCLK source is started by the LFCLKSTART task\n\nValue on reset: 1"]
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
#[doc = "Field `SRC` reader - Select which LFCLK source is started by the LFCLKSTART task"]
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
#[doc = "Field `SRC` writer - Select which LFCLK source is started by the LFCLKSTART task"]
pub type SrcW<'a, REG> = crate::FieldWriter<'a, REG, 2, Src>;
impl<'a, REG> SrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "32.768 kHz RC oscillator"]
    #[inline(always)]
    pub fn lfrc(self) -> &'a mut crate::W<REG> {
        self.variant(Src::Lfrc)
    }
    #[doc = "32.768 kHz crystal oscillator"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut crate::W<REG> {
        self.variant(Src::Lfxo)
    }
    #[doc = "32.768 kHz synthesized from HFCLK"]
    #[inline(always)]
    pub fn lfsynt(self) -> &'a mut crate::W<REG> {
        self.variant(Src::Lfsynt)
    }
}
impl R {
    #[doc = "Bits 0:1 - Select which LFCLK source is started by the LFCLKSTART task"]
    #[inline(always)]
    pub fn src(&self) -> SrcR {
        SrcR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Select which LFCLK source is started by the LFCLKSTART task"]
    #[inline(always)]
    pub fn src(&mut self) -> SrcW<'_, LfclksrcSpec> {
        SrcW::new(self, 0)
    }
}
#[doc = "Clock source for LFCLK\n\nYou can [`read`](crate::Reg::read) this register and get [`lfclksrc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfclksrc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets LFCLKSRC to value 0x01"]
impl crate::Resettable for LfclksrcSpec {
    const RESET_VALUE: u32 = 0x01;
}
