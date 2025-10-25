#[doc = "Register `PIDR4` reader"]
pub type R = crate::R<Pidr4Spec>;
#[doc = "Register `PIDR4` writer"]
pub type W = crate::W<Pidr4Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Coresight peripheral identification registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pidr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pidr4Spec;
impl crate::RegisterSpec for Pidr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pidr4::R`](R) reader structure"]
impl crate::Readable for Pidr4Spec {}
#[doc = "`write(|w| ..)` method takes [`pidr4::W`](W) writer structure"]
impl crate::Writable for Pidr4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PIDR4 to value 0"]
impl crate::Resettable for Pidr4Spec {}
