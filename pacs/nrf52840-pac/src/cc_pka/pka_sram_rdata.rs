#[doc = "Register `PKA_SRAM_RDATA` reader"]
pub type R = crate::R<PkaSramRdataSpec>;
#[doc = "Field `DATA` reader - Data to read from PKA SRAM"]
pub type DataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Data to read from PKA SRAM"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
#[doc = "Read data from PKA SRAM. Reading from this register triggers a DMA transaction read data from PKA SRAM. The DMA address offset is automatically incremented during read.\n\nYou can [`read`](crate::Reg::read) this register and get [`pka_sram_rdata::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PkaSramRdataSpec;
impl crate::RegisterSpec for PkaSramRdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pka_sram_rdata::R`](R) reader structure"]
impl crate::Readable for PkaSramRdataSpec {}
#[doc = "`reset()` method sets PKA_SRAM_RDATA to value 0"]
impl crate::Resettable for PkaSramRdataSpec {}
