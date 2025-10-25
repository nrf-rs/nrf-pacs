#[doc = "Register `SRC_MEM_ADDR` writer"]
pub type W = crate::W<SrcMemAddrSpec>;
#[doc = "Field `ADDR` writer - Source address in memory."]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Source address in memory."]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<'_, SrcMemAddrSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "Data source address in memory.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`src_mem_addr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrcMemAddrSpec;
impl crate::RegisterSpec for SrcMemAddrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`src_mem_addr::W`](W) writer structure"]
impl crate::Writable for SrcMemAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SRC_MEM_ADDR to value 0"]
impl crate::Resettable for SrcMemAddrSpec {}
