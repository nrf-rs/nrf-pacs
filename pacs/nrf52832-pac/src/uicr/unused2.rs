#[doc = "Register `UNUSED2` reader"]
pub type R = crate::R<Unused2Spec>;
#[doc = "Register `UNUSED2` writer"]
pub type W = crate::W<Unused2Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Unspecified\n\nYou can [`read`](crate::Reg::read) this register and get [`unused2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`unused2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Unused2Spec;
impl crate::RegisterSpec for Unused2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`unused2::R`](R) reader structure"]
impl crate::Readable for Unused2Spec {}
#[doc = "`write(|w| ..)` method takes [`unused2::W`](W) writer structure"]
impl crate::Writable for Unused2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UNUSED2 to value 0"]
impl crate::Resettable for Unused2Spec {}
