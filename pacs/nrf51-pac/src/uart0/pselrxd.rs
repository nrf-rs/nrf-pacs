#[doc = "Register `PSELRXD` reader"]
pub type R = crate::R<PselrxdSpec>;
#[doc = "Register `PSELRXD` writer"]
pub type W = crate::W<PselrxdSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Pin select for RXD.\n\nYou can [`read`](crate::Reg::read) this register and get [`pselrxd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pselrxd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PselrxdSpec;
impl crate::RegisterSpec for PselrxdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pselrxd::R`](R) reader structure"]
impl crate::Readable for PselrxdSpec {}
#[doc = "`write(|w| ..)` method takes [`pselrxd::W`](W) writer structure"]
impl crate::Writable for PselrxdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSELRXD to value 0xffff_ffff"]
impl crate::Resettable for PselrxdSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
