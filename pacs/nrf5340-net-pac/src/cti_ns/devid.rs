#[doc = "Register `DEVID` reader"]
pub struct R(crate::R<DEVID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEVID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEVID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEVID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EXTMUXNUM` reader - Indicates the number of multiplexers available on Trigger Inputs and Trigger Outputs that are using asicctl. The default value of 0b00000 indicates that no multiplexing is present."]
pub struct EXTMUXNUM_R(crate::FieldReader<u8, u8>);
impl EXTMUXNUM_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXTMUXNUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTMUXNUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NUMTRIG` reader - Number of ECT triggers available."]
pub struct NUMTRIG_R(crate::FieldReader<u8, u8>);
impl NUMTRIG_R {
    pub(crate) fn new(bits: u8) -> Self {
        NUMTRIG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NUMTRIG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NUMCH` reader - Number of ECT channels available."]
pub struct NUMCH_R(crate::FieldReader<u8, u8>);
impl NUMCH_R {
    pub(crate) fn new(bits: u8) -> Self {
        NUMCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NUMCH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:4 - Indicates the number of multiplexers available on Trigger Inputs and Trigger Outputs that are using asicctl. The default value of 0b00000 indicates that no multiplexing is present."]
    #[inline(always)]
    pub fn extmuxnum(&self) -> EXTMUXNUM_R {
        EXTMUXNUM_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:15 - Number of ECT triggers available."]
    #[inline(always)]
    pub fn numtrig(&self) -> NUMTRIG_R {
        NUMTRIG_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Number of ECT channels available."]
    #[inline(always)]
    pub fn numch(&self) -> NUMCH_R {
        NUMCH_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
#[doc = "Device Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devid](index.html) module"]
pub struct DEVID_SPEC;
impl crate::RegisterSpec for DEVID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [devid::R](R) reader structure"]
impl crate::Readable for DEVID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DEVID to value 0x0004_0800"]
impl crate::Resettable for DEVID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0004_0800
    }
}
