#[doc = "Register `A3` reader"]
pub type R = crate::R<A3Spec>;
#[doc = "Register `A3` writer"]
pub type W = crate::W<A3Spec>;
#[doc = "Field `A3` reader - Slope of 4th piece wise linear function"]
pub type A3R = crate::FieldReader<u16>;
#[doc = "Field `A3` writer - Slope of 4th piece wise linear function"]
pub type A3W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Slope of 4th piece wise linear function"]
    #[inline(always)]
    pub fn a3(&self) -> A3R {
        A3R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Slope of 4th piece wise linear function"]
    #[inline(always)]
    pub fn a3(&mut self) -> A3W<'_, A3Spec> {
        A3W::new(self, 0)
    }
}
#[doc = "Slope of 4th piece wise linear function\n\nYou can [`read`](crate::Reg::read) this register and get [`a3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`a3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct A3Spec;
impl crate::RegisterSpec for A3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`a3::R`](R) reader structure"]
impl crate::Readable for A3Spec {}
#[doc = "`write(|w| ..)` method takes [`a3::W`](W) writer structure"]
impl crate::Writable for A3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets A3 to value 0x0400"]
impl crate::Resettable for A3Spec {
    const RESET_VALUE: u32 = 0x0400;
}
