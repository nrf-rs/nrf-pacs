#[doc = "Register `A0` reader"]
pub type R = crate::R<A0Spec>;
#[doc = "Register `A0` writer"]
pub type W = crate::W<A0Spec>;
#[doc = "Field `A0` reader - Slope of first piecewise linear function"]
pub type A0R = crate::FieldReader<u16>;
#[doc = "Field `A0` writer - Slope of first piecewise linear function"]
pub type A0W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Slope of first piecewise linear function"]
    #[inline(always)]
    pub fn a0(&self) -> A0R {
        A0R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Slope of first piecewise linear function"]
    #[inline(always)]
    pub fn a0(&mut self) -> A0W<'_, A0Spec> {
        A0W::new(self, 0)
    }
}
#[doc = "Slope of first piecewise linear function\n\nYou can [`read`](crate::Reg::read) this register and get [`a0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`a0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct A0Spec;
impl crate::RegisterSpec for A0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`a0::R`](R) reader structure"]
impl crate::Readable for A0Spec {}
#[doc = "`write(|w| ..)` method takes [`a0::W`](W) writer structure"]
impl crate::Writable for A0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets A0 to value 0x02d9"]
impl crate::Resettable for A0Spec {
    const RESET_VALUE: u32 = 0x02d9;
}
