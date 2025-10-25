#[doc = "Register `PSELCSN` reader"]
pub type R = crate::R<PselcsnSpec>;
#[doc = "Register `PSELCSN` writer"]
pub type W = crate::W<PselcsnSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Pin select for CSN.\n\nYou can [`read`](crate::Reg::read) this register and get [`pselcsn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pselcsn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PselcsnSpec;
impl crate::RegisterSpec for PselcsnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pselcsn::R`](R) reader structure"]
impl crate::Readable for PselcsnSpec {}
#[doc = "`write(|w| ..)` method takes [`pselcsn::W`](W) writer structure"]
impl crate::Writable for PselcsnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSELCSN to value 0xffff_ffff"]
impl crate::Resettable for PselcsnSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
