#[doc = "Register `NUMRAMBLOCK` reader"]
pub type R = crate::R<NumramblockSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Number of individualy controllable RAM blocks.\n\nYou can [`read`](crate::Reg::read) this register and get [`numramblock::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NumramblockSpec;
impl crate::RegisterSpec for NumramblockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`numramblock::R`](R) reader structure"]
impl crate::Readable for NumramblockSpec {}
#[doc = "`reset()` method sets NUMRAMBLOCK to value 0xffff_ffff"]
impl crate::Resettable for NumramblockSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
