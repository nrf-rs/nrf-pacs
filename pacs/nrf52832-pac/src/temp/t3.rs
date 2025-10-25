#[doc = "Register `T3` reader"]
pub type R = crate::R<T3Spec>;
#[doc = "Register `T3` writer"]
pub type W = crate::W<T3Spec>;
#[doc = "Field `T3` reader - End point of 4th piece wise linear function"]
pub type T3R = crate::FieldReader;
#[doc = "Field `T3` writer - End point of 4th piece wise linear function"]
pub type T3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - End point of 4th piece wise linear function"]
    #[inline(always)]
    pub fn t3(&self) -> T3R {
        T3R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - End point of 4th piece wise linear function"]
    #[inline(always)]
    pub fn t3(&mut self) -> T3W<'_, T3Spec> {
        T3W::new(self, 0)
    }
}
#[doc = "End point of 4th piece wise linear function\n\nYou can [`read`](crate::Reg::read) this register and get [`t3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`t3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T3Spec;
impl crate::RegisterSpec for T3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t3::R`](R) reader structure"]
impl crate::Readable for T3Spec {}
#[doc = "`write(|w| ..)` method takes [`t3::W`](W) writer structure"]
impl crate::Writable for T3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets T3 to value 0x19"]
impl crate::Resettable for T3Spec {
    const RESET_VALUE: u32 = 0x19;
}
