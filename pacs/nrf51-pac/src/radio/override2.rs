#[doc = "Register `OVERRIDE2` reader"]
pub type R = crate::R<Override2Spec>;
#[doc = "Register `OVERRIDE2` writer"]
pub type W = crate::W<Override2Spec>;
#[doc = "Field `OVERRIDE2` reader - Trim value override 2."]
pub type Override2R = crate::FieldReader<u32>;
#[doc = "Field `OVERRIDE2` writer - Trim value override 2."]
pub type Override2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Trim value override 2."]
    #[inline(always)]
    pub fn override2(&self) -> Override2R {
        Override2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Trim value override 2."]
    #[inline(always)]
    pub fn override2(&mut self) -> Override2W<'_, Override2Spec> {
        Override2W::new(self, 0)
    }
}
#[doc = "Trim value override register 2.\n\nYou can [`read`](crate::Reg::read) this register and get [`override2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`override2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Override2Spec;
impl crate::RegisterSpec for Override2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`override2::R`](R) reader structure"]
impl crate::Readable for Override2Spec {}
#[doc = "`write(|w| ..)` method takes [`override2::W`](W) writer structure"]
impl crate::Writable for Override2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OVERRIDE2 to value 0"]
impl crate::Resettable for Override2Spec {}
