#[doc = "Register `SAMPLE` reader"]
pub type R = crate::R<SampleSpec>;
#[doc = "Field `SAMPLE` reader - Last motion sample"]
pub type SampleR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Last motion sample"]
    #[inline(always)]
    pub fn sample(&self) -> SampleR {
        SampleR::new(self.bits)
    }
}
#[doc = "Motion sample value\n\nYou can [`read`](crate::Reg::read) this register and get [`sample::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SampleSpec;
impl crate::RegisterSpec for SampleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sample::R`](R) reader structure"]
impl crate::Readable for SampleSpec {}
#[doc = "`reset()` method sets SAMPLE to value 0"]
impl crate::Resettable for SampleSpec {}
