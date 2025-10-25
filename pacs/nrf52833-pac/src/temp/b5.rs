#[doc = "Register `B5` reader"]
pub type R = crate::R<B5Spec>;
#[doc = "Register `B5` writer"]
pub type W = crate::W<B5Spec>;
#[doc = "Field `B5` reader - y-intercept of sixth piecewise linear function"]
pub type B5R = crate::FieldReader<u16>;
#[doc = "Field `B5` writer - y-intercept of sixth piecewise linear function"]
pub type B5W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - y-intercept of sixth piecewise linear function"]
    #[inline(always)]
    pub fn b5(&self) -> B5R {
        B5R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - y-intercept of sixth piecewise linear function"]
    #[inline(always)]
    pub fn b5(&mut self) -> B5W<'_, B5Spec> {
        B5W::new(self, 0)
    }
}
#[doc = "y-intercept of sixth piecewise linear function\n\nYou can [`read`](crate::Reg::read) this register and get [`b5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`b5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct B5Spec;
impl crate::RegisterSpec for B5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`b5::R`](R) reader structure"]
impl crate::Readable for B5Spec {}
#[doc = "`write(|w| ..)` method takes [`b5::W`](W) writer structure"]
impl crate::Writable for B5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets B5 to value 0x027c"]
impl crate::Resettable for B5Spec {
    const RESET_VALUE: u32 = 0x027c;
}
