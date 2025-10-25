#[doc = "Register `UNUSED3` reader"]
pub type R = crate::R<Unused3Spec>;
#[doc = "Register `UNUSED3` writer"]
pub type W = crate::W<Unused3Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Unspecified\n\nYou can [`read`](crate::Reg::read) this register and get [`unused3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`unused3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Unused3Spec;
impl crate::RegisterSpec for Unused3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`unused3::R`](R) reader structure"]
impl crate::Readable for Unused3Spec {}
#[doc = "`write(|w| ..)` method takes [`unused3::W`](W) writer structure"]
impl crate::Writable for Unused3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UNUSED3 to value 0"]
impl crate::Resettable for Unused3Spec {}
