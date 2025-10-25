#[doc = "Register `IRKPTR` reader"]
pub type R = crate::R<IrkptrSpec>;
#[doc = "Register `IRKPTR` writer"]
pub type W = crate::W<IrkptrSpec>;
#[doc = "Field `IRKPTR` reader - Pointer to the IRK data structure"]
pub type IrkptrR = crate::FieldReader<u32>;
#[doc = "Field `IRKPTR` writer - Pointer to the IRK data structure"]
pub type IrkptrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Pointer to the IRK data structure"]
    #[inline(always)]
    pub fn irkptr(&self) -> IrkptrR {
        IrkptrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Pointer to the IRK data structure"]
    #[inline(always)]
    pub fn irkptr(&mut self) -> IrkptrW<'_, IrkptrSpec> {
        IrkptrW::new(self, 0)
    }
}
#[doc = "Pointer to IRK data structure\n\nYou can [`read`](crate::Reg::read) this register and get [`irkptr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irkptr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrkptrSpec;
impl crate::RegisterSpec for IrkptrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irkptr::R`](R) reader structure"]
impl crate::Readable for IrkptrSpec {}
#[doc = "`write(|w| ..)` method takes [`irkptr::W`](W) writer structure"]
impl crate::Writable for IrkptrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IRKPTR to value 0"]
impl crate::Resettable for IrkptrSpec {}
