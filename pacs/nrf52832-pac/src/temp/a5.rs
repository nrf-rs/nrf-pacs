#[doc = "Register `A5` reader"]
pub type R = crate::R<A5Spec>;
#[doc = "Register `A5` writer"]
pub type W = crate::W<A5Spec>;
#[doc = "Field `A5` reader - Slope of 6th piece wise linear function"]
pub type A5R = crate::FieldReader<u16>;
#[doc = "Field `A5` writer - Slope of 6th piece wise linear function"]
pub type A5W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Slope of 6th piece wise linear function"]
    #[inline(always)]
    pub fn a5(&self) -> A5R {
        A5R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Slope of 6th piece wise linear function"]
    #[inline(always)]
    pub fn a5(&mut self) -> A5W<'_, A5Spec> {
        A5W::new(self, 0)
    }
}
#[doc = "Slope of 6th piece wise linear function\n\nYou can [`read`](crate::Reg::read) this register and get [`a5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`a5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct A5Spec;
impl crate::RegisterSpec for A5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`a5::R`](R) reader structure"]
impl crate::Readable for A5Spec {}
#[doc = "`write(|w| ..)` method takes [`a5::W`](W) writer structure"]
impl crate::Writable for A5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets A5 to value 0x037b"]
impl crate::Resettable for A5Spec {
    const RESET_VALUE: u32 = 0x037b;
}
