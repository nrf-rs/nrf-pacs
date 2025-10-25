#[doc = "Register `PSELRTS` reader"]
pub type R = crate::R<PselrtsSpec>;
#[doc = "Register `PSELRTS` writer"]
pub type W = crate::W<PselrtsSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Pin select for RTS.\n\nYou can [`read`](crate::Reg::read) this register and get [`pselrts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pselrts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PselrtsSpec;
impl crate::RegisterSpec for PselrtsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pselrts::R`](R) reader structure"]
impl crate::Readable for PselrtsSpec {}
#[doc = "`write(|w| ..)` method takes [`pselrts::W`](W) writer structure"]
impl crate::Writable for PselrtsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSELRTS to value 0xffff_ffff"]
impl crate::Resettable for PselrtsSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
