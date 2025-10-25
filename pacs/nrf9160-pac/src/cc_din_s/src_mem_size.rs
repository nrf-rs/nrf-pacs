#[doc = "Register `SRC_MEM_SIZE` writer"]
pub type W = crate::W<SrcMemSizeSpec>;
#[doc = "Field `SIZE` writer - Total number of bytes to read from memory."]
pub type SizeW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
#[doc = "Field `FIRST` writer - This field is reserved"]
pub type FirstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LAST` writer - This field is reserved"]
pub type LastW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bits 0:29 - Total number of bytes to read from memory."]
    #[inline(always)]
    pub fn size(&mut self) -> SizeW<'_, SrcMemSizeSpec> {
        SizeW::new(self, 0)
    }
    #[doc = "Bit 30 - This field is reserved"]
    #[inline(always)]
    pub fn first(&mut self) -> FirstW<'_, SrcMemSizeSpec> {
        FirstW::new(self, 30)
    }
    #[doc = "Bit 31 - This field is reserved"]
    #[inline(always)]
    pub fn last(&mut self) -> LastW<'_, SrcMemSizeSpec> {
        LastW::new(self, 31)
    }
}
#[doc = "The number of bytes to be read from memory. Writing to this register triggers the DMA operation.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`src_mem_size::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrcMemSizeSpec;
impl crate::RegisterSpec for SrcMemSizeSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`src_mem_size::W`](W) writer structure"]
impl crate::Writable for SrcMemSizeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SRC_MEM_SIZE to value 0"]
impl crate::Resettable for SrcMemSizeSpec {}
