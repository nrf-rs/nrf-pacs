#[doc = "Register `PKA_SRAM_WCLEAR` writer"]
pub type W = crate::W<PkaSramWclearSpec>;
#[doc = "Field `CLEAR` writer - Clear the PKA SRAM write buffer."]
pub type ClearW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Clear the PKA SRAM write buffer."]
    #[inline(always)]
    pub fn clear(&mut self) -> ClearW<'_, PkaSramWclearSpec> {
        ClearW::new(self, 0)
    }
}
#[doc = "Register for clearing PKA SRAM write buffer.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pka_sram_wclear::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PkaSramWclearSpec;
impl crate::RegisterSpec for PkaSramWclearSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pka_sram_wclear::W`](W) writer structure"]
impl crate::Writable for PkaSramWclearSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PKA_SRAM_WCLEAR to value 0"]
impl crate::Resettable for PkaSramWclearSpec {}
