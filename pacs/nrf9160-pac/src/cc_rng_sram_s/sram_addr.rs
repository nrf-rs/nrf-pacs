#[doc = "Register `SRAM_ADDR` writer"]
pub type W = crate::W<SramAddrSpec>;
#[doc = "Field `SRAM_ADDR` writer - RNG SRAM starting address"]
pub type SramAddrW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl W {
    #[doc = "Bits 0:14 - RNG SRAM starting address"]
    #[inline(always)]
    pub fn sram_addr(&mut self) -> SramAddrW<'_, SramAddrSpec> {
        SramAddrW::new(self, 0)
    }
}
#[doc = "First address given to RNG SRAM DMA for read/write transactions from/to RNG SRAM.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram_addr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SramAddrSpec;
impl crate::RegisterSpec for SramAddrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sram_addr::W`](W) writer structure"]
impl crate::Writable for SramAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SRAM_ADDR to value 0"]
impl crate::Resettable for SramAddrSpec {}
