#[doc = "Register `FWID` reader"]
pub struct R(crate::R<FWID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FWID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FWID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FWID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FWID` reader - Identification number for the firmware loaded into the chip."]
pub struct FWID_R(crate::FieldReader<u16, u16>);
impl FWID_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        FWID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FWID_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Identification number for the firmware loaded into the chip."]
    #[inline(always)]
    pub fn fwid(&self) -> FWID_R {
        FWID_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Firmware ID.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fwid](index.html) module"]
pub struct FWID_SPEC;
impl crate::RegisterSpec for FWID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fwid::R](R) reader structure"]
impl crate::Readable for FWID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FWID to value 0xffff_ffff"]
impl crate::Resettable for FWID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
