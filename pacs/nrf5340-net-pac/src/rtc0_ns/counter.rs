#[doc = "Register `COUNTER` reader"]
pub type R = crate::R<CounterSpec>;
#[doc = "Field `COUNTER` reader - Counter value"]
pub type CounterR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23 - Counter value"]
    #[inline(always)]
    pub fn counter(&self) -> CounterR {
        CounterR::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "Current counter value\n\nYou can [`read`](crate::Reg::read) this register and get [`counter::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CounterSpec;
impl crate::RegisterSpec for CounterSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`counter::R`](R) reader structure"]
impl crate::Readable for CounterSpec {}
#[doc = "`reset()` method sets COUNTER to value 0"]
impl crate::Resettable for CounterSpec {}
