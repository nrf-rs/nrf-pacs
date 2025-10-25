#[doc = "Register `PSELB` reader"]
pub type R = crate::R<PselbSpec>;
#[doc = "Register `PSELB` writer"]
pub type W = crate::W<PselbSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Pin select for phase B input.\n\nYou can [`read`](crate::Reg::read) this register and get [`pselb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pselb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PselbSpec;
impl crate::RegisterSpec for PselbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pselb::R`](R) reader structure"]
impl crate::Readable for PselbSpec {}
#[doc = "`write(|w| ..)` method takes [`pselb::W`](W) writer structure"]
impl crate::Writable for PselbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSELB to value 0xffff_ffff"]
impl crate::Resettable for PselbSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
