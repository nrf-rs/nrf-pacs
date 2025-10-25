#[doc = "Register `SRC_SRAM_ADDR` reader"]
pub type R = crate::R<SrcSramAddrSpec>;
#[doc = "Register `SRC_SRAM_ADDR` writer"]
pub type W = crate::W<SrcSramAddrSpec>;
#[doc = "Field `ADDR` reader - Source address in RNG SRAM."]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - Source address in RNG SRAM."]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Source address in RNG SRAM."]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Source address in RNG SRAM."]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<'_, SrcSramAddrSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "Data source address in RNG SRAM.\n\nYou can [`read`](crate::Reg::read) this register and get [`src_sram_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`src_sram_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrcSramAddrSpec;
impl crate::RegisterSpec for SrcSramAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`src_sram_addr::R`](R) reader structure"]
impl crate::Readable for SrcSramAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`src_sram_addr::W`](W) writer structure"]
impl crate::Writable for SrcSramAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SRC_SRAM_ADDR to value 0"]
impl crate::Resettable for SrcSramAddrSpec {}
