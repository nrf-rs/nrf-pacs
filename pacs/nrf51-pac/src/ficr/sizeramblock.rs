#[doc = "Register `SIZERAMBLOCK[%s]` reader"]
pub type R = crate::R<SizeramblockSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Deprecated array of size of RAM block in bytes. This name is kept for backward compatinility purposes. Use SIZERAMBLOCKS instead.\n\nYou can [`read`](crate::Reg::read) this register and get [`sizeramblock::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SizeramblockSpec;
impl crate::RegisterSpec for SizeramblockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sizeramblock::R`](R) reader structure"]
impl crate::Readable for SizeramblockSpec {}
#[doc = "`reset()` method sets SIZERAMBLOCK[%s] to value 0xffff_ffff"]
impl crate::Resettable for SizeramblockSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
