#[doc = "Register `ERASEPAGE` reader"]
pub type R = crate::R<ErasepageSpec>;
#[doc = "Register `ERASEPAGE` writer"]
pub type W = crate::W<ErasepageSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Register for erasing a non-protected non-volatile memory page.\n\nYou can [`read`](crate::Reg::read) this register and get [`erasepage::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`erasepage::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErasepageSpec;
impl crate::RegisterSpec for ErasepageSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`erasepage::R`](R) reader structure"]
impl crate::Readable for ErasepageSpec {}
#[doc = "`write(|w| ..)` method takes [`erasepage::W`](W) writer structure"]
impl crate::Writable for ErasepageSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ERASEPAGE to value 0"]
impl crate::Resettable for ErasepageSpec {}
