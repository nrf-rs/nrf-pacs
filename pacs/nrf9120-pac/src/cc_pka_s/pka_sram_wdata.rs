#[doc = "Register `PKA_SRAM_WDATA` writer"]
pub type W = crate::W<PkaSramWdataSpec>;
#[doc = "Field `DATA` writer - Data to write to PKA SRAM."]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Data to write to PKA SRAM."]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, PkaSramWdataSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "Write data to PKA SRAM. Writing to this register triggers a DMA transaction writing data into PKA SRAM. The DMA address offset is automatically incremented during write.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pka_sram_wdata::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PkaSramWdataSpec;
impl crate::RegisterSpec for PkaSramWdataSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pka_sram_wdata::W`](W) writer structure"]
impl crate::Writable for PkaSramWdataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PKA_SRAM_WDATA to value 0"]
impl crate::Resettable for PkaSramWdataSpec {}
