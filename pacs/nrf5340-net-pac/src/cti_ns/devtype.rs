#[doc = "Register `DEVTYPE` reader"]
pub struct R(crate::R<DEVTYPE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEVTYPE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEVTYPE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEVTYPE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Major classification of the type of the debug component as specified in the Arm Architecture Specification for this debug and trace component.\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MAJOR_A {
    #[doc = "4: Indicates that this component allows a debugger to control other components in an Arm CoreSight SoC-400 system."]
    CONTROLLER = 4,
}
impl From<MAJOR_A> for u8 {
    #[inline(always)]
    fn from(variant: MAJOR_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MAJOR` reader - Major classification of the type of the debug component as specified in the Arm Architecture Specification for this debug and trace component."]
pub struct MAJOR_R(crate::FieldReader<u8, MAJOR_A>);
impl MAJOR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MAJOR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MAJOR_A> {
        match self.bits {
            4 => Some(MAJOR_A::CONTROLLER),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CONTROLLER`"]
    #[inline(always)]
    pub fn is_controller(&self) -> bool {
        **self == MAJOR_A::CONTROLLER
    }
}
impl core::ops::Deref for MAJOR_R {
    type Target = crate::FieldReader<u8, MAJOR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Sub-classification of the type of the debug component as specified in the Arm Architecture Specification within the major classification as specified in the MAJOR field.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SUB_A {
    #[doc = "1: Indicates that this component is a sub-triggering component."]
    CROSSTRIGGER = 1,
}
impl From<SUB_A> for u8 {
    #[inline(always)]
    fn from(variant: SUB_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SUB` reader - Sub-classification of the type of the debug component as specified in the Arm Architecture Specification within the major classification as specified in the MAJOR field."]
pub struct SUB_R(crate::FieldReader<u8, SUB_A>);
impl SUB_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SUB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SUB_A> {
        match self.bits {
            1 => Some(SUB_A::CROSSTRIGGER),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CROSSTRIGGER`"]
    #[inline(always)]
    pub fn is_crosstrigger(&self) -> bool {
        **self == SUB_A::CROSSTRIGGER
    }
}
impl core::ops::Deref for SUB_R {
    type Target = crate::FieldReader<u8, SUB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - Major classification of the type of the debug component as specified in the Arm Architecture Specification for this debug and trace component."]
    #[inline(always)]
    pub fn major(&self) -> MAJOR_R {
        MAJOR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Sub-classification of the type of the debug component as specified in the Arm Architecture Specification within the major classification as specified in the MAJOR field."]
    #[inline(always)]
    pub fn sub(&self) -> SUB_R {
        SUB_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "Device Type Identifier register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devtype](index.html) module"]
pub struct DEVTYPE_SPEC;
impl crate::RegisterSpec for DEVTYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [devtype::R](R) reader structure"]
impl crate::Readable for DEVTYPE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DEVTYPE to value 0x14"]
impl crate::Resettable for DEVTYPE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x14
    }
}
