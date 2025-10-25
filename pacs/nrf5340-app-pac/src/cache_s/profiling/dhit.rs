#[doc = "Register `DHIT` reader"]
pub type R = crate::R<DhitSpec>;
#[doc = "Field `HITS` reader - Number of data cache hits"]
pub type HitsR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Number of data cache hits"]
    #[inline(always)]
    pub fn hits(&self) -> HitsR {
        HitsR::new(self.bits)
    }
}
#[doc = "Description cluster: Data fetch cache hit counter for cache region n, where n=0 means Flash and n=1 means XIP.\n\nYou can [`read`](crate::Reg::read) this register and get [`dhit::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DhitSpec;
impl crate::RegisterSpec for DhitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dhit::R`](R) reader structure"]
impl crate::Readable for DhitSpec {}
#[doc = "`reset()` method sets DHIT to value 0"]
impl crate::Resettable for DhitSpec {}
