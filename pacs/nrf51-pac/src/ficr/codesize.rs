#[doc = "Register `CODESIZE` reader"]
pub type R = crate::R<CodesizeSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Code memory size in pages.\n\nYou can [`read`](crate::Reg::read) this register and get [`codesize::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CodesizeSpec;
impl crate::RegisterSpec for CodesizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`codesize::R`](R) reader structure"]
impl crate::Readable for CodesizeSpec {}
#[doc = "`reset()` method sets CODESIZE to value 0xffff_ffff"]
impl crate::Resettable for CodesizeSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
