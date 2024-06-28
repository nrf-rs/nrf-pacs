#[doc = "Register `HFXOSRC` reader"]
pub type R = crate::R<HfxosrcSpec>;
#[doc = "Register `HFXOSRC` writer"]
pub type W = crate::W<HfxosrcSpec>;
#[doc = "HFXO clock source selection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hfxosrc {
    #[doc = "1: 32 MHz crystal oscillator"]
    Xtal = 1,
    #[doc = "0: 32 MHz temperature compensated crystal oscillator (TCXO)"]
    Tcxo = 0,
}
impl From<Hfxosrc> for bool {
    #[inline(always)]
    fn from(variant: Hfxosrc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HFXOSRC` reader - HFXO clock source selection"]
pub type HfxosrcR = crate::BitReader<Hfxosrc>;
impl HfxosrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hfxosrc {
        match self.bits {
            true => Hfxosrc::Xtal,
            false => Hfxosrc::Tcxo,
        }
    }
    #[doc = "32 MHz crystal oscillator"]
    #[inline(always)]
    pub fn is_xtal(&self) -> bool {
        *self == Hfxosrc::Xtal
    }
    #[doc = "32 MHz temperature compensated crystal oscillator (TCXO)"]
    #[inline(always)]
    pub fn is_tcxo(&self) -> bool {
        *self == Hfxosrc::Tcxo
    }
}
#[doc = "Field `HFXOSRC` writer - HFXO clock source selection"]
pub type HfxosrcW<'a, REG> = crate::BitWriter<'a, REG, Hfxosrc>;
impl<'a, REG> HfxosrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "32 MHz crystal oscillator"]
    #[inline(always)]
    pub fn xtal(self) -> &'a mut crate::W<REG> {
        self.variant(Hfxosrc::Xtal)
    }
    #[doc = "32 MHz temperature compensated crystal oscillator (TCXO)"]
    #[inline(always)]
    pub fn tcxo(self) -> &'a mut crate::W<REG> {
        self.variant(Hfxosrc::Tcxo)
    }
}
impl R {
    #[doc = "Bit 0 - HFXO clock source selection"]
    #[inline(always)]
    pub fn hfxosrc(&self) -> HfxosrcR {
        HfxosrcR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HFXO clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn hfxosrc(&mut self) -> HfxosrcW<HfxosrcSpec> {
        HfxosrcW::new(self, 0)
    }
}
#[doc = "HFXO clock source selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfxosrc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfxosrc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HfxosrcSpec;
impl crate::RegisterSpec for HfxosrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfxosrc::R`](R) reader structure"]
impl crate::Readable for HfxosrcSpec {}
#[doc = "`write(|w| ..)` method takes [`hfxosrc::W`](W) writer structure"]
impl crate::Writable for HfxosrcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HFXOSRC to value 0xffff_ffff"]
impl crate::Resettable for HfxosrcSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
