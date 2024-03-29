#[doc = "Register `CTESTATUS` reader"]
pub struct R(crate::R<CTESTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTESTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTESTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTESTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CTETIME` reader - CTETime parsed from packet"]
pub type CTETIME_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RFU` reader - RFU parsed from packet"]
pub type RFU_R = crate::BitReader<bool>;
#[doc = "Field `CTETYPE` reader - CTEType parsed from packet"]
pub type CTETYPE_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:4 - CTETime parsed from packet"]
    #[inline(always)]
    pub fn ctetime(&self) -> CTETIME_R {
        CTETIME_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - RFU parsed from packet"]
    #[inline(always)]
    pub fn rfu(&self) -> RFU_R {
        RFU_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - CTEType parsed from packet"]
    #[inline(always)]
    pub fn ctetype(&self) -> CTETYPE_R {
        CTETYPE_R::new(((self.bits >> 6) & 3) as u8)
    }
}
#[doc = "CTEInfo parsed from received packet\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctestatus](index.html) module"]
pub struct CTESTATUS_SPEC;
impl crate::RegisterSpec for CTESTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctestatus::R](R) reader structure"]
impl crate::Readable for CTESTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CTESTATUS to value 0"]
impl crate::Resettable for CTESTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
