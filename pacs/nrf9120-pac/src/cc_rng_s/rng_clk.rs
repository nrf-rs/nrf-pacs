#[doc = "Register `RNG_CLK` writer"]
pub type W = crate::W<RngClkSpec>;
#[doc = "Enables clock for the RNG engine.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable {
    #[doc = "0: Disable clock for RNG engine."]
    Disable = 0,
    #[doc = "1: Enable clock for RNG engine."]
    Enable = 1,
}
impl From<Enable> for bool {
    #[inline(always)]
    fn from(variant: Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` writer - Enables clock for the RNG engine."]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG, Enable>;
impl<'a, REG> EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable clock for RNG engine."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Disable)
    }
    #[doc = "Enable clock for RNG engine."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Enable)
    }
}
impl W {
    #[doc = "Bit 0 - Enables clock for the RNG engine."]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, RngClkSpec> {
        EnableW::new(self, 0)
    }
}
#[doc = "Control clock for the RNG engine.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_clk::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RngClkSpec;
impl crate::RegisterSpec for RngClkSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`rng_clk::W`](W) writer structure"]
impl crate::Writable for RngClkSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RNG_CLK to value 0"]
impl crate::Resettable for RngClkSpec {}
