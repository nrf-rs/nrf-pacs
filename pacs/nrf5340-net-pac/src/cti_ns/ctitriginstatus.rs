#[doc = "Register `CTITRIGINSTATUS` reader"]
pub struct R(crate::R<CTITRIGINSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTITRIGINSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTITRIGINSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTITRIGINSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Processor Halted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPUHALTED_A {
    #[doc = "1: Ctitrigin 0 is active."]
    ACTIVE = 1,
    #[doc = "0: Ctitrigin 0 is inactive."]
    INACTIVE = 0,
}
impl From<CPUHALTED_A> for bool {
    #[inline(always)]
    fn from(variant: CPUHALTED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPUHALTED` reader - Processor Halted"]
pub struct CPUHALTED_R(crate::FieldReader<bool, CPUHALTED_A>);
impl CPUHALTED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CPUHALTED_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPUHALTED_A {
        match self.bits {
            true => CPUHALTED_A::ACTIVE,
            false => CPUHALTED_A::INACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        **self == CPUHALTED_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        **self == CPUHALTED_A::INACTIVE
    }
}
impl core::ops::Deref for CPUHALTED_R {
    type Target = crate::FieldReader<bool, CPUHALTED_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "DWT Comparator Output 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DWTCOMPOUT0_A {
    #[doc = "1: Ctitrigin 1 is active."]
    ACTIVE = 1,
    #[doc = "0: Ctitrigin 1 is inactive."]
    INACTIVE = 0,
}
impl From<DWTCOMPOUT0_A> for bool {
    #[inline(always)]
    fn from(variant: DWTCOMPOUT0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DWTCOMPOUT0` reader - DWT Comparator Output 0"]
pub struct DWTCOMPOUT0_R(crate::FieldReader<bool, DWTCOMPOUT0_A>);
impl DWTCOMPOUT0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DWTCOMPOUT0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DWTCOMPOUT0_A {
        match self.bits {
            true => DWTCOMPOUT0_A::ACTIVE,
            false => DWTCOMPOUT0_A::INACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        **self == DWTCOMPOUT0_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        **self == DWTCOMPOUT0_A::INACTIVE
    }
}
impl core::ops::Deref for DWTCOMPOUT0_R {
    type Target = crate::FieldReader<bool, DWTCOMPOUT0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "DWT Comparator Output 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DWTCOMPOUT1_A {
    #[doc = "1: Ctitrigin 2 is active."]
    ACTIVE = 1,
    #[doc = "0: Ctitrigin 2 is inactive."]
    INACTIVE = 0,
}
impl From<DWTCOMPOUT1_A> for bool {
    #[inline(always)]
    fn from(variant: DWTCOMPOUT1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DWTCOMPOUT1` reader - DWT Comparator Output 1"]
pub struct DWTCOMPOUT1_R(crate::FieldReader<bool, DWTCOMPOUT1_A>);
impl DWTCOMPOUT1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DWTCOMPOUT1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DWTCOMPOUT1_A {
        match self.bits {
            true => DWTCOMPOUT1_A::ACTIVE,
            false => DWTCOMPOUT1_A::INACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        **self == DWTCOMPOUT1_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        **self == DWTCOMPOUT1_A::INACTIVE
    }
}
impl core::ops::Deref for DWTCOMPOUT1_R {
    type Target = crate::FieldReader<bool, DWTCOMPOUT1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "DWT Comparator Output 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DWTCOMPOUT2_A {
    #[doc = "1: Ctitrigin 3 is active."]
    ACTIVE = 1,
    #[doc = "0: Ctitrigin 3 is inactive."]
    INACTIVE = 0,
}
impl From<DWTCOMPOUT2_A> for bool {
    #[inline(always)]
    fn from(variant: DWTCOMPOUT2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DWTCOMPOUT2` reader - DWT Comparator Output 2"]
pub struct DWTCOMPOUT2_R(crate::FieldReader<bool, DWTCOMPOUT2_A>);
impl DWTCOMPOUT2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DWTCOMPOUT2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DWTCOMPOUT2_A {
        match self.bits {
            true => DWTCOMPOUT2_A::ACTIVE,
            false => DWTCOMPOUT2_A::INACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        **self == DWTCOMPOUT2_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        **self == DWTCOMPOUT2_A::INACTIVE
    }
}
impl core::ops::Deref for DWTCOMPOUT2_R {
    type Target = crate::FieldReader<bool, DWTCOMPOUT2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "N/A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNUSED0_A {
    #[doc = "1: Ctitrigin 4 is active."]
    ACTIVE = 1,
    #[doc = "0: Ctitrigin 4 is inactive."]
    INACTIVE = 0,
}
impl From<UNUSED0_A> for bool {
    #[inline(always)]
    fn from(variant: UNUSED0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UNUSED0` reader - N/A"]
pub struct UNUSED0_R(crate::FieldReader<bool, UNUSED0_A>);
impl UNUSED0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UNUSED0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UNUSED0_A {
        match self.bits {
            true => UNUSED0_A::ACTIVE,
            false => UNUSED0_A::INACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        **self == UNUSED0_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        **self == UNUSED0_A::INACTIVE
    }
}
impl core::ops::Deref for UNUSED0_R {
    type Target = crate::FieldReader<bool, UNUSED0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "N/A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNUSED1_A {
    #[doc = "1: Ctitrigin 5 is active."]
    ACTIVE = 1,
    #[doc = "0: Ctitrigin 5 is inactive."]
    INACTIVE = 0,
}
impl From<UNUSED1_A> for bool {
    #[inline(always)]
    fn from(variant: UNUSED1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UNUSED1` reader - N/A"]
pub struct UNUSED1_R(crate::FieldReader<bool, UNUSED1_A>);
impl UNUSED1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UNUSED1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UNUSED1_A {
        match self.bits {
            true => UNUSED1_A::ACTIVE,
            false => UNUSED1_A::INACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        **self == UNUSED1_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        **self == UNUSED1_A::INACTIVE
    }
}
impl core::ops::Deref for UNUSED1_R {
    type Target = crate::FieldReader<bool, UNUSED1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "N/A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNUSED2_A {
    #[doc = "1: Ctitrigin 6 is active."]
    ACTIVE = 1,
    #[doc = "0: Ctitrigin 6 is inactive."]
    INACTIVE = 0,
}
impl From<UNUSED2_A> for bool {
    #[inline(always)]
    fn from(variant: UNUSED2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UNUSED2` reader - N/A"]
pub struct UNUSED2_R(crate::FieldReader<bool, UNUSED2_A>);
impl UNUSED2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UNUSED2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UNUSED2_A {
        match self.bits {
            true => UNUSED2_A::ACTIVE,
            false => UNUSED2_A::INACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        **self == UNUSED2_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        **self == UNUSED2_A::INACTIVE
    }
}
impl core::ops::Deref for UNUSED2_R {
    type Target = crate::FieldReader<bool, UNUSED2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "N/A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNUSED3_A {
    #[doc = "1: Ctitrigin 7 is active."]
    ACTIVE = 1,
    #[doc = "0: Ctitrigin 7 is inactive."]
    INACTIVE = 0,
}
impl From<UNUSED3_A> for bool {
    #[inline(always)]
    fn from(variant: UNUSED3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UNUSED3` reader - N/A"]
pub struct UNUSED3_R(crate::FieldReader<bool, UNUSED3_A>);
impl UNUSED3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UNUSED3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UNUSED3_A {
        match self.bits {
            true => UNUSED3_A::ACTIVE,
            false => UNUSED3_A::INACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        **self == UNUSED3_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        **self == UNUSED3_A::INACTIVE
    }
}
impl core::ops::Deref for UNUSED3_R {
    type Target = crate::FieldReader<bool, UNUSED3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Processor Halted"]
    #[inline(always)]
    pub fn cpuhalted(&self) -> CPUHALTED_R {
        CPUHALTED_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DWT Comparator Output 0"]
    #[inline(always)]
    pub fn dwtcompout0(&self) -> DWTCOMPOUT0_R {
        DWTCOMPOUT0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DWT Comparator Output 1"]
    #[inline(always)]
    pub fn dwtcompout1(&self) -> DWTCOMPOUT1_R {
        DWTCOMPOUT1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DWT Comparator Output 2"]
    #[inline(always)]
    pub fn dwtcompout2(&self) -> DWTCOMPOUT2_R {
        DWTCOMPOUT2_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - N/A"]
    #[inline(always)]
    pub fn unused0(&self) -> UNUSED0_R {
        UNUSED0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - N/A"]
    #[inline(always)]
    pub fn unused1(&self) -> UNUSED1_R {
        UNUSED1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - N/A"]
    #[inline(always)]
    pub fn unused2(&self) -> UNUSED2_R {
        UNUSED2_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - N/A"]
    #[inline(always)]
    pub fn unused3(&self) -> UNUSED3_R {
        UNUSED3_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
#[doc = "CTI Trigger In Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctitriginstatus](index.html) module"]
pub struct CTITRIGINSTATUS_SPEC;
impl crate::RegisterSpec for CTITRIGINSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctitriginstatus::R](R) reader structure"]
impl crate::Readable for CTITRIGINSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CTITRIGINSTATUS to value 0"]
impl crate::Resettable for CTITRIGINSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
