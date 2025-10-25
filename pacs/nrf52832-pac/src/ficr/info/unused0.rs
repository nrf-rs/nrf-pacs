#[doc = "Register `UNUSED0[%s]` reader"]
pub type R = crate::R<Unused0Spec>;
#[doc = "Register `UNUSED0[%s]` writer"]
pub type W = crate::W<Unused0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Description collection\\[0\\]: Unspecified\n\nYou can [`read`](crate::Reg::read) this register and get [`unused0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`unused0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Unused0Spec;
impl crate::RegisterSpec for Unused0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`unused0::R`](R) reader structure"]
impl crate::Readable for Unused0Spec {}
#[doc = "`write(|w| ..)` method takes [`unused0::W`](W) writer structure"]
impl crate::Writable for Unused0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UNUSED0[%s] to value 0"]
impl crate::Resettable for Unused0Spec {}
