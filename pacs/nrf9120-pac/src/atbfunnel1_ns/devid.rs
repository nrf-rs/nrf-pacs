#[doc = "Register `DEVID` reader"]
pub type R = crate::R<DevidSpec>;
#[doc = "Field `PORTCOUNT` reader - Indicates the number of input ports connected. 0x0 and 0x1 are illegal values."]
pub type PortcountR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Indicates the number of input ports connected. 0x0 and 0x1 are illegal values."]
    #[inline(always)]
    pub fn portcount(&self) -> PortcountR {
        PortcountR::new((self.bits & 0x0f) as u8)
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
