#[doc = "Register `SRAM_DATA` reader"]
pub type R = crate::R<SramDataSpec>;
#[doc = "Register `SRAM_DATA` writer"]
pub type W = crate::W<SramDataSpec>;
#[doc = "Field `SRAM_DATA` reader - 32 bits DMA read/write from/to RNG SRAM. A 'read' or 'write' operation to this register will trigger the DMA address to be automatically incremented."]
pub type SramDataR = crate::FieldReader<u32>;
#[doc = "Field `SRAM_DATA` writer - 32 bits DMA read/write from/to RNG SRAM. A 'read' or 'write' operation to this register will trigger the DMA address to be automatically incremented."]
pub type SramDataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 32 bits DMA read/write from/to RNG SRAM. A 'read' or 'write' operation to this register will trigger the DMA address to be automatically incremented."]
    #[inline(always)]
    pub fn sram_data(&self) -> SramDataR {
        SramDataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 32 bits DMA read/write from/to RNG SRAM. A 'read' or 'write' operation to this register will trigger the DMA address to be automatically incremented."]
    #[inline(always)]
    pub fn sram_data(&mut self) -> SramDataW<'_, SramDataSpec> {
        SramDataW::new(self, 0)
    }
}
#[doc = "Read/Write data from RNG SRAM\n\nYou can [`read`](crate::Reg::read) this register and get [`sram_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SramDataSpec;
impl crate::RegisterSpec for SramDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sram_data::R`](R) reader structure"]
impl crate::Readable for SramDataSpec {}
#[doc = "`write(|w| ..)` method takes [`sram_data::W`](W) writer structure"]
impl crate::Writable for SramDataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SRAM_DATA to value 0"]
impl crate::Resettable for SramDataSpec {}
