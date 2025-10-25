#[doc = "Register `A4` reader"]
pub type R = crate::R<A4Spec>;
#[doc = "Register `A4` writer"]
pub type W = crate::W<A4Spec>;
#[doc = "Field `A4` reader - Slope of 5th piece wise linear function"]
pub type A4R = crate::FieldReader<u16>;
#[doc = "Field `A4` writer - Slope of 5th piece wise linear function"]
pub type A4W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Slope of 5th piece wise linear function"]
    #[inline(always)]
    pub fn a4(&self) -> A4R {
        A4R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Slope of 5th piece wise linear function"]
    #[inline(always)]
    pub fn a4(&mut self) -> A4W<'_, A4Spec> {
        A4W::new(self, 0)
    }
}
#[doc = "Slope of 5th piece wise linear function\n\nYou can [`read`](crate::Reg::read) this register and get [`a4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`a4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct A4Spec;
impl crate::RegisterSpec for A4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`a4::R`](R) reader structure"]
impl crate::Readable for A4Spec {}
#[doc = "`write(|w| ..)` method takes [`a4::W`](W) writer structure"]
impl crate::Writable for A4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets A4 to value 0x04bd"]
impl crate::Resettable for A4Spec {
    const RESET_VALUE: u32 = 0x04bd;
}
