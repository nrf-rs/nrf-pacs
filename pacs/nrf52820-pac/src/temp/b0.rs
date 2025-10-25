#[doc = "Register `B0` reader"]
pub type R = crate::R<B0Spec>;
#[doc = "Register `B0` writer"]
pub type W = crate::W<B0Spec>;
#[doc = "Field `B0` reader - y-intercept of first piecewise linear function"]
pub type B0R = crate::FieldReader<u16>;
#[doc = "Field `B0` writer - y-intercept of first piecewise linear function"]
pub type B0W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - y-intercept of first piecewise linear function"]
    #[inline(always)]
    pub fn b0(&self) -> B0R {
        B0R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - y-intercept of first piecewise linear function"]
    #[inline(always)]
    pub fn b0(&mut self) -> B0W<'_, B0Spec> {
        B0W::new(self, 0)
    }
}
#[doc = "y-intercept of first piecewise linear function\n\nYou can [`read`](crate::Reg::read) this register and get [`b0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`b0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct B0Spec;
impl crate::RegisterSpec for B0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`b0::R`](R) reader structure"]
impl crate::Readable for B0Spec {}
#[doc = "`write(|w| ..)` method takes [`b0::W`](W) writer structure"]
impl crate::Writable for B0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets B0 to value 0x3fef"]
impl crate::Resettable for B0Spec {
    const RESET_VALUE: u32 = 0x3fef;
}
