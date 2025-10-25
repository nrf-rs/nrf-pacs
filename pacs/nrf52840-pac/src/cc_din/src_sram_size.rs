#[doc = "Register `SRC_SRAM_SIZE` writer"]
pub type W = crate::W<SrcSramSizeSpec>;
#[doc = "Field `SIZE` writer - Total number of bytes to read from RNG SRAM."]
pub type SizeW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Total number of bytes to read from RNG SRAM."]
    #[inline(always)]
    pub fn size(&mut self) -> SizeW<'_, SrcSramSizeSpec> {
        SizeW::new(self, 0)
    }
}
#[doc = "The number of bytes to be read from RNG SRAM. Writing to this register triggers the DMA operation.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`src_sram_size::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrcSramSizeSpec;
impl crate::RegisterSpec for SrcSramSizeSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`src_sram_size::W`](W) writer structure"]
impl crate::Writable for SrcSramSizeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SRC_SRAM_SIZE to value 0"]
impl crate::Resettable for SrcSramSizeSpec {}
