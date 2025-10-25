#[doc = "Register `WVALUEH` reader"]
pub type R = crate::R<WvaluehSpec>;
#[doc = "Field `WVALUEH` reader - SETUP data, byte 3, MSB of wValue"]
pub type WvaluehR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - SETUP data, byte 3, MSB of wValue"]
    #[inline(always)]
    pub fn wvalueh(&self) -> WvaluehR {
        WvaluehR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "SETUP data, byte 3, MSB of wValue\n\nYou can [`read`](crate::Reg::read) this register and get [`wvalueh::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WvaluehSpec;
impl crate::RegisterSpec for WvaluehSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wvalueh::R`](R) reader structure"]
impl crate::Readable for WvaluehSpec {}
#[doc = "`reset()` method sets WVALUEH to value 0"]
impl crate::Resettable for WvaluehSpec {}
