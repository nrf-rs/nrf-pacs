#[doc = "Register `OVERRIDE0` reader"]
pub type R = crate::R<Override0Spec>;
#[doc = "Register `OVERRIDE0` writer"]
pub type W = crate::W<Override0Spec>;
#[doc = "Field `OVERRIDE0` reader - Trim value override 0."]
pub type Override0R = crate::FieldReader<u32>;
#[doc = "Field `OVERRIDE0` writer - Trim value override 0."]
pub type Override0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Trim value override 0."]
    #[inline(always)]
    pub fn override0(&self) -> Override0R {
        Override0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Trim value override 0."]
    #[inline(always)]
    pub fn override0(&mut self) -> Override0W<'_, Override0Spec> {
        Override0W::new(self, 0)
    }
}
#[doc = "Trim value override register 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`override0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`override0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Override0Spec;
impl crate::RegisterSpec for Override0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`override0::R`](R) reader structure"]
impl crate::Readable for Override0Spec {}
#[doc = "`write(|w| ..)` method takes [`override0::W`](W) writer structure"]
impl crate::Writable for Override0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OVERRIDE0 to value 0"]
impl crate::Resettable for Override0Spec {}
