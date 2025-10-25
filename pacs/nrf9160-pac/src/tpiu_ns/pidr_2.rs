#[doc = "Register `PIDR_2` reader"]
pub type R = crate::R<Pidr2Spec>;
#[doc = "Register `PIDR_2` writer"]
pub type W = crate::W<Pidr2Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Coresight peripheral identification registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pidr_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pidr2Spec;
impl crate::RegisterSpec for Pidr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pidr_2::R`](R) reader structure"]
impl crate::Readable for Pidr2Spec {}
#[doc = "`write(|w| ..)` method takes [`pidr_2::W`](W) writer structure"]
impl crate::Writable for Pidr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PIDR_2 to value 0"]
impl crate::Resettable for Pidr2Spec {}
