#[doc = "Register `TEP` reader"]
pub type R = crate::R<TepSpec>;
#[doc = "Register `TEP` writer"]
pub type W = crate::W<TepSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Channel task end-point.\n\nYou can [`read`](crate::Reg::read) this register and get [`tep::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tep::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TepSpec;
impl crate::RegisterSpec for TepSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tep::R`](R) reader structure"]
impl crate::Readable for TepSpec {}
#[doc = "`write(|w| ..)` method takes [`tep::W`](W) writer structure"]
impl crate::Writable for TepSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TEP to value 0"]
impl crate::Resettable for TepSpec {}
