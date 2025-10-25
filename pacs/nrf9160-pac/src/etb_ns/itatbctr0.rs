#[doc = "Register `ITATBCTR0` reader"]
pub type R = crate::R<Itatbctr0Spec>;
#[doc = "Field `ATVALID` reader - Read the value of atvalids."]
pub type AtvalidR = crate::BitReader;
#[doc = "Field `AFREADY` reader - Read the value of afreadys."]
pub type AfreadyR = crate::BitReader;
#[doc = "Field `ATBYTES` reader - Read the value of atbytess."]
pub type AtbytesR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Read the value of atvalids."]
    #[inline(always)]
    pub fn atvalid(&self) -> AtvalidR {
        AtvalidR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Read the value of afreadys."]
    #[inline(always)]
    pub fn afready(&self) -> AfreadyR {
        AfreadyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Read the value of atbytess."]
    #[inline(always)]
    pub fn atbytes(&self) -> AtbytesR {
        AtbytesR::new(((self.bits >> 8) & 3) as u8)
    }
}
#[doc = "Integration Test ATB Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`itatbctr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Itatbctr0Spec;
impl crate::RegisterSpec for Itatbctr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itatbctr0::R`](R) reader structure"]
impl crate::Readable for Itatbctr0Spec {}
#[doc = "`reset()` method sets ITATBCTR0 to value 0"]
impl crate::Resettable for Itatbctr0Spec {}
