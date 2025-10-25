#[doc = "Register `PSELLED` reader"]
pub type R = crate::R<PselledSpec>;
#[doc = "Register `PSELLED` writer"]
pub type W = crate::W<PselledSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Pin select for LED output.\n\nYou can [`read`](crate::Reg::read) this register and get [`pselled::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pselled::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PselledSpec;
impl crate::RegisterSpec for PselledSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pselled::R`](R) reader structure"]
impl crate::Readable for PselledSpec {}
#[doc = "`write(|w| ..)` method takes [`pselled::W`](W) writer structure"]
impl crate::Writable for PselledSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSELLED to value 0xffff_ffff"]
impl crate::Resettable for PselledSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
