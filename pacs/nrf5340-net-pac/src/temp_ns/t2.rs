#[doc = "Register `T2` reader"]
pub type R = crate::R<T2Spec>;
#[doc = "Register `T2` writer"]
pub type W = crate::W<T2Spec>;
#[doc = "Field `T2` reader - Endpoint of third piecewise linear function"]
pub type T2R = crate::FieldReader;
#[doc = "Field `T2` writer - Endpoint of third piecewise linear function"]
pub type T2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Endpoint of third piecewise linear function"]
    #[inline(always)]
    pub fn t2(&self) -> T2R {
        T2R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Endpoint of third piecewise linear function"]
    #[inline(always)]
    pub fn t2(&mut self) -> T2W<'_, T2Spec> {
        T2W::new(self, 0)
    }
}
#[doc = "Endpoint of third piecewise linear function\n\nYou can [`read`](crate::Reg::read) this register and get [`t2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`t2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T2Spec;
impl crate::RegisterSpec for T2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t2::R`](R) reader structure"]
impl crate::Readable for T2Spec {}
#[doc = "`write(|w| ..)` method takes [`t2::W`](W) writer structure"]
impl crate::Writable for T2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets T2 to value 0x10"]
impl crate::Resettable for T2Spec {
    const RESET_VALUE: u32 = 0x10;
}
