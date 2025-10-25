#[doc = "Register `LEDPOL` reader"]
pub type R = crate::R<LedpolSpec>;
#[doc = "Register `LEDPOL` writer"]
pub type W = crate::W<LedpolSpec>;
#[doc = "LED output pin polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ledpol {
    #[doc = "0: Led active on output pin low"]
    ActiveLow = 0,
    #[doc = "1: Led active on output pin high"]
    ActiveHigh = 1,
}
impl From<Ledpol> for bool {
    #[inline(always)]
    fn from(variant: Ledpol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LEDPOL` reader - LED output pin polarity"]
pub type LedpolR = crate::BitReader<Ledpol>;
impl LedpolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ledpol {
        match self.bits {
            false => Ledpol::ActiveLow,
            true => Ledpol::ActiveHigh,
        }
    }
    #[doc = "Led active on output pin low"]
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == Ledpol::ActiveLow
    }
    #[doc = "Led active on output pin high"]
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == Ledpol::ActiveHigh
    }
}
#[doc = "Field `LEDPOL` writer - LED output pin polarity"]
pub type LedpolW<'a, REG> = crate::BitWriter<'a, REG, Ledpol>;
impl<'a, REG> LedpolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Led active on output pin low"]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut crate::W<REG> {
        self.variant(Ledpol::ActiveLow)
    }
    #[doc = "Led active on output pin high"]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut crate::W<REG> {
        self.variant(Ledpol::ActiveHigh)
    }
}
impl R {
    #[doc = "Bit 0 - LED output pin polarity"]
    #[inline(always)]
    pub fn ledpol(&self) -> LedpolR {
        LedpolR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LED output pin polarity"]
    #[inline(always)]
    pub fn ledpol(&mut self) -> LedpolW<'_, LedpolSpec> {
        LedpolW::new(self, 0)
    }
}
#[doc = "LED output pin polarity\n\nYou can [`read`](crate::Reg::read) this register and get [`ledpol::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ledpol::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LedpolSpec;
impl crate::RegisterSpec for LedpolSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ledpol::R`](R) reader structure"]
impl crate::Readable for LedpolSpec {}
#[doc = "`write(|w| ..)` method takes [`ledpol::W`](W) writer structure"]
impl crate::Writable for LedpolSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LEDPOL to value 0"]
impl crate::Resettable for LedpolSpec {}
