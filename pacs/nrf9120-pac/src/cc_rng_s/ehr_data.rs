#[doc = "Register `EHR_DATA[%s]` reader"]
pub type R = crate::R<EhrDataSpec>;
#[doc = "Field `VALUE` reader - Random data value."]
pub type ValueR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Random data value."]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new(self.bits)
    }
}
#[doc = "Description collection: The entropy holding registers (EHR) hold 192-bits random data collected by the TRNG. The initial EHR_DATA\\[0\\] register holds the least significant bits \\[31:0\\] of the random data value.\n\nYou can [`read`](crate::Reg::read) this register and get [`ehr_data::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EhrDataSpec;
impl crate::RegisterSpec for EhrDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ehr_data::R`](R) reader structure"]
impl crate::Readable for EhrDataSpec {}
#[doc = "`reset()` method sets EHR_DATA[%s] to value 0"]
impl crate::Resettable for EhrDataSpec {}
