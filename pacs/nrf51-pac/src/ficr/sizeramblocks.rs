#[doc = "Register `SIZERAMBLOCKS` reader"]
pub type R = crate::R<SizeramblocksSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Size of RAM blocks in bytes.\n\nYou can [`read`](crate::Reg::read) this register and get [`sizeramblocks::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SizeramblocksSpec;
impl crate::RegisterSpec for SizeramblocksSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sizeramblocks::R`](R) reader structure"]
impl crate::Readable for SizeramblocksSpec {}
#[doc = "`reset()` method sets SIZERAMBLOCKS to value 0xffff_ffff"]
impl crate::Resettable for SizeramblocksSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
