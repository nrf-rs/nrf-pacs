#[doc = "Register `PSELSCK` reader"]
pub type R = crate::R<PselsckSpec>;
#[doc = "Register `PSELSCK` writer"]
pub type W = crate::W<PselsckSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Pin select for SCK.\n\nYou can [`read`](crate::Reg::read) this register and get [`pselsck::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pselsck::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PselsckSpec;
impl crate::RegisterSpec for PselsckSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pselsck::R`](R) reader structure"]
impl crate::Readable for PselsckSpec {}
#[doc = "`write(|w| ..)` method takes [`pselsck::W`](W) writer structure"]
impl crate::Writable for PselsckSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSELSCK to value 0xffff_ffff"]
impl crate::Resettable for PselsckSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
