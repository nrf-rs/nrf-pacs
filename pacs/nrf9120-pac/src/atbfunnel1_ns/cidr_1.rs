#[doc = "Register `CIDR_1` reader"]
pub type R = crate::R<Cidr1Spec>;
#[doc = "Register `CIDR_1` writer"]
pub type W = crate::W<Cidr1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Coresight component identification registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`cidr_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cidr_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cidr1Spec;
impl crate::RegisterSpec for Cidr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cidr_1::R`](R) reader structure"]
impl crate::Readable for Cidr1Spec {}
#[doc = "`write(|w| ..)` method takes [`cidr_1::W`](W) writer structure"]
impl crate::Writable for Cidr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CIDR_1 to value 0"]
impl crate::Resettable for Cidr1Spec {}
