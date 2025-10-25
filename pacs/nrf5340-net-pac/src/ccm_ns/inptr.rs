#[doc = "Register `INPTR` reader"]
pub type R = crate::R<InptrSpec>;
#[doc = "Register `INPTR` writer"]
pub type W = crate::W<InptrSpec>;
#[doc = "Field `INPTR` reader - Input pointer"]
pub type InptrR = crate::FieldReader<u32>;
#[doc = "Field `INPTR` writer - Input pointer"]
pub type InptrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Input pointer"]
    #[inline(always)]
    pub fn inptr(&self) -> InptrR {
        InptrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Input pointer"]
    #[inline(always)]
    pub fn inptr(&mut self) -> InptrW<'_, InptrSpec> {
        InptrW::new(self, 0)
    }
}
#[doc = "Input pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`inptr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inptr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InptrSpec;
impl crate::RegisterSpec for InptrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inptr::R`](R) reader structure"]
impl crate::Readable for InptrSpec {}
#[doc = "`write(|w| ..)` method takes [`inptr::W`](W) writer structure"]
impl crate::Writable for InptrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INPTR to value 0"]
impl crate::Resettable for InptrSpec {}
