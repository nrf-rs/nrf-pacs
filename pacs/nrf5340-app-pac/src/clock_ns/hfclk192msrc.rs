#[doc = "Register `HFCLK192MSRC` reader"]
pub type R = crate::R<Hfclk192msrcSpec>;
#[doc = "Register `HFCLK192MSRC` writer"]
pub type W = crate::W<Hfclk192msrcSpec>;
#[doc = "Select which HFCLK192M source is started by the HFCLK192MSTART task\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Src {
    #[doc = "0: HFCLK192MSTART task starts HFINT oscillator"]
    Hfint = 0,
    #[doc = "1: HFCLK192MSTART task starts HFXO oscillator"]
    Hfxo = 1,
}
impl From<Src> for bool {
    #[inline(always)]
    fn from(variant: Src) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRC` reader - Select which HFCLK192M source is started by the HFCLK192MSTART task"]
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
    #[doc = "HFCLK192MSTART task starts HFINT oscillator"]
    #[inline(always)]
    pub fn is_hfint(&self) -> bool {
        *self == Src::Hfint
    }
    #[doc = "HFCLK192MSTART task starts HFXO oscillator"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == Src::Hfxo
    }
}
#[doc = "Field `SRC` writer - Select which HFCLK192M source is started by the HFCLK192MSTART task"]
pub type SrcW<'a, REG> = crate::BitWriter<'a, REG, Src>;
impl<'a, REG> SrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HFCLK192MSTART task starts HFINT oscillator"]
    #[inline(always)]
    pub fn hfint(self) -> &'a mut crate::W<REG> {
        self.variant(Src::Hfint)
    }
    #[doc = "HFCLK192MSTART task starts HFXO oscillator"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut crate::W<REG> {
        self.variant(Src::Hfxo)
    }
}
impl R {
    #[doc = "Bit 0 - Select which HFCLK192M source is started by the HFCLK192MSTART task"]
    #[inline(always)]
    pub fn src(&self) -> SrcR {
        SrcR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Select which HFCLK192M source is started by the HFCLK192MSTART task"]
    #[inline(always)]
    pub fn src(&mut self) -> SrcW<'_, Hfclk192msrcSpec> {
        SrcW::new(self, 0)
    }
}
#[doc = "Clock source for HFCLK192M\n\nYou can [`read`](crate::Reg::read) this register and get [`hfclk192msrc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfclk192msrc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hfclk192msrcSpec;
impl crate::RegisterSpec for Hfclk192msrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfclk192msrc::R`](R) reader structure"]
impl crate::Readable for Hfclk192msrcSpec {}
#[doc = "`write(|w| ..)` method takes [`hfclk192msrc::W`](W) writer structure"]
impl crate::Writable for Hfclk192msrcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HFCLK192MSRC to value 0x01"]
impl crate::Resettable for Hfclk192msrcSpec {
    const RESET_VALUE: u32 = 0x01;
}
