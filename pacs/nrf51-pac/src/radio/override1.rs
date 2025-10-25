#[doc = "Register `OVERRIDE1` reader"]
pub type R = crate::R<Override1Spec>;
#[doc = "Register `OVERRIDE1` writer"]
pub type W = crate::W<Override1Spec>;
#[doc = "Field `OVERRIDE1` reader - Trim value override 1."]
pub type Override1R = crate::FieldReader<u32>;
#[doc = "Field `OVERRIDE1` writer - Trim value override 1."]
pub type Override1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Trim value override 1."]
    #[inline(always)]
    pub fn override1(&self) -> Override1R {
        Override1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Trim value override 1."]
    #[inline(always)]
    pub fn override1(&mut self) -> Override1W<'_, Override1Spec> {
        Override1W::new(self, 0)
    }
}
#[doc = "Trim value override register 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`override1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`override1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Override1Spec;
impl crate::RegisterSpec for Override1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`override1::R`](R) reader structure"]
impl crate::Readable for Override1Spec {}
#[doc = "`write(|w| ..)` method takes [`override1::W`](W) writer structure"]
impl crate::Writable for Override1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OVERRIDE1 to value 0"]
impl crate::Resettable for Override1Spec {}
