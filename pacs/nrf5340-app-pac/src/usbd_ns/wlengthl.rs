#[doc = "Register `WLENGTHL` reader"]
pub type R = crate::R<WlengthlSpec>;
#[doc = "Field `WLENGTHL` reader - SETUP data, byte 6, LSB of wLength"]
pub type WlengthlR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - SETUP data, byte 6, LSB of wLength"]
    #[inline(always)]
    pub fn wlengthl(&self) -> WlengthlR {
        WlengthlR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "SETUP data, byte 6, LSB of wLength\n\nYou can [`read`](crate::Reg::read) this register and get [`wlengthl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WlengthlSpec;
impl crate::RegisterSpec for WlengthlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wlengthl::R`](R) reader structure"]
impl crate::Readable for WlengthlSpec {}
#[doc = "`reset()` method sets WLENGTHL to value 0"]
impl crate::Resettable for WlengthlSpec {}
