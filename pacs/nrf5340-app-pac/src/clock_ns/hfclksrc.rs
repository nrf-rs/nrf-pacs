#[doc = "Register `HFCLKSRC` reader"]
pub type R = crate::R<HfclksrcSpec>;
#[doc = "Register `HFCLKSRC` writer"]
pub type W = crate::W<HfclksrcSpec>;
#[doc = "Select which HFCLK source is started by the HFCLKSTART task\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Src {
    #[doc = "0: HFCLKSTART task starts HFINT oscillator"]
    Hfint = 0,
    #[doc = "1: HFCLKSTART task starts HFXO oscillator"]
    Hfxo = 1,
}
impl From<Src> for bool {
    #[inline(always)]
    fn from(variant: Src) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRC` reader - Select which HFCLK source is started by the HFCLKSTART task"]
pub type SrcR = crate::BitReader<Src>;
impl SrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Src {
        match self.bits {
            false => Src::Hfint,
            true => Src::Hfxo,
        }
    }
    #[doc = "HFCLKSTART task starts HFINT oscillator"]
    #[inline(always)]
    pub fn is_hfint(&self) -> bool {
        *self == Src::Hfint
    }
    #[doc = "HFCLKSTART task starts HFXO oscillator"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == Src::Hfxo
    }
}
#[doc = "Field `SRC` writer - Select which HFCLK source is started by the HFCLKSTART task"]
pub type SrcW<'a, REG> = crate::BitWriter<'a, REG, Src>;
impl<'a, REG> SrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HFCLKSTART task starts HFINT oscillator"]
    #[inline(always)]
    pub fn hfint(self) -> &'a mut crate::W<REG> {
        self.variant(Src::Hfint)
    }
    #[doc = "HFCLKSTART task starts HFXO oscillator"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut crate::W<REG> {
        self.variant(Src::Hfxo)
    }
}
impl R {
    #[doc = "Bit 0 - Select which HFCLK source is started by the HFCLKSTART task"]
    #[inline(always)]
    pub fn src(&self) -> SrcR {
        SrcR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Select which HFCLK source is started by the HFCLKSTART task"]
    #[inline(always)]
    pub fn src(&mut self) -> SrcW<'_, HfclksrcSpec> {
        SrcW::new(self, 0)
    }
}
#[doc = "Clock source for HFCLK128M/HFCLK64M\n\nYou can [`read`](crate::Reg::read) this register and get [`hfclksrc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfclksrc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HfclksrcSpec;
impl crate::RegisterSpec for HfclksrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfclksrc::R`](R) reader structure"]
impl crate::Readable for HfclksrcSpec {}
#[doc = "`write(|w| ..)` method takes [`hfclksrc::W`](W) writer structure"]
impl crate::Writable for HfclksrcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HFCLKSRC to value 0x01"]
impl crate::Resettable for HfclksrcSpec {
    const RESET_VALUE: u32 = 0x01;
}
