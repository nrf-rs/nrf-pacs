#[doc = "Register `IMISS` reader"]
pub type R = crate::R<ImissSpec>;
#[doc = "Field `MISSES` reader - Number of instruction cache misses"]
pub type MissesR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Number of instruction cache misses"]
    #[inline(always)]
    pub fn misses(&self) -> MissesR {
        MissesR::new(self.bits)
    }
}
#[doc = "Description cluster: Instruction fetch cache miss counter for cache region n, where n=0 means Flash and n=1 means XIP.\n\nYou can [`read`](crate::Reg::read) this register and get [`imiss::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImissSpec;
impl crate::RegisterSpec for ImissSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imiss::R`](R) reader structure"]
impl crate::Readable for ImissSpec {}
#[doc = "`reset()` method sets IMISS to value 0"]
impl crate::Resettable for ImissSpec {}
