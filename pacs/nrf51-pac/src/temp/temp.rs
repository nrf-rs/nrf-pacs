#[doc = "Register `TEMP` reader"]
pub type R = crate::R<TempSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Die temperature in degC, 2's complement format, 0.25 degC pecision.\n\nYou can [`read`](crate::Reg::read) this register and get [`temp::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TempSpec;
impl crate::RegisterSpec for TempSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`temp::R`](R) reader structure"]
impl crate::Readable for TempSpec {}
#[doc = "`reset()` method sets TEMP to value 0"]
impl crate::Resettable for TempSpec {}
