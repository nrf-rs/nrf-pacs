#[doc = "Register `RA` reader"]
pub type R = crate::R<RaSpec>;
#[doc = "Register `RA` writer"]
pub type W = crate::W<RaSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Description cluster\\[0\\]: Read access to peripheral region 0 detected\n\nYou can [`read`](crate::Reg::read) this register and get [`ra::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ra::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RaSpec;
impl crate::RegisterSpec for RaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ra::R`](R) reader structure"]
impl crate::Readable for RaSpec {}
#[doc = "`write(|w| ..)` method takes [`ra::W`](W) writer structure"]
impl crate::Writable for RaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RA to value 0"]
impl crate::Resettable for RaSpec {}
