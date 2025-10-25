#[doc = "Register `PSELA` reader"]
pub type R = crate::R<PselaSpec>;
#[doc = "Register `PSELA` writer"]
pub type W = crate::W<PselaSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Pin select for phase A input.\n\nYou can [`read`](crate::Reg::read) this register and get [`psela::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psela::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PselaSpec;
impl crate::RegisterSpec for PselaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psela::R`](R) reader structure"]
impl crate::Readable for PselaSpec {}
#[doc = "`write(|w| ..)` method takes [`psela::W`](W) writer structure"]
impl crate::Writable for PselaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSELA to value 0xffff_ffff"]
impl crate::Resettable for PselaSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
