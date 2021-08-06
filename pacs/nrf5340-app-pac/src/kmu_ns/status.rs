#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Key slot ID successfully selected by the KMU\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELECTED_A {
    #[doc = "0: No key slot ID selected by KMU"]
    DISABLED = 0,
    #[doc = "1: Key slot ID successfully selected by KMU"]
    ENABLED = 1,
}
impl From<SELECTED_A> for bool {
    #[inline(always)]
    fn from(variant: SELECTED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SELECTED` reader - Key slot ID successfully selected by the KMU"]
pub struct SELECTED_R(crate::FieldReader<bool, SELECTED_A>);
impl SELECTED_R {
    pub(crate) fn new(bits: bool) -> Self {
        SELECTED_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SELECTED_A {
        match self.bits {
            false => SELECTED_A::DISABLED,
            true => SELECTED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == SELECTED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == SELECTED_A::ENABLED
    }
}
impl core::ops::Deref for SELECTED_R {
    type Target = crate::FieldReader<bool, SELECTED_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Violation status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLOCKED_A {
    #[doc = "0: No access violation detected"]
    DISABLED = 0,
    #[doc = "1: Access violation detected and blocked"]
    ENABLED = 1,
}
impl From<BLOCKED_A> for bool {
    #[inline(always)]
    fn from(variant: BLOCKED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BLOCKED` reader - Violation status"]
pub struct BLOCKED_R(crate::FieldReader<bool, BLOCKED_A>);
impl BLOCKED_R {
    pub(crate) fn new(bits: bool) -> Self {
        BLOCKED_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLOCKED_A {
        match self.bits {
            false => BLOCKED_A::DISABLED,
            true => BLOCKED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == BLOCKED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == BLOCKED_A::ENABLED
    }
}
impl core::ops::Deref for BLOCKED_R {
    type Target = crate::FieldReader<bool, BLOCKED_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Key slot ID successfully selected by the KMU"]
    #[inline(always)]
    pub fn selected(&self) -> SELECTED_R {
        SELECTED_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Violation status"]
    #[inline(always)]
    pub fn blocked(&self) -> BLOCKED_R {
        BLOCKED_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
#[doc = "Status bits for KMU operation\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
