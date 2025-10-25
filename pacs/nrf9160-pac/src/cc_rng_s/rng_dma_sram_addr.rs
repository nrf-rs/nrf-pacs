#[doc = "Register `RNG_DMA_SRAM_ADDR` reader"]
pub type R = crate::R<RngDmaSramAddrSpec>;
#[doc = "Register `RNG_DMA_SRAM_ADDR` writer"]
pub type W = crate::W<RngDmaSramAddrSpec>;
#[doc = "Field `RNG_SRAM_DMA_ADDR` reader - Start address of the TRNG data in TRNG SRAM."]
pub type RngSramDmaAddrR = crate::FieldReader<u16>;
#[doc = "Field `RNG_SRAM_DMA_ADDR` writer - Start address of the TRNG data in TRNG SRAM."]
pub type RngSramDmaAddrW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - Start address of the TRNG data in TRNG SRAM."]
    #[inline(always)]
    pub fn rng_sram_dma_addr(&self) -> RngSramDmaAddrR {
        RngSramDmaAddrR::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Start address of the TRNG data in TRNG SRAM."]
    #[inline(always)]
    pub fn rng_sram_dma_addr(&mut self) -> RngSramDmaAddrW<'_, RngDmaSramAddrSpec> {
        RngSramDmaAddrW::new(self, 0)
    }
}
#[doc = "This register defines the start address in TRNG SRAM for the TRNG data to be collected by the RNG DMA engine.\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_dma_sram_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_dma_sram_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RngDmaSramAddrSpec;
impl crate::RegisterSpec for RngDmaSramAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rng_dma_sram_addr::R`](R) reader structure"]
impl crate::Readable for RngDmaSramAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`rng_dma_sram_addr::W`](W) writer structure"]
impl crate::Writable for RngDmaSramAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RNG_DMA_SRAM_ADDR to value 0"]
impl crate::Resettable for RngDmaSramAddrSpec {}
