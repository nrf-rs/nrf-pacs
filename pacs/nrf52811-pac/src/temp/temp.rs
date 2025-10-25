#[doc = "Register `TEMP` reader"]
pub type R = crate::R<TempSpec>;
#[doc = "Field `TEMP` reader - Temperature in degC (0.25deg steps)"]
pub type TempR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Temperature in degC (0.25deg steps)"]
    #[inline(always)]
    pub fn temp(&self) -> TempR {
        TempR::new(self.bits)
    }
}
#[doc = "Temperature in degC (0.25deg steps)\n\nYou can [`read`](crate::Reg::read) this register and get [`temp::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TempSpec;
impl crate::RegisterSpec for TempSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`temp::R`](R) reader structure"]
impl crate::Readable for TempSpec {}
#[doc = "`reset()` method sets TEMP to value 0"]
impl crate::Resettable for TempSpec {}
