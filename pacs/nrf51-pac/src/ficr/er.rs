#[doc = "Register `ER[%s]` reader"]
pub type R = crate::R<ErSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Encryption root.\n\nYou can [`read`](crate::Reg::read) this register and get [`er::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErSpec;
impl crate::RegisterSpec for ErSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`er::R`](R) reader structure"]
impl crate::Readable for ErSpec {}
#[doc = "`reset()` method sets ER[%s] to value 0xffff_ffff"]
impl crate::Resettable for ErSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
