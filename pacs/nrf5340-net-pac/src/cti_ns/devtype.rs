#[doc = "Register `DEVTYPE` reader"]
pub type R = crate::R<DevtypeSpec>;
#[doc = "Major classification of the type of the debug component as specified in the Arm Architecture Specification for this debug and trace component.\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Major {
    #[doc = "4: Indicates that this component allows a debugger to control other components in an Arm CoreSight SoC-400 system."]
    Controller = 4,
}
impl From<Major> for u8 {
    #[inline(always)]
    fn from(variant: Major) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Major {
    type Ux = u8;
}
impl crate::IsEnum for Major {}
#[doc = "Field `MAJOR` reader - Major classification of the type of the debug component as specified in the Arm Architecture Specification for this debug and trace component."]
pub type MajorR = crate::FieldReader<Major>;
impl MajorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Major> {
        match self.bits {
            4 => Some(Major::Controller),
            _ => None,
        }
    }
    #[doc = "Indicates that this component allows a debugger to control other components in an Arm CoreSight SoC-400 system."]
    #[inline(always)]
    pub fn is_controller(&self) -> bool {
        *self == Major::Controller
    }
}
#[doc = "Sub-classification of the type of the debug component as specified in the Arm Architecture Specification within the major classification as specified in the MAJOR field.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sub {
    #[doc = "1: Indicates that this component is a sub-triggering component."]
    Crosstrigger = 1,
}
impl From<Sub> for u8 {
    #[inline(always)]
    fn from(variant: Sub) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sub {
    type Ux = u8;
}
impl crate::IsEnum for Sub {}
#[doc = "Field `SUB` reader - Sub-classification of the type of the debug component as specified in the Arm Architecture Specification within the major classification as specified in the MAJOR field."]
pub type SubR = crate::FieldReader<Sub>;
impl SubR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sub> {
        match self.bits {
            1 => Some(Sub::Crosstrigger),
            _ => None,
        }
    }
    #[doc = "Indicates that this component is a sub-triggering component."]
    #[inline(always)]
    pub fn is_crosstrigger(&self) -> bool {
        *self == Sub::Crosstrigger
    }
}
impl R {
    #[doc = "Bits 0:3 - Major classification of the type of the debug component as specified in the Arm Architecture Specification for this debug and trace component."]
    #[inline(always)]
    pub fn major(&self) -> MajorR {
        MajorR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Sub-classification of the type of the debug component as specified in the Arm Architecture Specification within the major classification as specified in the MAJOR field."]
    #[inline(always)]
    pub fn sub(&self) -> SubR {
        SubR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "Device Type Identifier register\n\nYou can [`read`](crate::Reg::read) this register and get [`devtype::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevtypeSpec;
impl crate::RegisterSpec for DevtypeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devtype::R`](R) reader structure"]
impl crate::Readable for DevtypeSpec {}
#[doc = "`reset()` method sets DEVTYPE to value 0x14"]
impl crate::Resettable for DevtypeSpec {
    const RESET_VALUE: u32 = 0x14;
}
