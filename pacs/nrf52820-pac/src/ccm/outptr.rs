#[doc = "Register `OUTPTR` reader"]
pub type R = crate::R<OutptrSpec>;
#[doc = "Register `OUTPTR` writer"]
pub type W = crate::W<OutptrSpec>;
#[doc = "Field `OUTPTR` reader - Output pointer"]
pub type OutptrR = crate::FieldReader<u32>;
#[doc = "Field `OUTPTR` writer - Output pointer"]
pub type OutptrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Output pointer"]
    #[inline(always)]
    pub fn outptr(&self) -> OutptrR {
        OutptrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Output pointer"]
    #[inline(always)]
    pub fn outptr(&mut self) -> OutptrW<'_, OutptrSpec> {
        OutptrW::new(self, 0)
    }
}
#[doc = "Output pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`outptr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outptr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutptrSpec;
impl crate::RegisterSpec for OutptrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`outptr::R`](R) reader structure"]
impl crate::Readable for OutptrSpec {}
#[doc = "`write(|w| ..)` method takes [`outptr::W`](W) writer structure"]
impl crate::Writable for OutptrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OUTPTR to value 0"]
impl crate::Resettable for OutptrSpec {}
