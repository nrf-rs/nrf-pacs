#[doc = "Register `PIDR_1` reader"]
pub type R = crate::R<Pidr1Spec>;
#[doc = "Register `PIDR_1` writer"]
pub type W = crate::W<Pidr1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Coresight peripheral identification registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pidr_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pidr1Spec;
impl crate::RegisterSpec for Pidr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pidr_1::R`](R) reader structure"]
impl crate::Readable for Pidr1Spec {}
#[doc = "`write(|w| ..)` method takes [`pidr_1::W`](W) writer structure"]
impl crate::Writable for Pidr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PIDR_1 to value 0"]
impl crate::Resettable for Pidr1Spec {}
