#[doc = "Register `BASE1` reader"]
pub type R = crate::R<Base1Spec>;
#[doc = "Register `BASE1` writer"]
pub type W = crate::W<Base1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Radio base address 1. Decision point: START task.\n\nYou can [`read`](crate::Reg::read) this register and get [`base1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`base1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Base1Spec;
impl crate::RegisterSpec for Base1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`base1::R`](R) reader structure"]
impl crate::Readable for Base1Spec {}
#[doc = "`write(|w| ..)` method takes [`base1::W`](W) writer structure"]
impl crate::Writable for Base1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BASE1 to value 0"]
impl crate::Resettable for Base1Spec {}
