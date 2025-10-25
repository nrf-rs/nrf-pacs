#[doc = "Register `ACC` reader"]
pub type R = crate::R<AccSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Accumulated valid transitions register.\n\nYou can [`read`](crate::Reg::read) this register and get [`acc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AccSpec;
impl crate::RegisterSpec for AccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acc::R`](R) reader structure"]
impl crate::Readable for AccSpec {}
#[doc = "`reset()` method sets ACC to value 0"]
impl crate::Resettable for AccSpec {}
