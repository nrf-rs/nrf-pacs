#[doc = "Register `ECBDATAPTR` reader"]
pub type R = crate::R<EcbdataptrSpec>;
#[doc = "Register `ECBDATAPTR` writer"]
pub type W = crate::W<EcbdataptrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "ECB block encrypt memory pointer.\n\nYou can [`read`](crate::Reg::read) this register and get [`ecbdataptr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecbdataptr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EcbdataptrSpec;
impl crate::RegisterSpec for EcbdataptrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecbdataptr::R`](R) reader structure"]
impl crate::Readable for EcbdataptrSpec {}
#[doc = "`write(|w| ..)` method takes [`ecbdataptr::W`](W) writer structure"]
impl crate::Writable for EcbdataptrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ECBDATAPTR to value 0"]
impl crate::Resettable for EcbdataptrSpec {}
