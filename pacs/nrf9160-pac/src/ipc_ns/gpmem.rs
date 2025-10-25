#[doc = "Register `GPMEM[%s]` reader"]
pub type R = crate::R<GpmemSpec>;
#[doc = "Register `GPMEM[%s]` writer"]
pub type W = crate::W<GpmemSpec>;
#[doc = "Field `GPMEM` reader - General purpose memory"]
pub type GpmemR = crate::FieldReader<u32>;
#[doc = "Field `GPMEM` writer - General purpose memory"]
pub type GpmemW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - General purpose memory"]
    #[inline(always)]
    pub fn gpmem(&self) -> GpmemR {
        GpmemR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - General purpose memory"]
    #[inline(always)]
    pub fn gpmem(&mut self) -> GpmemW<'_, GpmemSpec> {
        GpmemW::new(self, 0)
    }
}
#[doc = "Description collection: General purpose memory\n\nYou can [`read`](crate::Reg::read) this register and get [`gpmem::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpmem::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpmemSpec;
impl crate::RegisterSpec for GpmemSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpmem::R`](R) reader structure"]
impl crate::Readable for GpmemSpec {}
#[doc = "`write(|w| ..)` method takes [`gpmem::W`](W) writer structure"]
impl crate::Writable for GpmemSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPMEM[%s] to value 0"]
impl crate::Resettable for GpmemSpec {}
