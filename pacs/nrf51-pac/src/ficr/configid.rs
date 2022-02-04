#[doc = "Register `CONFIGID` reader"]
pub struct R(crate::R<CONFIGID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFIGID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFIGID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFIGID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HWID` reader - Hardware Identification Number."]
pub struct HWID_R(crate::FieldReader<u16, u16>);
impl HWID_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        HWID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HWID_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FWID` reader - Firmware Identification Number pre-loaded into the flash."]
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
    #[doc = "Bits 0:15 - Hardware Identification Number."]
    #[inline(always)]
    pub fn hwid(&self) -> HWID_R {
        HWID_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Firmware Identification Number pre-loaded into the flash."]
    #[inline(always)]
    pub fn fwid(&self) -> FWID_R {
        FWID_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Configuration identifier.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [configid](index.html) module"]
pub struct CONFIGID_SPEC;
impl crate::RegisterSpec for CONFIGID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [configid::R](R) reader structure"]
impl crate::Readable for CONFIGID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CONFIGID to value 0xffff_ffff"]
impl crate::Resettable for CONFIGID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
