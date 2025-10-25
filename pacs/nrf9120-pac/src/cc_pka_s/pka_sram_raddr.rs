#[doc = "Register `PKA_SRAM_RADDR` writer"]
pub type W = crate::W<PkaSramRaddrSpec>;
#[doc = "Field `ADDR` writer - PKA SRAM start address for read transaction"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - PKA SRAM start address for read transaction"]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<'_, PkaSramRaddrSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "Start address in PKA SRAM for subsequent read transactions.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pka_sram_raddr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PkaSramRaddrSpec;
impl crate::RegisterSpec for PkaSramRaddrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pka_sram_raddr::W`](W) writer structure"]
impl crate::Writable for PkaSramRaddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PKA_SRAM_RADDR to value 0"]
impl crate::Resettable for PkaSramRaddrSpec {}
