#[doc = "Register `CLENR0` reader"]
pub type R = crate::R<Clenr0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Length of code region 0 in bytes.\n\nYou can [`read`](crate::Reg::read) this register and get [`clenr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Clenr0Spec;
impl crate::RegisterSpec for Clenr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clenr0::R`](R) reader structure"]
impl crate::Readable for Clenr0Spec {}
#[doc = "`reset()` method sets CLENR0 to value 0xffff_ffff"]
impl crate::Resettable for Clenr0Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
