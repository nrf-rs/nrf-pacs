#[doc = "Register `PIDR_3` reader"]
pub type R = crate::R<Pidr3Spec>;
#[doc = "Register `PIDR_3` writer"]
pub type W = crate::W<Pidr3Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Coresight peripheral identification registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pidr_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pidr3Spec;
impl crate::RegisterSpec for Pidr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pidr_3::R`](R) reader structure"]
impl crate::Readable for Pidr3Spec {}
#[doc = "`write(|w| ..)` method takes [`pidr_3::W`](W) writer structure"]
impl crate::Writable for Pidr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PIDR_3 to value 0"]
impl crate::Resettable for Pidr3Spec {}
