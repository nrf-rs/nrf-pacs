#[doc = "Register `PSELCTS` reader"]
pub type R = crate::R<PselctsSpec>;
#[doc = "Register `PSELCTS` writer"]
pub type W = crate::W<PselctsSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Pin select for CTS.\n\nYou can [`read`](crate::Reg::read) this register and get [`pselcts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pselcts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PselctsSpec;
impl crate::RegisterSpec for PselctsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pselcts::R`](R) reader structure"]
impl crate::Readable for PselctsSpec {}
#[doc = "`write(|w| ..)` method takes [`pselcts::W`](W) writer structure"]
impl crate::Writable for PselctsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSELCTS to value 0xffff_ffff"]
impl crate::Resettable for PselctsSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
