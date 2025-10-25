#[doc = "Register `PSELMISO` reader"]
pub type R = crate::R<PselmisoSpec>;
#[doc = "Register `PSELMISO` writer"]
pub type W = crate::W<PselmisoSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Pin select for MISO.\n\nYou can [`read`](crate::Reg::read) this register and get [`pselmiso::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pselmiso::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PselmisoSpec;
impl crate::RegisterSpec for PselmisoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pselmiso::R`](R) reader structure"]
impl crate::Readable for PselmisoSpec {}
#[doc = "`write(|w| ..)` method takes [`pselmiso::W`](W) writer structure"]
impl crate::Writable for PselmisoSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSELMISO to value 0xffff_ffff"]
impl crate::Resettable for PselmisoSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
