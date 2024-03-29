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
#[doc = "Field `UICRAPPROTECT` reader - Status bit for UICR part of access port protection at last reset."]
pub type UICRAPPROTECT_R = crate::BitReader<UICRAPPROTECT_A>;
#[doc = "Status bit for UICR part of access port protection at last reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UICRAPPROTECT_A {
    #[doc = "0: APPROTECT was enabled in UICR"]
    ENABLED = 0,
    #[doc = "1: APPROTECT wasdisabled in UICR"]
    DISABLED = 1,
}
impl From<UICRAPPROTECT_A> for bool {
    #[inline(always)]
    fn from(variant: UICRAPPROTECT_A) -> Self {
        variant as u8 != 0
    }
}
impl UICRAPPROTECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UICRAPPROTECT_A {
        match self.bits {
            false => UICRAPPROTECT_A::ENABLED,
            true => UICRAPPROTECT_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UICRAPPROTECT_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UICRAPPROTECT_A::DISABLED
    }
}
#[doc = "Field `UICRSECUREAPPROTECT` reader - Status bit for UICR part of secure access port protection at last reset."]
pub type UICRSECUREAPPROTECT_R = crate::BitReader<UICRSECUREAPPROTECT_A>;
#[doc = "Status bit for UICR part of secure access port protection at last reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UICRSECUREAPPROTECT_A {
    #[doc = "0: SECUREAPPROTECT was enabled in UICR"]
    ENABLED = 0,
    #[doc = "1: SECUREAPPROTECT was disabled in UICR"]
    DISABLED = 1,
}
impl From<UICRSECUREAPPROTECT_A> for bool {
    #[inline(always)]
    fn from(variant: UICRSECUREAPPROTECT_A) -> Self {
        variant as u8 != 0
    }
}
impl UICRSECUREAPPROTECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UICRSECUREAPPROTECT_A {
        match self.bits {
            false => UICRSECUREAPPROTECT_A::ENABLED,
            true => UICRSECUREAPPROTECT_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UICRSECUREAPPROTECT_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UICRSECUREAPPROTECT_A::DISABLED
    }
}
#[doc = "Field `DBGIFACEMODE` reader - Status bit for device debug interface mode"]
pub type DBGIFACEMODE_R = crate::BitReader<DBGIFACEMODE_A>;
#[doc = "Status bit for device debug interface mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBGIFACEMODE_A {
    #[doc = "0: No debugger attached"]
    DISABLED = 0,
    #[doc = "1: Debugger is attached and device is in debug interface mode"]
    ENABLED = 1,
}
impl From<DBGIFACEMODE_A> for bool {
    #[inline(always)]
    fn from(variant: DBGIFACEMODE_A) -> Self {
        variant as u8 != 0
    }
}
impl DBGIFACEMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBGIFACEMODE_A {
        match self.bits {
            false => DBGIFACEMODE_A::DISABLED,
            true => DBGIFACEMODE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DBGIFACEMODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DBGIFACEMODE_A::ENABLED
    }
}
impl R {
    #[doc = "Bit 0 - Status bit for UICR part of access port protection at last reset."]
    #[inline(always)]
    pub fn uicrapprotect(&self) -> UICRAPPROTECT_R {
        UICRAPPROTECT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Status bit for UICR part of secure access port protection at last reset."]
    #[inline(always)]
    pub fn uicrsecureapprotect(&self) -> UICRSECUREAPPROTECT_R {
        UICRSECUREAPPROTECT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Status bit for device debug interface mode"]
    #[inline(always)]
    pub fn dbgifacemode(&self) -> DBGIFACEMODE_R {
        DBGIFACEMODE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Status bits for CTRL-AP peripheral.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
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
