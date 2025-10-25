#[doc = "Register `START` reader"]
pub type R = crate::R<StartSpec>;
#[doc = "Field `START` reader - Reserved for future use"]
pub type StartR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Reserved for future use"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(self.bits)
    }
}
#[doc = "Description cluster\\[0\\]: Reserved for future use\n\nYou can [`read`](crate::Reg::read) this register and get [`start::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StartSpec;
impl crate::RegisterSpec for StartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`start::R`](R) reader structure"]
impl crate::Readable for StartSpec {}
#[doc = "`reset()` method sets START to value 0"]
impl crate::Resettable for StartSpec {}
