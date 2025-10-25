#[doc = "Register `CTESTATUS` reader"]
pub type R = crate::R<CtestatusSpec>;
#[doc = "Field `CTETIME` reader - CTETime parsed from packet"]
pub type CtetimeR = crate::FieldReader;
#[doc = "Field `RFU` reader - RFU parsed from packet"]
pub type RfuR = crate::BitReader;
#[doc = "Field `CTETYPE` reader - CTEType parsed from packet"]
pub type CtetypeR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4 - CTETime parsed from packet"]
    #[inline(always)]
    pub fn ctetime(&self) -> CtetimeR {
        CtetimeR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - RFU parsed from packet"]
    #[inline(always)]
    pub fn rfu(&self) -> RfuR {
        RfuR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - CTEType parsed from packet"]
    #[inline(always)]
    pub fn ctetype(&self) -> CtetypeR {
        CtetypeR::new(((self.bits >> 6) & 3) as u8)
    }
}
#[doc = "CTEInfo parsed from received packet\n\nYou can [`read`](crate::Reg::read) this register and get [`ctestatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtestatusSpec;
impl crate::RegisterSpec for CtestatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctestatus::R`](R) reader structure"]
impl crate::Readable for CtestatusSpec {}
#[doc = "`reset()` method sets CTESTATUS to value 0"]
impl crate::Resettable for CtestatusSpec {}
