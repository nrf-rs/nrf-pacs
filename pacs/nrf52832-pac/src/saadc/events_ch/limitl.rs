#[doc = "Register `LIMITL` reader"]
pub type R = crate::R<LimitlSpec>;
#[doc = "Register `LIMITL` writer"]
pub type W = crate::W<LimitlSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Description cluster\\[0\\]: Last results is equal or below CH\\[0\\].LIMIT.LOW\n\nYou can [`read`](crate::Reg::read) this register and get [`limitl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`limitl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LimitlSpec;
impl crate::RegisterSpec for LimitlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`limitl::R`](R) reader structure"]
impl crate::Readable for LimitlSpec {}
#[doc = "`write(|w| ..)` method takes [`limitl::W`](W) writer structure"]
impl crate::Writable for LimitlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LIMITL to value 0"]
impl crate::Resettable for LimitlSpec {}
