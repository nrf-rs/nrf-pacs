#[doc = "Register `DEVICETYPE` reader"]
pub struct R(crate::R<DEVICETYPE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEVICETYPE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEVICETYPE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEVICETYPE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Device type\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum DEVICETYPE_A {
    #[doc = "0: Device is an physical DIE"]
    DIE = 0,
    #[doc = "4294967295: Device is an FPGA"]
    FPGA = 4294967295,
}
impl From<DEVICETYPE_A> for u32 {
    #[inline(always)]
    fn from(variant: DEVICETYPE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DEVICETYPE` reader - Device type"]
pub struct DEVICETYPE_R(crate::FieldReader<u32, DEVICETYPE_A>);
impl DEVICETYPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        DEVICETYPE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DEVICETYPE_A> {
        match self.bits {
            0 => Some(DEVICETYPE_A::DIE),
            4294967295 => Some(DEVICETYPE_A::FPGA),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DIE`"]
    #[inline(always)]
    pub fn is_die(&self) -> bool {
        **self == DEVICETYPE_A::DIE
    }
    #[doc = "Checks if the value of the field is `FPGA`"]
    #[inline(always)]
    pub fn is_fpga(&self) -> bool {
        **self == DEVICETYPE_A::FPGA
    }
}
impl core::ops::Deref for DEVICETYPE_R {
    type Target = crate::FieldReader<u32, DEVICETYPE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Device type"]
    #[inline(always)]
    pub fn devicetype(&self) -> DEVICETYPE_R {
        DEVICETYPE_R::new(self.bits)
    }
}
#[doc = "Device type\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devicetype](index.html) module"]
pub struct DEVICETYPE_SPEC;
impl crate::RegisterSpec for DEVICETYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [devicetype::R](R) reader structure"]
impl crate::Readable for DEVICETYPE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DEVICETYPE to value 0xffff_ffff"]
impl crate::Resettable for DEVICETYPE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
