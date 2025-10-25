#[doc = "Register `B4` reader"]
pub type R = crate::R<B4Spec>;
#[doc = "Register `B4` writer"]
pub type W = crate::W<B4Spec>;
#[doc = "Field `B4` reader - y-intercept of fifth piecewise linear function"]
pub type B4R = crate::FieldReader<u16>;
#[doc = "Field `B4` writer - y-intercept of fifth piecewise linear function"]
pub type B4W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - y-intercept of fifth piecewise linear function"]
    #[inline(always)]
    pub fn b4(&self) -> B4R {
        B4R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - y-intercept of fifth piecewise linear function"]
    #[inline(always)]
    pub fn b4(&mut self) -> B4W<'_, B4Spec> {
        B4W::new(self, 0)
    }
}
#[doc = "y-intercept of fifth piecewise linear function\n\nYou can [`read`](crate::Reg::read) this register and get [`b4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`b4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct B4Spec;
impl crate::RegisterSpec for B4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`b4::R`](R) reader structure"]
impl crate::Readable for B4Spec {}
#[doc = "`write(|w| ..)` method takes [`b4::W`](W) writer structure"]
impl crate::Writable for B4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets B4 to value 0x0124"]
impl crate::Resettable for B4Spec {
    const RESET_VALUE: u32 = 0x0124;
}
