#[doc = "Register `PSELMOSI` reader"]
pub type R = crate::R<PselmosiSpec>;
#[doc = "Register `PSELMOSI` writer"]
pub type W = crate::W<PselmosiSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Pin select for MOSI.\n\nYou can [`read`](crate::Reg::read) this register and get [`pselmosi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pselmosi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PselmosiSpec;
impl crate::RegisterSpec for PselmosiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pselmosi::R`](R) reader structure"]
impl crate::Readable for PselmosiSpec {}
#[doc = "`write(|w| ..)` method takes [`pselmosi::W`](W) writer structure"]
impl crate::Writable for PselmosiSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSELMOSI to value 0xffff_ffff"]
impl crate::Resettable for PselmosiSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
