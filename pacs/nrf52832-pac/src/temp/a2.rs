#[doc = "Register `A2` reader"]
pub type R = crate::R<A2Spec>;
#[doc = "Register `A2` writer"]
pub type W = crate::W<A2Spec>;
#[doc = "Field `A2` reader - Slope of 3rd piece wise linear function"]
pub type A2R = crate::FieldReader<u16>;
#[doc = "Field `A2` writer - Slope of 3rd piece wise linear function"]
pub type A2W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Slope of 3rd piece wise linear function"]
    #[inline(always)]
    pub fn a2(&self) -> A2R {
        A2R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Slope of 3rd piece wise linear function"]
    #[inline(always)]
    pub fn a2(&mut self) -> A2W<'_, A2Spec> {
        A2W::new(self, 0)
    }
}
#[doc = "Slope of 3rd piece wise linear function\n\nYou can [`read`](crate::Reg::read) this register and get [`a2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`a2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct A2Spec;
impl crate::RegisterSpec for A2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`a2::R`](R) reader structure"]
impl crate::Readable for A2Spec {}
#[doc = "`write(|w| ..)` method takes [`a2::W`](W) writer structure"]
impl crate::Writable for A2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets A2 to value 0x035d"]
impl crate::Resettable for A2Spec {
    const RESET_VALUE: u32 = 0x035d;
}
