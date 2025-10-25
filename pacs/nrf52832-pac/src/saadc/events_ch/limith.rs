#[doc = "Register `LIMITH` reader"]
pub type R = crate::R<LimithSpec>;
#[doc = "Register `LIMITH` writer"]
pub type W = crate::W<LimithSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Description cluster\\[0\\]: Last results is equal or above CH\\[0\\].LIMIT.HIGH\n\nYou can [`read`](crate::Reg::read) this register and get [`limith::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`limith::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LimithSpec;
impl crate::RegisterSpec for LimithSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`limith::R`](R) reader structure"]
impl crate::Readable for LimithSpec {}
#[doc = "`write(|w| ..)` method takes [`limith::W`](W) writer structure"]
impl crate::Writable for LimithSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LIMITH to value 0"]
impl crate::Resettable for LimithSpec {}
