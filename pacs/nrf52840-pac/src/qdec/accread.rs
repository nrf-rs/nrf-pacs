#[doc = "Register `ACCREAD` reader"]
pub type R = crate::R<AccreadSpec>;
#[doc = "Field `ACCREAD` reader - Snapshot of the ACC register."]
pub type AccreadR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Snapshot of the ACC register."]
    #[inline(always)]
    pub fn accread(&self) -> AccreadR {
        AccreadR::new(self.bits)
    }
}
#[doc = "Snapshot of the ACC register, updated by the READCLRACC or RDCLRACC task\n\nYou can [`read`](crate::Reg::read) this register and get [`accread::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AccreadSpec;
impl crate::RegisterSpec for AccreadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`accread::R`](R) reader structure"]
impl crate::Readable for AccreadSpec {}
#[doc = "`reset()` method sets ACCREAD to value 0"]
impl crate::Resettable for AccreadSpec {}
