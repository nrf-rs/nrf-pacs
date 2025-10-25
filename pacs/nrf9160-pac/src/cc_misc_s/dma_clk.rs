#[doc = "Register `DMA_CLK` writer"]
pub type W = crate::W<DmaClkSpec>;
#[doc = "Enables clock for the DMA engines.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable {
    #[doc = "0: Disable clock for the DMA engines."]
    Disable = 0,
    #[doc = "1: Enable clock for the DMA engines."]
    Enable = 1,
}
impl From<Enable> for bool {
    #[inline(always)]
    fn from(variant: Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` writer - Enables clock for the DMA engines."]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG, Enable>;
impl<'a, REG> EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable clock for the DMA engines."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Disable)
    }
    #[doc = "Enable clock for the DMA engines."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Enable)
    }
}
impl W {
    #[doc = "Bit 0 - Enables clock for the DMA engines."]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, DmaClkSpec> {
        EnableW::new(self, 0)
    }
}
#[doc = "Clock control for the DMA engines.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_clk::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaClkSpec;
impl crate::RegisterSpec for DmaClkSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dma_clk::W`](W) writer structure"]
impl crate::Writable for DmaClkSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_CLK to value 0"]
impl crate::Resettable for DmaClkSpec {}
