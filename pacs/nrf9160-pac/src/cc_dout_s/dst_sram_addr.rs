#[doc = "Register `DST_SRAM_ADDR` reader"]
pub type R = crate::R<DstSramAddrSpec>;
#[doc = "Register `DST_SRAM_ADDR` writer"]
pub type W = crate::W<DstSramAddrSpec>;
#[doc = "Field `ADDR` reader - Destination address in RNG SRAM."]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - Destination address in RNG SRAM."]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Destination address in RNG SRAM."]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Destination address in RNG SRAM."]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<'_, DstSramAddrSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "Data destination address in RNG SRAM.\n\nYou can [`read`](crate::Reg::read) this register and get [`dst_sram_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dst_sram_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DstSramAddrSpec;
impl crate::RegisterSpec for DstSramAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dst_sram_addr::R`](R) reader structure"]
impl crate::Readable for DstSramAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`dst_sram_addr::W`](W) writer structure"]
impl crate::Writable for DstSramAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DST_SRAM_ADDR to value 0"]
impl crate::Resettable for DstSramAddrSpec {}
