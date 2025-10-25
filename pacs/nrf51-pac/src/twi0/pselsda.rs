#[doc = "Register `PSELSDA` reader"]
pub type R = crate::R<PselsdaSpec>;
#[doc = "Register `PSELSDA` writer"]
pub type W = crate::W<PselsdaSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Pin select for SDA.\n\nYou can [`read`](crate::Reg::read) this register and get [`pselsda::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pselsda::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PselsdaSpec;
impl crate::RegisterSpec for PselsdaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pselsda::R`](R) reader structure"]
impl crate::Readable for PselsdaSpec {}
#[doc = "`write(|w| ..)` method takes [`pselsda::W`](W) writer structure"]
impl crate::Writable for PselsdaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSELSDA to value 0xffff_ffff"]
impl crate::Resettable for PselsdaSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
