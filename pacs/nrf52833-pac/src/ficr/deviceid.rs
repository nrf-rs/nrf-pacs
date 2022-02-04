#[doc = "Register `DEVICEID[%s]` reader"]
pub struct R(crate::R<DEVICEID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEVICEID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEVICEID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEVICEID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DEVICEID` reader - 64 bit unique device identifier"]
pub struct DEVICEID_R(crate::FieldReader<u32, u32>);
impl DEVICEID_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        DEVICEID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEVICEID_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - 64 bit unique device identifier"]
    #[inline(always)]
    pub fn deviceid(&self) -> DEVICEID_R {
        DEVICEID_R::new(self.bits)
    }
}
#[doc = "Description collection: Device identifier\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [deviceid](index.html) module"]
pub struct DEVICEID_SPEC;
impl crate::RegisterSpec for DEVICEID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [deviceid::R](R) reader structure"]
impl crate::Readable for DEVICEID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DEVICEID[%s]
to value 0xffff_ffff"]
impl crate::Resettable for DEVICEID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
