#[doc = "Register `WVALUEL` reader"]
pub type R = crate::R<WvaluelSpec>;
#[doc = "Field `WVALUEL` reader - SETUP data, byte 2, LSB of wValue"]
pub type WvaluelR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - SETUP data, byte 2, LSB of wValue"]
    #[inline(always)]
    pub fn wvaluel(&self) -> WvaluelR {
        WvaluelR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "SETUP data, byte 2, LSB of wValue\n\nYou can [`read`](crate::Reg::read) this register and get [`wvaluel::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WvaluelSpec;
impl crate::RegisterSpec for WvaluelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wvaluel::R`](R) reader structure"]
impl crate::Readable for WvaluelSpec {}
#[doc = "`reset()` method sets WVALUEL to value 0"]
impl crate::Resettable for WvaluelSpec {}
