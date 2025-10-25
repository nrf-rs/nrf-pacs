#[doc = "Register `DST_SRAM_SIZE` writer"]
pub type W = crate::W<DstSramSizeSpec>;
#[doc = "Field `SIZE` writer - Total number of bytes to write to RNG SRAM."]
pub type SizeW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Total number of bytes to write to RNG SRAM."]
    #[inline(always)]
    pub fn size(&mut self) -> SizeW<'_, DstSramSizeSpec> {
        SizeW::new(self, 0)
    }
}
#[doc = "The number of bytes to be written to RNG SRAM.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dst_sram_size::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DstSramSizeSpec;
impl crate::RegisterSpec for DstSramSizeSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dst_sram_size::W`](W) writer structure"]
impl crate::Writable for DstSramSizeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DST_SRAM_SIZE to value 0"]
impl crate::Resettable for DstSramSizeSpec {}
