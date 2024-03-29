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
#[doc = "Field `DEBUGREQ` reader - Processor debug request"]
pub type DEBUGREQ_R = crate::BitReader<DEBUGREQ_A>;
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
impl DEBUGREQ_R {
    #[doc = "Get enumerated values variant"]
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
        *self == DEBUGREQ_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == DEBUGREQ_A::INACTIVE
    }
}
#[doc = "Field `CPURESTART` reader - Processor Restart"]
pub type CPURESTART_R = crate::BitReader<CPURESTART_A>;
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
impl CPURESTART_R {
    #[doc = "Get enumerated values variant"]
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
        *self == CPURESTART_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == CPURESTART_A::INACTIVE
    }
}
#[doc = "Field `UNUSED0` reader - N/A"]
pub type UNUSED0_R = crate::BitReader<UNUSED0_A>;
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
impl UNUSED0_R {
    #[doc = "Get enumerated values variant"]
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
        *self == UNUSED0_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == UNUSED0_A::INACTIVE
    }
}
#[doc = "Field `UNUSED1` reader - N/A"]
pub type UNUSED1_R = crate::BitReader<UNUSED1_A>;
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
impl UNUSED1_R {
    #[doc = "Get enumerated values variant"]
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
        *self == UNUSED1_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == UNUSED1_A::INACTIVE
    }
}
#[doc = "Field `UNUSED2` reader - N/A"]
pub type UNUSED2_R = crate::BitReader<UNUSED2_A>;
#[doc = "N/A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNUSED2_A {
    #[doc = "1: Ctitrigout 4 is active."]
    ACTIVE = 1,
    #[doc = "0: Ctitrigout 4 is inactive."]
    INACTIVE = 0,
}
impl From<UNUSED2_A> for bool {
    #[inline(always)]
    fn from(variant: UNUSED2_A) -> Self {
        variant as u8 != 0
    }
}
impl UNUSED2_R {
    #[doc = "Get enumerated values variant"]
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
        *self == UNUSED2_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == UNUSED2_A::INACTIVE
    }
}
#[doc = "Field `UNUSED3` reader - N/A"]
pub type UNUSED3_R = crate::BitReader<UNUSED3_A>;
#[doc = "N/A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNUSED3_A {
    #[doc = "1: Ctitrigout 5 is active."]
    ACTIVE = 1,
    #[doc = "0: Ctitrigout 5 is inactive."]
    INACTIVE = 0,
}
impl From<UNUSED3_A> for bool {
    #[inline(always)]
    fn from(variant: UNUSED3_A) -> Self {
        variant as u8 != 0
    }
}
impl UNUSED3_R {
    #[doc = "Get enumerated values variant"]
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
        *self == UNUSED3_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == UNUSED3_A::INACTIVE
    }
}
#[doc = "Field `UNUSED4` reader - N/A"]
pub type UNUSED4_R = crate::BitReader<UNUSED4_A>;
#[doc = "N/A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNUSED4_A {
    #[doc = "1: Ctitrigout 6 is active."]
    ACTIVE = 1,
    #[doc = "0: Ctitrigout 6 is inactive."]
    INACTIVE = 0,
}
impl From<UNUSED4_A> for bool {
    #[inline(always)]
    fn from(variant: UNUSED4_A) -> Self {
        variant as u8 != 0
    }
}
impl UNUSED4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UNUSED4_A {
        match self.bits {
            true => UNUSED4_A::ACTIVE,
            false => UNUSED4_A::INACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == UNUSED4_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == UNUSED4_A::INACTIVE
    }
}
#[doc = "Field `UNUSED5` reader - N/A"]
pub type UNUSED5_R = crate::BitReader<UNUSED5_A>;
#[doc = "N/A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNUSED5_A {
    #[doc = "1: Ctitrigout 7 is active."]
    ACTIVE = 1,
    #[doc = "0: Ctitrigout 7 is inactive."]
    INACTIVE = 0,
}
impl From<UNUSED5_A> for bool {
    #[inline(always)]
    fn from(variant: UNUSED5_A) -> Self {
        variant as u8 != 0
    }
}
impl UNUSED5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UNUSED5_A {
        match self.bits {
            true => UNUSED5_A::ACTIVE,
            false => UNUSED5_A::INACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == UNUSED5_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == UNUSED5_A::INACTIVE
    }
}
impl R {
    #[doc = "Bit 0 - Processor debug request"]
    #[inline(always)]
    pub fn debugreq(&self) -> DEBUGREQ_R {
        DEBUGREQ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Processor Restart"]
    #[inline(always)]
    pub fn cpurestart(&self) -> CPURESTART_R {
        CPURESTART_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    pub fn unused0(&self) -> UNUSED0_R {
        UNUSED0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - N/A"]
    #[inline(always)]
    pub fn unused1(&self) -> UNUSED1_R {
        UNUSED1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - N/A"]
    #[inline(always)]
    pub fn unused2(&self) -> UNUSED2_R {
        UNUSED2_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - N/A"]
    #[inline(always)]
    pub fn unused3(&self) -> UNUSED3_R {
        UNUSED3_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - N/A"]
    #[inline(always)]
    pub fn unused4(&self) -> UNUSED4_R {
        UNUSED4_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - N/A"]
    #[inline(always)]
    pub fn unused5(&self) -> UNUSED5_R {
        UNUSED5_R::new(((self.bits >> 7) & 1) != 0)
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
