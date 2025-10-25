#[doc = "Register `RNG_DMA_SAMPLES_NUM` reader"]
pub type R = crate::R<RngDmaSamplesNumSpec>;
#[doc = "Register `RNG_DMA_SAMPLES_NUM` writer"]
pub type W = crate::W<RngDmaSamplesNumSpec>;
#[doc = "Field `RNG_SAMPLES_NUM` reader - Defines the number of 192-bits samples that the DMA engine collects per run."]
pub type RngSamplesNumR = crate::FieldReader;
#[doc = "Field `RNG_SAMPLES_NUM` writer - Defines the number of 192-bits samples that the DMA engine collects per run."]
pub type RngSamplesNumW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Defines the number of 192-bits samples that the DMA engine collects per run."]
    #[inline(always)]
    pub fn rng_samples_num(&self) -> RngSamplesNumR {
        RngSamplesNumR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Defines the number of 192-bits samples that the DMA engine collects per run."]
    #[inline(always)]
    pub fn rng_samples_num(&mut self) -> RngSamplesNumW<'_, RngDmaSamplesNumSpec> {
        RngSamplesNumW::new(self, 0)
    }
}
#[doc = "This register defines the number of 192-bits samples that the RNG DMA engine collects per run.\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_dma_samples_num::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_dma_samples_num::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RngDmaSamplesNumSpec;
impl crate::RegisterSpec for RngDmaSamplesNumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rng_dma_samples_num::R`](R) reader structure"]
impl crate::Readable for RngDmaSamplesNumSpec {}
#[doc = "`write(|w| ..)` method takes [`rng_dma_samples_num::W`](W) writer structure"]
impl crate::Writable for RngDmaSamplesNumSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RNG_DMA_SAMPLES_NUM to value 0"]
impl crate::Resettable for RngDmaSamplesNumSpec {}
