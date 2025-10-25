#[doc = "Register `ITATBCTR1` reader"]
pub type R = crate::R<Itatbctr1Spec>;
#[doc = "Field `ATID` reader - Read the value of atids."]
pub type AtidR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:6 - Read the value of atids."]
    #[inline(always)]
    pub fn atid(&self) -> AtidR {
        AtidR::new((self.bits & 0x7f) as u8)
    }
}
#[doc = "Integration Test ATB Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`itatbctr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Itatbctr1Spec;
impl crate::RegisterSpec for Itatbctr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itatbctr1::R`](R) reader structure"]
impl crate::Readable for Itatbctr1Spec {}
#[doc = "`reset()` method sets ITATBCTR1 to value 0"]
impl crate::Resettable for Itatbctr1Spec {}
