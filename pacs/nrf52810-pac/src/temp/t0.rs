#[doc = "Register `T0` reader"]
pub type R = crate::R<T0Spec>;
#[doc = "Register `T0` writer"]
pub type W = crate::W<T0Spec>;
#[doc = "Field `T0` reader - End point of 1st piece wise linear function"]
pub type T0R = crate::FieldReader;
#[doc = "Field `T0` writer - End point of 1st piece wise linear function"]
pub type T0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - End point of 1st piece wise linear function"]
    #[inline(always)]
    pub fn t0(&self) -> T0R {
        T0R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - End point of 1st piece wise linear function"]
    #[inline(always)]
    pub fn t0(&mut self) -> T0W<'_, T0Spec> {
        T0W::new(self, 0)
    }
}
#[doc = "End point of 1st piece wise linear function\n\nYou can [`read`](crate::Reg::read) this register and get [`t0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`t0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T0Spec;
impl crate::RegisterSpec for T0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t0::R`](R) reader structure"]
impl crate::Readable for T0Spec {}
#[doc = "`write(|w| ..)` method takes [`t0::W`](W) writer structure"]
impl crate::Writable for T0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets T0 to value 0xe2"]
impl crate::Resettable for T0Spec {
    const RESET_VALUE: u32 = 0xe2;
}
