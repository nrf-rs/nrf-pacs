#[doc = "Register `PSELTXD` reader"]
pub type R = crate::R<PseltxdSpec>;
#[doc = "Register `PSELTXD` writer"]
pub type W = crate::W<PseltxdSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Pin select for TXD.\n\nYou can [`read`](crate::Reg::read) this register and get [`pseltxd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pseltxd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PseltxdSpec;
impl crate::RegisterSpec for PseltxdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pseltxd::R`](R) reader structure"]
impl crate::Readable for PseltxdSpec {}
#[doc = "`write(|w| ..)` method takes [`pseltxd::W`](W) writer structure"]
impl crate::Writable for PseltxdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSELTXD to value 0xffff_ffff"]
impl crate::Resettable for PseltxdSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
