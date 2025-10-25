#[doc = "Register `KEY3` writer"]
pub type W = crate::W<Key3Spec>;
#[doc = "Field `KEY3` writer - Bits 127:96 of XIP AES KEY"]
pub type Key3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Bits 127:96 of XIP AES KEY"]
    #[inline(always)]
    pub fn key3(&mut self) -> Key3W<'_, Key3Spec> {
        Key3W::new(self, 0)
    }
}
#[doc = "Bits 127:96 of XIP AES KEY\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key3::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Key3Spec;
impl crate::RegisterSpec for Key3Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`key3::W`](W) writer structure"]
impl crate::Writable for Key3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets KEY3 to value 0"]
impl crate::Resettable for Key3Spec {}
