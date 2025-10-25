#[doc = "Register `DMISS` reader"]
pub type R = crate::R<DmissSpec>;
#[doc = "Field `MISSES` reader - Number of data cache misses"]
pub type MissesR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Number of data cache misses"]
    #[inline(always)]
    pub fn misses(&self) -> MissesR {
        MissesR::new(self.bits)
    }
}
#[doc = "Description cluster: Data fetch cache miss counter for cache region n, where n=0 means Flash and n=1 means XIP.\n\nYou can [`read`](crate::Reg::read) this register and get [`dmiss::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmissSpec;
impl crate::RegisterSpec for DmissSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmiss::R`](R) reader structure"]
impl crate::Readable for DmissSpec {}
#[doc = "`reset()` method sets DMISS to value 0"]
impl crate::Resettable for DmissSpec {}
