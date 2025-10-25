#[doc = "Register `UNUSED1` reader"]
pub type R = crate::R<Unused1Spec>;
#[doc = "Register `UNUSED1` writer"]
pub type W = crate::W<Unused1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Unspecified\n\nYou can [`read`](crate::Reg::read) this register and get [`unused1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`unused1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Unused1Spec;
impl crate::RegisterSpec for Unused1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`unused1::R`](R) reader structure"]
impl crate::Readable for Unused1Spec {}
#[doc = "`write(|w| ..)` method takes [`unused1::W`](W) writer structure"]
impl crate::Writable for Unused1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UNUSED1 to value 0"]
impl crate::Resettable for Unused1Spec {}
