#[doc = "Register `END` reader"]
pub type R = crate::R<EndSpec>;
#[doc = "Register `END` writer"]
pub type W = crate::W<EndSpec>;
#[doc = "Field `END` reader - End address of region."]
pub type EndR = crate::FieldReader<u32>;
#[doc = "Field `END` writer - End address of region."]
pub type EndW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - End address of region."]
    #[inline(always)]
    pub fn end(&self) -> EndR {
        EndR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - End address of region."]
    #[inline(always)]
    pub fn end(&mut self) -> EndW<'_, EndSpec> {
        EndW::new(self, 0)
    }
}
#[doc = "Description cluster\\[0\\]: End address of region 0\n\nYou can [`read`](crate::Reg::read) this register and get [`end::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`end::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EndSpec;
impl crate::RegisterSpec for EndSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`end::R`](R) reader structure"]
impl crate::Readable for EndSpec {}
#[doc = "`write(|w| ..)` method takes [`end::W`](W) writer structure"]
impl crate::Writable for EndSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets END to value 0"]
impl crate::Resettable for EndSpec {}
