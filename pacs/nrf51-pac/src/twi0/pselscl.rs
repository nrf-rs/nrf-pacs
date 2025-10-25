#[doc = "Register `PSELSCL` reader"]
pub type R = crate::R<PselsclSpec>;
#[doc = "Register `PSELSCL` writer"]
pub type W = crate::W<PselsclSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Pin select for SCL.\n\nYou can [`read`](crate::Reg::read) this register and get [`pselscl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pselscl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PselsclSpec;
impl crate::RegisterSpec for PselsclSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pselscl::R`](R) reader structure"]
impl crate::Readable for PselsclSpec {}
#[doc = "`write(|w| ..)` method takes [`pselscl::W`](W) writer structure"]
impl crate::Writable for PselsclSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSELSCL to value 0xffff_ffff"]
impl crate::Resettable for PselsclSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
