#[doc = "Register `UNUSED` reader"]
pub type R = crate::R<UnusedSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Unused.\n\nYou can [`read`](crate::Reg::read) this register and get [`unused::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UnusedSpec;
impl crate::RegisterSpec for UnusedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`unused::R`](R) reader structure"]
impl crate::Readable for UnusedSpec {}
#[doc = "`reset()` method sets UNUSED to value 0"]
impl crate::Resettable for UnusedSpec {}
