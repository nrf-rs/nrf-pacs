#[doc = "Register `DST_MEM_ADDR` writer"]
pub type W = crate::W<DstMemAddrSpec>;
#[doc = "Field `ADDR` writer - Destination address in memory."]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Destination address in memory."]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<'_, DstMemAddrSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "Data destination address in memory.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dst_mem_addr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DstMemAddrSpec;
impl crate::RegisterSpec for DstMemAddrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dst_mem_addr::W`](W) writer structure"]
impl crate::Writable for DstMemAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DST_MEM_ADDR to value 0"]
impl crate::Resettable for DstMemAddrSpec {}
