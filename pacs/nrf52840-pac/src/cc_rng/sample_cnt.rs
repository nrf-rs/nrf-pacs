#[doc = "Register `SAMPLE_CNT` reader"]
pub type R = crate::R<SampleCntSpec>;
#[doc = "Register `SAMPLE_CNT` writer"]
pub type W = crate::W<SampleCntSpec>;
#[doc = "Field `VALUE` reader - Number of CPU clock cycles between two consecutive noise source samples."]
pub type ValueR = crate::FieldReader<u32>;
#[doc = "Field `VALUE` writer - Number of CPU clock cycles between two consecutive noise source samples."]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Number of CPU clock cycles between two consecutive noise source samples."]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Number of CPU clock cycles between two consecutive noise source samples."]
    #[inline(always)]
    pub fn value(&mut self) -> ValueW<'_, SampleCntSpec> {
        ValueW::new(self, 0)
    }
}
#[doc = "Sample count defining the number of CPU clock cycles between two consecutive noise source samples.\n\nYou can [`read`](crate::Reg::read) this register and get [`sample_cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sample_cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SampleCntSpec;
impl crate::RegisterSpec for SampleCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sample_cnt::R`](R) reader structure"]
impl crate::Readable for SampleCntSpec {}
#[doc = "`write(|w| ..)` method takes [`sample_cnt::W`](W) writer structure"]
impl crate::Writable for SampleCntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAMPLE_CNT to value 0xffff"]
impl crate::Resettable for SampleCntSpec {
    const RESET_VALUE: u32 = 0xffff;
}
