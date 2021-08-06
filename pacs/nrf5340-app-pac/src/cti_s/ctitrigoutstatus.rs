#[doc = "Register `CTITRIGOUTSTATUS` reader"]
pub struct R(crate::R<CTITRIGOUTSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTITRIGOUTSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTITRIGOUTSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTITRIGOUTSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Processor debug request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEBUGREQ_A {
    #[doc = "1: Ctitrigout 0 is active."]
    ACTIVE = 1,
    #[doc = "0: Ctitrigout 0 is inactive."]
    INACTIVE = 0,
}
impl From<DEBUGREQ_A> for bool {
    #[inline(always)]
    fn from(variant: DEBUGREQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEBUGREQ` reader - Processor debug request"]
pub struct DEBUGREQ_R(crate::FieldReader<bool, DEBUGREQ_A>);
impl DEBUGREQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        DEBUGREQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEBUGREQ_A {
        match self.bits {
            true => DEBUGREQ_A::ACTIVE,
            false => DEBUGREQ_A::INACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        **self == DEBUGREQ_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        **self == DEBUGREQ_A::INACTIVE
    }
}
impl core::ops::Deref for DEBUGREQ_R {
    type Target = crate::FieldReader<bool, DEBUGREQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Processor Restart\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPURESTART_A {
    #[doc = "1: Ctitrigout 1 is active."]
    ACTIVE = 1,
    #[doc = "0: Ctitrigout 1 is inactive."]
    INACTIVE = 0,
}
impl From<CPURESTART_A> for bool {
    #[inline(always)]
    fn from(variant: CPURESTART_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPURESTART` reader - Processor Restart"]
pub struct CPURESTART_R(crate::FieldReader<bool, CPURESTART_A>);
impl CPURESTART_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPURESTART_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPURESTART_A {
        match self.bits {
            true => CPURESTART_A::ACTIVE,
            false => CPURESTART_A::INACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        **self == CPURESTART_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        **self == CPURESTART_A::INACTIVE
    }
}
impl core::ops::Deref for CPURESTART_R {
    type Target = crate::FieldReader<bool, CPURESTART_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "N/A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNUSED0_A {
    #[doc = "1: Ctitrigout 2 is active."]
    ACTIVE = 1,
    #[doc = "0: Ctitrigout 2 is inactive."]
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
    #[doc = "1: Ctitrigout 3 is active."]
    ACTIVE = 1,
    #[doc = "0: Ctitrigout 3 is inactive."]
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
#[doc = "ETM Event Input 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETMEVTIN0_A {
    #[doc = "1: Ctitrigout 4 is active."]
    ACTIVE = 1,
    #[doc = "0: Ctitrigout 4 is inactive."]
    INACTIVE = 0,
}
impl From<ETMEVTIN0_A> for bool {
    #[inline(always)]
    fn from(variant: ETMEVTIN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETMEVTIN0` reader - ETM Event Input 0"]
pub struct ETMEVTIN0_R(crate::FieldReader<bool, ETMEVTIN0_A>);
impl ETMEVTIN0_R {
    pub(crate) fn new(bits: bool) -> Self {
        ETMEVTIN0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETMEVTIN0_A {
        match self.bits {
            true => ETMEVTIN0_A::ACTIVE,
            false => ETMEVTIN0_A::INACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        **self == ETMEVTIN0_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        **self == ETMEVTIN0_A::INACTIVE
    }
}
impl core::ops::Deref for ETMEVTIN0_R {
    type Target = crate::FieldReader<bool, ETMEVTIN0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "ETM Event Input 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETMEVTIN1_A {
    #[doc = "1: Ctitrigout 5 is active."]
    ACTIVE = 1,
    #[doc = "0: Ctitrigout 5 is inactive."]
    INACTIVE = 0,
}
impl From<ETMEVTIN1_A> for bool {
    #[inline(always)]
    fn from(variant: ETMEVTIN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETMEVTIN1` reader - ETM Event Input 1"]
pub struct ETMEVTIN1_R(crate::FieldReader<bool, ETMEVTIN1_A>);
impl ETMEVTIN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        ETMEVTIN1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETMEVTIN1_A {
        match self.bits {
            true => ETMEVTIN1_A::ACTIVE,
            false => ETMEVTIN1_A::INACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        **self == ETMEVTIN1_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        **self == ETMEVTIN1_A::INACTIVE
    }
}
impl core::ops::Deref for ETMEVTIN1_R {
    type Target = crate::FieldReader<bool, ETMEVTIN1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "ETM Event Input 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETMEVTIN2_A {
    #[doc = "1: Ctitrigout 6 is active."]
    ACTIVE = 1,
    #[doc = "0: Ctitrigout 6 is inactive."]
    INACTIVE = 0,
}
impl From<ETMEVTIN2_A> for bool {
    #[inline(always)]
    fn from(variant: ETMEVTIN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETMEVTIN2` reader - ETM Event Input 2"]
pub struct ETMEVTIN2_R(crate::FieldReader<bool, ETMEVTIN2_A>);
impl ETMEVTIN2_R {
    pub(crate) fn new(bits: bool) -> Self {
        ETMEVTIN2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETMEVTIN2_A {
        match self.bits {
            true => ETMEVTIN2_A::ACTIVE,
            false => ETMEVTIN2_A::INACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        **self == ETMEVTIN2_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        **self == ETMEVTIN2_A::INACTIVE
    }
}
impl core::ops::Deref for ETMEVTIN2_R {
    type Target = crate::FieldReader<bool, ETMEVTIN2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "ETM Event Input 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETMEVTIN3_A {
    #[doc = "1: Ctitrigout 7 is active."]
    ACTIVE = 1,
    #[doc = "0: Ctitrigout 7 is inactive."]
    INACTIVE = 0,
}
impl From<ETMEVTIN3_A> for bool {
    #[inline(always)]
    fn from(variant: ETMEVTIN3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETMEVTIN3` reader - ETM Event Input 3"]
pub struct ETMEVTIN3_R(crate::FieldReader<bool, ETMEVTIN3_A>);
impl ETMEVTIN3_R {
    pub(crate) fn new(bits: bool) -> Self {
        ETMEVTIN3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETMEVTIN3_A {
        match self.bits {
            true => ETMEVTIN3_A::ACTIVE,
            false => ETMEVTIN3_A::INACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        **self == ETMEVTIN3_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        **self == ETMEVTIN3_A::INACTIVE
    }
}
impl core::ops::Deref for ETMEVTIN3_R {
    type Target = crate::FieldReader<bool, ETMEVTIN3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Processor debug request"]
    #[inline(always)]
    pub fn debugreq(&self) -> DEBUGREQ_R {
        DEBUGREQ_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Processor Restart"]
    #[inline(always)]
    pub fn cpurestart(&self) -> CPURESTART_R {
        CPURESTART_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    pub fn unused0(&self) -> UNUSED0_R {
        UNUSED0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - N/A"]
    #[inline(always)]
    pub fn unused1(&self) -> UNUSED1_R {
        UNUSED1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ETM Event Input 0"]
    #[inline(always)]
    pub fn etmevtin0(&self) -> ETMEVTIN0_R {
        ETMEVTIN0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ETM Event Input 1"]
    #[inline(always)]
    pub fn etmevtin1(&self) -> ETMEVTIN1_R {
        ETMEVTIN1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - ETM Event Input 2"]
    #[inline(always)]
    pub fn etmevtin2(&self) -> ETMEVTIN2_R {
        ETMEVTIN2_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - ETM Event Input 3"]
    #[inline(always)]
    pub fn etmevtin3(&self) -> ETMEVTIN3_R {
        ETMEVTIN3_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
#[doc = "CTI Trigger Out Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctitrigoutstatus](index.html) module"]
pub struct CTITRIGOUTSTATUS_SPEC;
impl crate::RegisterSpec for CTITRIGOUTSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctitrigoutstatus::R](R) reader structure"]
impl crate::Readable for CTITRIGOUTSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CTITRIGOUTSTATUS to value 0"]
impl crate::Resettable for CTITRIGOUTSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
