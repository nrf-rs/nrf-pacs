#[doc = "Register `IRR` reader"]
pub type R = crate::R<IrrSpec>;
#[doc = "Field `SRAM_TO_DIN_INT` reader - The RNG SRAM to DIN DMA done interrupt status. This interrupt is asserted when all data was delivered from RNG SRAM to DIN buffer."]
pub type SramToDinIntR = crate::BitReader;
#[doc = "Field `DOUT_TO_SRAM_INT` reader - The DOUT to RNG SRAM DMA done interrupt status. This interrupt is asserted when all data was delivered from DOUT buffer to RNG SRAM."]
pub type DoutToSramIntR = crate::BitReader;
#[doc = "Field `MEM_TO_DIN_INT` reader - The memory to DIN DMA done interrupt status. This interrupt is asserted when all data was delivered from memory to DIN buffer."]
pub type MemToDinIntR = crate::BitReader;
#[doc = "Field `DOUT_TO_MEM_INT` reader - The DOUT to memory DMA done interrupt status. This interrupt is asserted when all data was delivered from DOUT buffer to memory."]
pub type DoutToMemIntR = crate::BitReader;
#[doc = "Field `AHB_ERR_INT` reader - The AHB error interrupt status."]
pub type AhbErrIntR = crate::BitReader;
#[doc = "Field `PKA_INT` reader - The PKA end of operation interrupt status."]
pub type PkaIntR = crate::BitReader;
#[doc = "Field `RNG_INT` reader - The RNG interrupt status."]
pub type RngIntR = crate::BitReader;
impl R {
    #[doc = "Bit 4 - The RNG SRAM to DIN DMA done interrupt status. This interrupt is asserted when all data was delivered from RNG SRAM to DIN buffer."]
    #[inline(always)]
    pub fn sram_to_din_int(&self) -> SramToDinIntR {
        SramToDinIntR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The DOUT to RNG SRAM DMA done interrupt status. This interrupt is asserted when all data was delivered from DOUT buffer to RNG SRAM."]
    #[inline(always)]
    pub fn dout_to_sram_int(&self) -> DoutToSramIntR {
        DoutToSramIntR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The memory to DIN DMA done interrupt status. This interrupt is asserted when all data was delivered from memory to DIN buffer."]
    #[inline(always)]
    pub fn mem_to_din_int(&self) -> MemToDinIntR {
        MemToDinIntR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The DOUT to memory DMA done interrupt status. This interrupt is asserted when all data was delivered from DOUT buffer to memory."]
    #[inline(always)]
    pub fn dout_to_mem_int(&self) -> DoutToMemIntR {
        DoutToMemIntR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The AHB error interrupt status."]
    #[inline(always)]
    pub fn ahb_err_int(&self) -> AhbErrIntR {
        AhbErrIntR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The PKA end of operation interrupt status."]
    #[inline(always)]
    pub fn pka_int(&self) -> PkaIntR {
        PkaIntR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The RNG interrupt status."]
    #[inline(always)]
    pub fn rng_int(&self) -> RngIntR {
        RngIntR::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "Interrupt request register. Each bit of this register holds the interrupt status of a single interrupt source. If corresponding IMR bit is unmasked, an interrupt is generated.\n\nYou can [`read`](crate::Reg::read) this register and get [`irr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrrSpec;
impl crate::RegisterSpec for IrrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irr::R`](R) reader structure"]
impl crate::Readable for IrrSpec {}
#[doc = "`reset()` method sets IRR to value 0"]
impl crate::Resettable for IrrSpec {}
