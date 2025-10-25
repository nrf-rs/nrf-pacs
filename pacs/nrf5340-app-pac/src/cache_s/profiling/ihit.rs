#[doc = "Register `IHIT` reader"]
pub type R = crate::R<IhitSpec>;
#[doc = "Field `HITS` reader - Number of instruction cache hits"]
pub type HitsR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Number of instruction cache hits"]
    #[inline(always)]
    pub fn hits(&self) -> HitsR {
        HitsR::new(self.bits)
    }
}
#[doc = "Description cluster: Instruction fetch cache hit counter for cache region n, where n=0 means Flash and n=1 means XIP.\n\nYou can [`read`](crate::Reg::read) this register and get [`ihit::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IhitSpec;
impl crate::RegisterSpec for IhitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ihit::R`](R) reader structure"]
impl crate::Readable for IhitSpec {}
#[doc = "`reset()` method sets IHIT to value 0"]
impl crate::Resettable for IhitSpec {}
