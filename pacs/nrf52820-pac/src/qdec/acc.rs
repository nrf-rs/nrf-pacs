#[doc = "Register `ACC` reader"]
pub type R = crate::R<AccSpec>;
#[doc = "Field `ACC` reader - Register accumulating all valid samples (not double transition) read from the SAMPLE register."]
pub type AccR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Register accumulating all valid samples (not double transition) read from the SAMPLE register."]
    #[inline(always)]
    pub fn acc(&self) -> AccR {
        AccR::new(self.bits)
    }
}
#[doc = "Register accumulating the valid transitions\n\nYou can [`read`](crate::Reg::read) this register and get [`acc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AccSpec;
impl crate::RegisterSpec for AccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acc::R`](R) reader structure"]
impl crate::Readable for AccSpec {}
#[doc = "`reset()` method sets ACC to value 0"]
impl crate::Resettable for AccSpec {}
