#[doc = "Register `ICR` writer"]
pub type W = crate::W<IcrSpec>;
#[doc = "Field `SRAM_TO_DIN_CLEAR` writer - The RNG SRAM to DIN DMA done interrupt clear."]
pub type SramToDinClearW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOUT_TO_SRAM_CLEAR` writer - The DOUT to RNG SRAM DMA done interrupt clear."]
pub type DoutToSramClearW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_TO_DIN_CLEAR` writer - The memory to DIN DMA done interrupt clear."]
pub type MemToDinClearW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOUT_TO_MEM_CLEAR` writer - The DOUT to memory DMA done interrupt clear."]
pub type DoutToMemClearW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHB_ERR_CLEAR` writer - The AHB error interrupt clear."]
pub type AhbErrClearW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PKA_CLEAR` writer - The PKA end of operation interrupt clear."]
pub type PkaClearW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNG_CLEAR` writer - The RNG interrupt clear. Register RNG_ISR in the RNG engine must be cleared before this interrupt can be cleared."]
pub type RngClearW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 4 - The RNG SRAM to DIN DMA done interrupt clear."]
    #[inline(always)]
    pub fn sram_to_din_clear(&mut self) -> SramToDinClearW<'_, IcrSpec> {
        SramToDinClearW::new(self, 4)
    }
    #[doc = "Bit 5 - The DOUT to RNG SRAM DMA done interrupt clear."]
    #[inline(always)]
    pub fn dout_to_sram_clear(&mut self) -> DoutToSramClearW<'_, IcrSpec> {
        DoutToSramClearW::new(self, 5)
    }
    #[doc = "Bit 6 - The memory to DIN DMA done interrupt clear."]
    #[inline(always)]
    pub fn mem_to_din_clear(&mut self) -> MemToDinClearW<'_, IcrSpec> {
        MemToDinClearW::new(self, 6)
    }
    #[doc = "Bit 7 - The DOUT to memory DMA done interrupt clear."]
    #[inline(always)]
    pub fn dout_to_mem_clear(&mut self) -> DoutToMemClearW<'_, IcrSpec> {
        DoutToMemClearW::new(self, 7)
    }
    #[doc = "Bit 8 - The AHB error interrupt clear."]
    #[inline(always)]
    pub fn ahb_err_clear(&mut self) -> AhbErrClearW<'_, IcrSpec> {
        AhbErrClearW::new(self, 8)
    }
    #[doc = "Bit 9 - The PKA end of operation interrupt clear."]
    #[inline(always)]
    pub fn pka_clear(&mut self) -> PkaClearW<'_, IcrSpec> {
        PkaClearW::new(self, 9)
    }
    #[doc = "Bit 10 - The RNG interrupt clear. Register RNG_ISR in the RNG engine must be cleared before this interrupt can be cleared."]
    #[inline(always)]
    pub fn rng_clear(&mut self) -> RngClearW<'_, IcrSpec> {
        RngClearW::new(self, 10)
    }
}
#[doc = "Interrupt clear register. Writing a 1 bit into a field in this register will clear the corresponding bit in IRR.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcrSpec;
impl crate::RegisterSpec for IcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for IcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for IcrSpec {}
