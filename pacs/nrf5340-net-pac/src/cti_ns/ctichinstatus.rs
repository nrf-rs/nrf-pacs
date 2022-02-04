#[doc = "Register `CTICHINSTATUS` reader"]
pub struct R(crate::R<CTICHINSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTICHINSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTICHINSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTICHINSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Shows the status of the ctitrigin 0 input.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTICHINSTATUS_0_A {
    #[doc = "1: Ctichin 0 is active."]
    ACTIVE = 1,
    #[doc = "0: Ctichin 0 is inactive."]
    INACTIVE = 0,
}
impl From<CTICHINSTATUS_0_A> for bool {
    #[inline(always)]
    fn from(variant: CTICHINSTATUS_0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTICHINSTATUS_0` reader - Shows the status of the ctitrigin 0 input."]
pub struct CTICHINSTATUS_0_R(crate::FieldReader<bool, CTICHINSTATUS_0_A>);
impl CTICHINSTATUS_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CTICHINSTATUS_0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTICHINSTATUS_0_A {
        match self.bits {
            true => CTICHINSTATUS_0_A::ACTIVE,
            false => CTICHINSTATUS_0_A::INACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        **self == CTICHINSTATUS_0_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        **self == CTICHINSTATUS_0_A::INACTIVE
    }
}
impl core::ops::Deref for CTICHINSTATUS_0_R {
    type Target = crate::FieldReader<bool, CTICHINSTATUS_0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Shows the status of the ctitrigin 1 input.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTICHINSTATUS_1_A {
    #[doc = "1: Ctichin 1 is active."]
    ACTIVE = 1,
    #[doc = "0: Ctichin 1 is inactive."]
    INACTIVE = 0,
}
impl From<CTICHINSTATUS_1_A> for bool {
    #[inline(always)]
    fn from(variant: CTICHINSTATUS_1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTICHINSTATUS_1` reader - Shows the status of the ctitrigin 1 input."]
pub struct CTICHINSTATUS_1_R(crate::FieldReader<bool, CTICHINSTATUS_1_A>);
impl CTICHINSTATUS_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CTICHINSTATUS_1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTICHINSTATUS_1_A {
        match self.bits {
            true => CTICHINSTATUS_1_A::ACTIVE,
            false => CTICHINSTATUS_1_A::INACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        **self == CTICHINSTATUS_1_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        **self == CTICHINSTATUS_1_A::INACTIVE
    }
}
impl core::ops::Deref for CTICHINSTATUS_1_R {
    type Target = crate::FieldReader<bool, CTICHINSTATUS_1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Shows the status of the ctitrigin 2 input.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTICHINSTATUS_2_A {
    #[doc = "1: Ctichin 2 is active."]
    ACTIVE = 1,
    #[doc = "0: Ctichin 2 is inactive."]
    INACTIVE = 0,
}
impl From<CTICHINSTATUS_2_A> for bool {
    #[inline(always)]
    fn from(variant: CTICHINSTATUS_2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTICHINSTATUS_2` reader - Shows the status of the ctitrigin 2 input."]
pub struct CTICHINSTATUS_2_R(crate::FieldReader<bool, CTICHINSTATUS_2_A>);
impl CTICHINSTATUS_2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CTICHINSTATUS_2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTICHINSTATUS_2_A {
        match self.bits {
            true => CTICHINSTATUS_2_A::ACTIVE,
            false => CTICHINSTATUS_2_A::INACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        **self == CTICHINSTATUS_2_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        **self == CTICHINSTATUS_2_A::INACTIVE
    }
}
impl core::ops::Deref for CTICHINSTATUS_2_R {
    type Target = crate::FieldReader<bool, CTICHINSTATUS_2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Shows the status of the ctitrigin 3 input.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTICHINSTATUS_3_A {
    #[doc = "1: Ctichin 3 is active."]
    ACTIVE = 1,
    #[doc = "0: Ctichin 3 is inactive."]
    INACTIVE = 0,
}
impl From<CTICHINSTATUS_3_A> for bool {
    #[inline(always)]
    fn from(variant: CTICHINSTATUS_3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTICHINSTATUS_3` reader - Shows the status of the ctitrigin 3 input."]
pub struct CTICHINSTATUS_3_R(crate::FieldReader<bool, CTICHINSTATUS_3_A>);
impl CTICHINSTATUS_3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CTICHINSTATUS_3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTICHINSTATUS_3_A {
        match self.bits {
            true => CTICHINSTATUS_3_A::ACTIVE,
            false => CTICHINSTATUS_3_A::INACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        **self == CTICHINSTATUS_3_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        **self == CTICHINSTATUS_3_A::INACTIVE
    }
}
impl core::ops::Deref for CTICHINSTATUS_3_R {
    type Target = crate::FieldReader<bool, CTICHINSTATUS_3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Shows the status of the ctitrigin 0 input."]
    #[inline(always)]
    pub fn ctichinstatus_0(&self) -> CTICHINSTATUS_0_R {
        CTICHINSTATUS_0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Shows the status of the ctitrigin 1 input."]
    #[inline(always)]
    pub fn ctichinstatus_1(&self) -> CTICHINSTATUS_1_R {
        CTICHINSTATUS_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Shows the status of the ctitrigin 2 input."]
    #[inline(always)]
    pub fn ctichinstatus_2(&self) -> CTICHINSTATUS_2_R {
        CTICHINSTATUS_2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Shows the status of the ctitrigin 3 input."]
    #[inline(always)]
    pub fn ctichinstatus_3(&self) -> CTICHINSTATUS_3_R {
        CTICHINSTATUS_3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
#[doc = "CTI Channel In Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctichinstatus](index.html) module"]
pub struct CTICHINSTATUS_SPEC;
impl crate::RegisterSpec for CTICHINSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctichinstatus::R](R) reader structure"]
impl crate::Readable for CTICHINSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CTICHINSTATUS to value 0"]
impl crate::Resettable for CTICHINSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
