#[doc = "Register `WLENGTHH` reader"]
pub type R = crate::R<WlengthhSpec>;
#[doc = "Field `WLENGTHH` reader - SETUP data, byte 7, MSB of wLength"]
pub type WlengthhR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - SETUP data, byte 7, MSB of wLength"]
    #[inline(always)]
    pub fn wlengthh(&self) -> WlengthhR {
        WlengthhR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "SETUP data, byte 7, MSB of wLength\n\nYou can [`read`](crate::Reg::read) this register and get [`wlengthh::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WlengthhSpec;
impl crate::RegisterSpec for WlengthhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wlengthh::R`](R) reader structure"]
impl crate::Readable for WlengthhSpec {}
#[doc = "`reset()` method sets WLENGTHH to value 0"]
impl crate::Resettable for WlengthhSpec {}
