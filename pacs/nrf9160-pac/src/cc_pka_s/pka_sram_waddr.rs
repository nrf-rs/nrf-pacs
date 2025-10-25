#[doc = "Register `PKA_SRAM_WADDR` writer"]
pub type W = crate::W<PkaSramWaddrSpec>;
#[doc = "Field `ADDR` writer - PKA SRAM start address for write transaction"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - PKA SRAM start address for write transaction"]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<'_, PkaSramWaddrSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "Start address in PKA SRAM for subsequent write transactions.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pka_sram_waddr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PkaSramWaddrSpec;
impl crate::RegisterSpec for PkaSramWaddrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pka_sram_waddr::W`](W) writer structure"]
impl crate::Writable for PkaSramWaddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PKA_SRAM_WADDR to value 0"]
impl crate::Resettable for PkaSramWaddrSpec {}
