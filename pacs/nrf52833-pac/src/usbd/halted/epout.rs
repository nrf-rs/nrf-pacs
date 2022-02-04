#[doc = "Register `EPOUT[%s]` reader"]
pub struct R(crate::R<EPOUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EPOUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EPOUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EPOUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "OUT endpoint halted status. Can be used as is as response to a GetStatus() request to endpoint.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum GETSTATUS_A {
    #[doc = "0: Endpoint is not halted"]
    NOTHALTED = 0,
    #[doc = "1: Endpoint is halted"]
    HALTED = 1,
}
impl From<GETSTATUS_A> for u16 {
    #[inline(always)]
    fn from(variant: GETSTATUS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GETSTATUS` reader - OUT endpoint halted status. Can be used as is as response to a GetStatus() request to endpoint."]
pub struct GETSTATUS_R(crate::FieldReader<u16, GETSTATUS_A>);
impl GETSTATUS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        GETSTATUS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GETSTATUS_A> {
        match self.bits {
            0 => Some(GETSTATUS_A::NOTHALTED),
            1 => Some(GETSTATUS_A::HALTED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NOTHALTED`"]
    #[inline(always)]
    pub fn is_not_halted(&self) -> bool {
        **self == GETSTATUS_A::NOTHALTED
    }
    #[doc = "Checks if the value of the field is `HALTED`"]
    #[inline(always)]
    pub fn is_halted(&self) -> bool {
        **self == GETSTATUS_A::HALTED
    }
}
impl core::ops::Deref for GETSTATUS_R {
    type Target = crate::FieldReader<u16, GETSTATUS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - OUT endpoint halted status. Can be used as is as response to a GetStatus() request to endpoint."]
    #[inline(always)]
    pub fn getstatus(&self) -> GETSTATUS_R {
        GETSTATUS_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Description collection: OUT endpoint halted status. Can be used as is as response to a GetStatus() request to endpoint.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epout](index.html) module"]
pub struct EPOUT_SPEC;
impl crate::RegisterSpec for EPOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [epout::R](R) reader structure"]
impl crate::Readable for EPOUT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EPOUT[%s]
to value 0"]
impl crate::Resettable for EPOUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
