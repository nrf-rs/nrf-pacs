#[doc = "Register `DEVID` reader"]
pub type R = crate::R<DevidSpec>;
#[doc = "Field `PORTNUM` reader - Indicates the number of master ports implemented."]
pub type PortnumR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Indicates the number of master ports implemented."]
    #[inline(always)]
    pub fn portnum(&self) -> PortnumR {
        PortnumR::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Indicates the capabilities of the component.\n\nYou can [`read`](crate::Reg::read) this register and get [`devid::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevidSpec;
impl crate::RegisterSpec for DevidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devid::R`](R) reader structure"]
impl crate::Readable for DevidSpec {}
#[doc = "`reset()` method sets DEVID to value 0"]
impl crate::Resettable for DevidSpec {}
