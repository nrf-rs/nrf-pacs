#[doc = "Register `END` reader"]
pub type R = crate::R<EndSpec>;
#[doc = "Field `END` reader - Reserved for future use"]
pub type EndR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Reserved for future use"]
    #[inline(always)]
    pub fn end(&self) -> EndR {
        EndR::new(self.bits)
    }
}
#[doc = "Description cluster: Reserved for future use\n\nYou can [`read`](crate::Reg::read) this register and get [`end::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EndSpec;
impl crate::RegisterSpec for EndSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`end::R`](R) reader structure"]
impl crate::Readable for EndSpec {}
#[doc = "`reset()` method sets END to value 0"]
impl crate::Resettable for EndSpec {}
