#[doc = "Register `OVERRIDE3` reader"]
pub type R = crate::R<Override3Spec>;
#[doc = "Register `OVERRIDE3` writer"]
pub type W = crate::W<Override3Spec>;
#[doc = "Field `OVERRIDE3` reader - Trim value override 3."]
pub type Override3R = crate::FieldReader<u32>;
#[doc = "Field `OVERRIDE3` writer - Trim value override 3."]
pub type Override3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Trim value override 3."]
    #[inline(always)]
    pub fn override3(&self) -> Override3R {
        Override3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Trim value override 3."]
    #[inline(always)]
    pub fn override3(&mut self) -> Override3W<'_, Override3Spec> {
        Override3W::new(self, 0)
    }
}
#[doc = "Trim value override register 3.\n\nYou can [`read`](crate::Reg::read) this register and get [`override3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`override3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Override3Spec;
impl crate::RegisterSpec for Override3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`override3::R`](R) reader structure"]
impl crate::Readable for Override3Spec {}
#[doc = "`write(|w| ..)` method takes [`override3::W`](W) writer structure"]
impl crate::Writable for Override3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OVERRIDE3 to value 0"]
impl crate::Resettable for Override3Spec {}
