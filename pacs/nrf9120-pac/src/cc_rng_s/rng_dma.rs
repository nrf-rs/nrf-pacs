#[doc = "Register `RNG_DMA` reader"]
pub type R = crate::R<RngDmaSpec>;
#[doc = "Register `RNG_DMA` writer"]
pub type W = crate::W<RngDmaSpec>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable {
    #[doc = "0: Disable RNG DMA engine"]
    Disable = 0,
    #[doc = "1: Enable RNG DMA engine This value is cleared when the RNG DMA engine completes its operation."]
    Enable = 1,
}
impl From<Enable> for bool {
    #[inline(always)]
    fn from(variant: Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - "]
pub type EnableR = crate::BitReader<Enable>;
impl EnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enable {
        match self.bits {
            false => Enable::Disable,
            true => Enable::Enable,
        }
    }
    #[doc = "Disable RNG DMA engine"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Enable::Disable
    }
    #[doc = "Enable RNG DMA engine This value is cleared when the RNG DMA engine completes its operation."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Enable::Enable
    }
}
#[doc = "Field `ENABLE` writer - "]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG, Enable>;
impl<'a, REG> EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable RNG DMA engine"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Disable)
    }
    #[doc = "Enable RNG DMA engine This value is cleared when the RNG DMA engine completes its operation."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Enable)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, RngDmaSpec> {
        EnableW::new(self, 0)
    }
}
#[doc = "Writing to this register enables the RNG DMA engine.\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_dma::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_dma::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RngDmaSpec;
impl crate::RegisterSpec for RngDmaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rng_dma::R`](R) reader structure"]
impl crate::Readable for RngDmaSpec {}
#[doc = "`write(|w| ..)` method takes [`rng_dma::W`](W) writer structure"]
impl crate::Writable for RngDmaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RNG_DMA to value 0"]
impl crate::Resettable for RngDmaSpec {}
