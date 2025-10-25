#[doc = "Register `PTR` reader"]
pub type R = crate::R<PtrSpec>;
#[doc = "Register `PTR` writer"]
pub type W = crate::W<PtrSpec>;
#[doc = "Field `SAMPLEPTR` reader - Address to write PDM samples to over DMA"]
pub type SampleptrR = crate::FieldReader<u32>;
#[doc = "Field `SAMPLEPTR` writer - Address to write PDM samples to over DMA"]
pub type SampleptrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Address to write PDM samples to over DMA"]
    #[inline(always)]
    pub fn sampleptr(&self) -> SampleptrR {
        SampleptrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Address to write PDM samples to over DMA"]
    #[inline(always)]
    pub fn sampleptr(&mut self) -> SampleptrW<'_, PtrSpec> {
        SampleptrW::new(self, 0)
    }
}
#[doc = "RAM address pointer to write samples to with EasyDMA\n\nYou can [`read`](crate::Reg::read) this register and get [`ptr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PtrSpec;
impl crate::RegisterSpec for PtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptr::R`](R) reader structure"]
impl crate::Readable for PtrSpec {}
#[doc = "`write(|w| ..)` method takes [`ptr::W`](W) writer structure"]
impl crate::Writable for PtrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PTR to value 0"]
impl crate::Resettable for PtrSpec {}
