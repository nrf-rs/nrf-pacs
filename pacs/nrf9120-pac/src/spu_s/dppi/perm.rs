#[doc = "Register `PERM` reader"]
pub struct R(crate::R<PERM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERM` writer"]
pub struct W(crate::W<PERM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PERM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHANNEL0` reader - Select secure attribute."]
pub type CHANNEL0_R = crate::BitReader<CHANNEL0_A>;
#[doc = "Select secure attribute.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL0_A {
    #[doc = "1: Channel0 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Channel0 has its non-secure attribute set"]
    NON_SECURE = 0,
}
impl From<CHANNEL0_A> for bool {
    #[inline(always)]
    fn from(variant: CHANNEL0_A) -> Self {
        variant as u8 != 0
    }
}
impl CHANNEL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHANNEL0_A {
        match self.bits {
            true => CHANNEL0_A::SECURE,
            false => CHANNEL0_A::NON_SECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == CHANNEL0_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NON_SECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == CHANNEL0_A::NON_SECURE
    }
}
#[doc = "Field `CHANNEL0` writer - Select secure attribute."]
pub type CHANNEL0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERM_SPEC, CHANNEL0_A, O>;
impl<'a, const O: u8> CHANNEL0_W<'a, O> {
    #[doc = "Channel0 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL0_A::SECURE)
    }
    #[doc = "Channel0 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(CHANNEL0_A::NON_SECURE)
    }
}
#[doc = "Field `CHANNEL1` reader - Select secure attribute."]
pub type CHANNEL1_R = crate::BitReader<CHANNEL1_A>;
#[doc = "Select secure attribute.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL1_A {
    #[doc = "1: Channel1 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Channel1 has its non-secure attribute set"]
    NON_SECURE = 0,
}
impl From<CHANNEL1_A> for bool {
    #[inline(always)]
    fn from(variant: CHANNEL1_A) -> Self {
        variant as u8 != 0
    }
}
impl CHANNEL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHANNEL1_A {
        match self.bits {
            true => CHANNEL1_A::SECURE,
            false => CHANNEL1_A::NON_SECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == CHANNEL1_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NON_SECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == CHANNEL1_A::NON_SECURE
    }
}
#[doc = "Field `CHANNEL1` writer - Select secure attribute."]
pub type CHANNEL1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERM_SPEC, CHANNEL1_A, O>;
impl<'a, const O: u8> CHANNEL1_W<'a, O> {
    #[doc = "Channel1 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL1_A::SECURE)
    }
    #[doc = "Channel1 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(CHANNEL1_A::NON_SECURE)
    }
}
#[doc = "Field `CHANNEL2` reader - Select secure attribute."]
pub type CHANNEL2_R = crate::BitReader<CHANNEL2_A>;
#[doc = "Select secure attribute.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL2_A {
    #[doc = "1: Channel2 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Channel2 has its non-secure attribute set"]
    NON_SECURE = 0,
}
impl From<CHANNEL2_A> for bool {
    #[inline(always)]
    fn from(variant: CHANNEL2_A) -> Self {
        variant as u8 != 0
    }
}
impl CHANNEL2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHANNEL2_A {
        match self.bits {
            true => CHANNEL2_A::SECURE,
            false => CHANNEL2_A::NON_SECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == CHANNEL2_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NON_SECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == CHANNEL2_A::NON_SECURE
    }
}
#[doc = "Field `CHANNEL2` writer - Select secure attribute."]
pub type CHANNEL2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERM_SPEC, CHANNEL2_A, O>;
impl<'a, const O: u8> CHANNEL2_W<'a, O> {
    #[doc = "Channel2 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL2_A::SECURE)
    }
    #[doc = "Channel2 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(CHANNEL2_A::NON_SECURE)
    }
}
#[doc = "Field `CHANNEL3` reader - Select secure attribute."]
pub type CHANNEL3_R = crate::BitReader<CHANNEL3_A>;
#[doc = "Select secure attribute.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL3_A {
    #[doc = "1: Channel3 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Channel3 has its non-secure attribute set"]
    NON_SECURE = 0,
}
impl From<CHANNEL3_A> for bool {
    #[inline(always)]
    fn from(variant: CHANNEL3_A) -> Self {
        variant as u8 != 0
    }
}
impl CHANNEL3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHANNEL3_A {
        match self.bits {
            true => CHANNEL3_A::SECURE,
            false => CHANNEL3_A::NON_SECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == CHANNEL3_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NON_SECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == CHANNEL3_A::NON_SECURE
    }
}
#[doc = "Field `CHANNEL3` writer - Select secure attribute."]
pub type CHANNEL3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERM_SPEC, CHANNEL3_A, O>;
impl<'a, const O: u8> CHANNEL3_W<'a, O> {
    #[doc = "Channel3 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL3_A::SECURE)
    }
    #[doc = "Channel3 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(CHANNEL3_A::NON_SECURE)
    }
}
#[doc = "Field `CHANNEL4` reader - Select secure attribute."]
pub type CHANNEL4_R = crate::BitReader<CHANNEL4_A>;
#[doc = "Select secure attribute.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL4_A {
    #[doc = "1: Channel4 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Channel4 has its non-secure attribute set"]
    NON_SECURE = 0,
}
impl From<CHANNEL4_A> for bool {
    #[inline(always)]
    fn from(variant: CHANNEL4_A) -> Self {
        variant as u8 != 0
    }
}
impl CHANNEL4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHANNEL4_A {
        match self.bits {
            true => CHANNEL4_A::SECURE,
            false => CHANNEL4_A::NON_SECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == CHANNEL4_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NON_SECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == CHANNEL4_A::NON_SECURE
    }
}
#[doc = "Field `CHANNEL4` writer - Select secure attribute."]
pub type CHANNEL4_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERM_SPEC, CHANNEL4_A, O>;
impl<'a, const O: u8> CHANNEL4_W<'a, O> {
    #[doc = "Channel4 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL4_A::SECURE)
    }
    #[doc = "Channel4 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(CHANNEL4_A::NON_SECURE)
    }
}
#[doc = "Field `CHANNEL5` reader - Select secure attribute."]
pub type CHANNEL5_R = crate::BitReader<CHANNEL5_A>;
#[doc = "Select secure attribute.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL5_A {
    #[doc = "1: Channel5 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Channel5 has its non-secure attribute set"]
    NON_SECURE = 0,
}
impl From<CHANNEL5_A> for bool {
    #[inline(always)]
    fn from(variant: CHANNEL5_A) -> Self {
        variant as u8 != 0
    }
}
impl CHANNEL5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHANNEL5_A {
        match self.bits {
            true => CHANNEL5_A::SECURE,
            false => CHANNEL5_A::NON_SECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == CHANNEL5_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NON_SECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == CHANNEL5_A::NON_SECURE
    }
}
#[doc = "Field `CHANNEL5` writer - Select secure attribute."]
pub type CHANNEL5_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERM_SPEC, CHANNEL5_A, O>;
impl<'a, const O: u8> CHANNEL5_W<'a, O> {
    #[doc = "Channel5 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL5_A::SECURE)
    }
    #[doc = "Channel5 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(CHANNEL5_A::NON_SECURE)
    }
}
#[doc = "Field `CHANNEL6` reader - Select secure attribute."]
pub type CHANNEL6_R = crate::BitReader<CHANNEL6_A>;
#[doc = "Select secure attribute.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL6_A {
    #[doc = "1: Channel6 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Channel6 has its non-secure attribute set"]
    NON_SECURE = 0,
}
impl From<CHANNEL6_A> for bool {
    #[inline(always)]
    fn from(variant: CHANNEL6_A) -> Self {
        variant as u8 != 0
    }
}
impl CHANNEL6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHANNEL6_A {
        match self.bits {
            true => CHANNEL6_A::SECURE,
            false => CHANNEL6_A::NON_SECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == CHANNEL6_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NON_SECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == CHANNEL6_A::NON_SECURE
    }
}
#[doc = "Field `CHANNEL6` writer - Select secure attribute."]
pub type CHANNEL6_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERM_SPEC, CHANNEL6_A, O>;
impl<'a, const O: u8> CHANNEL6_W<'a, O> {
    #[doc = "Channel6 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL6_A::SECURE)
    }
    #[doc = "Channel6 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(CHANNEL6_A::NON_SECURE)
    }
}
#[doc = "Field `CHANNEL7` reader - Select secure attribute."]
pub type CHANNEL7_R = crate::BitReader<CHANNEL7_A>;
#[doc = "Select secure attribute.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL7_A {
    #[doc = "1: Channel7 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Channel7 has its non-secure attribute set"]
    NON_SECURE = 0,
}
impl From<CHANNEL7_A> for bool {
    #[inline(always)]
    fn from(variant: CHANNEL7_A) -> Self {
        variant as u8 != 0
    }
}
impl CHANNEL7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHANNEL7_A {
        match self.bits {
            true => CHANNEL7_A::SECURE,
            false => CHANNEL7_A::NON_SECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == CHANNEL7_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NON_SECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == CHANNEL7_A::NON_SECURE
    }
}
#[doc = "Field `CHANNEL7` writer - Select secure attribute."]
pub type CHANNEL7_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERM_SPEC, CHANNEL7_A, O>;
impl<'a, const O: u8> CHANNEL7_W<'a, O> {
    #[doc = "Channel7 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL7_A::SECURE)
    }
    #[doc = "Channel7 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(CHANNEL7_A::NON_SECURE)
    }
}
#[doc = "Field `CHANNEL8` reader - Select secure attribute."]
pub type CHANNEL8_R = crate::BitReader<CHANNEL8_A>;
#[doc = "Select secure attribute.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL8_A {
    #[doc = "1: Channel8 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Channel8 has its non-secure attribute set"]
    NON_SECURE = 0,
}
impl From<CHANNEL8_A> for bool {
    #[inline(always)]
    fn from(variant: CHANNEL8_A) -> Self {
        variant as u8 != 0
    }
}
impl CHANNEL8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHANNEL8_A {
        match self.bits {
            true => CHANNEL8_A::SECURE,
            false => CHANNEL8_A::NON_SECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == CHANNEL8_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NON_SECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == CHANNEL8_A::NON_SECURE
    }
}
#[doc = "Field `CHANNEL8` writer - Select secure attribute."]
pub type CHANNEL8_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERM_SPEC, CHANNEL8_A, O>;
impl<'a, const O: u8> CHANNEL8_W<'a, O> {
    #[doc = "Channel8 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL8_A::SECURE)
    }
    #[doc = "Channel8 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(CHANNEL8_A::NON_SECURE)
    }
}
#[doc = "Field `CHANNEL9` reader - Select secure attribute."]
pub type CHANNEL9_R = crate::BitReader<CHANNEL9_A>;
#[doc = "Select secure attribute.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL9_A {
    #[doc = "1: Channel9 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Channel9 has its non-secure attribute set"]
    NON_SECURE = 0,
}
impl From<CHANNEL9_A> for bool {
    #[inline(always)]
    fn from(variant: CHANNEL9_A) -> Self {
        variant as u8 != 0
    }
}
impl CHANNEL9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHANNEL9_A {
        match self.bits {
            true => CHANNEL9_A::SECURE,
            false => CHANNEL9_A::NON_SECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == CHANNEL9_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NON_SECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == CHANNEL9_A::NON_SECURE
    }
}
#[doc = "Field `CHANNEL9` writer - Select secure attribute."]
pub type CHANNEL9_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERM_SPEC, CHANNEL9_A, O>;
impl<'a, const O: u8> CHANNEL9_W<'a, O> {
    #[doc = "Channel9 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL9_A::SECURE)
    }
    #[doc = "Channel9 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(CHANNEL9_A::NON_SECURE)
    }
}
#[doc = "Field `CHANNEL10` reader - Select secure attribute."]
pub type CHANNEL10_R = crate::BitReader<CHANNEL10_A>;
#[doc = "Select secure attribute.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL10_A {
    #[doc = "1: Channel10 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Channel10 has its non-secure attribute set"]
    NON_SECURE = 0,
}
impl From<CHANNEL10_A> for bool {
    #[inline(always)]
    fn from(variant: CHANNEL10_A) -> Self {
        variant as u8 != 0
    }
}
impl CHANNEL10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHANNEL10_A {
        match self.bits {
            true => CHANNEL10_A::SECURE,
            false => CHANNEL10_A::NON_SECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == CHANNEL10_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NON_SECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == CHANNEL10_A::NON_SECURE
    }
}
#[doc = "Field `CHANNEL10` writer - Select secure attribute."]
pub type CHANNEL10_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERM_SPEC, CHANNEL10_A, O>;
impl<'a, const O: u8> CHANNEL10_W<'a, O> {
    #[doc = "Channel10 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL10_A::SECURE)
    }
    #[doc = "Channel10 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(CHANNEL10_A::NON_SECURE)
    }
}
#[doc = "Field `CHANNEL11` reader - Select secure attribute."]
pub type CHANNEL11_R = crate::BitReader<CHANNEL11_A>;
#[doc = "Select secure attribute.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL11_A {
    #[doc = "1: Channel11 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Channel11 has its non-secure attribute set"]
    NON_SECURE = 0,
}
impl From<CHANNEL11_A> for bool {
    #[inline(always)]
    fn from(variant: CHANNEL11_A) -> Self {
        variant as u8 != 0
    }
}
impl CHANNEL11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHANNEL11_A {
        match self.bits {
            true => CHANNEL11_A::SECURE,
            false => CHANNEL11_A::NON_SECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == CHANNEL11_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NON_SECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == CHANNEL11_A::NON_SECURE
    }
}
#[doc = "Field `CHANNEL11` writer - Select secure attribute."]
pub type CHANNEL11_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERM_SPEC, CHANNEL11_A, O>;
impl<'a, const O: u8> CHANNEL11_W<'a, O> {
    #[doc = "Channel11 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL11_A::SECURE)
    }
    #[doc = "Channel11 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(CHANNEL11_A::NON_SECURE)
    }
}
#[doc = "Field `CHANNEL12` reader - Select secure attribute."]
pub type CHANNEL12_R = crate::BitReader<CHANNEL12_A>;
#[doc = "Select secure attribute.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL12_A {
    #[doc = "1: Channel12 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Channel12 has its non-secure attribute set"]
    NON_SECURE = 0,
}
impl From<CHANNEL12_A> for bool {
    #[inline(always)]
    fn from(variant: CHANNEL12_A) -> Self {
        variant as u8 != 0
    }
}
impl CHANNEL12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHANNEL12_A {
        match self.bits {
            true => CHANNEL12_A::SECURE,
            false => CHANNEL12_A::NON_SECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == CHANNEL12_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NON_SECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == CHANNEL12_A::NON_SECURE
    }
}
#[doc = "Field `CHANNEL12` writer - Select secure attribute."]
pub type CHANNEL12_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERM_SPEC, CHANNEL12_A, O>;
impl<'a, const O: u8> CHANNEL12_W<'a, O> {
    #[doc = "Channel12 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL12_A::SECURE)
    }
    #[doc = "Channel12 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(CHANNEL12_A::NON_SECURE)
    }
}
#[doc = "Field `CHANNEL13` reader - Select secure attribute."]
pub type CHANNEL13_R = crate::BitReader<CHANNEL13_A>;
#[doc = "Select secure attribute.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL13_A {
    #[doc = "1: Channel13 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Channel13 has its non-secure attribute set"]
    NON_SECURE = 0,
}
impl From<CHANNEL13_A> for bool {
    #[inline(always)]
    fn from(variant: CHANNEL13_A) -> Self {
        variant as u8 != 0
    }
}
impl CHANNEL13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHANNEL13_A {
        match self.bits {
            true => CHANNEL13_A::SECURE,
            false => CHANNEL13_A::NON_SECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == CHANNEL13_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NON_SECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == CHANNEL13_A::NON_SECURE
    }
}
#[doc = "Field `CHANNEL13` writer - Select secure attribute."]
pub type CHANNEL13_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERM_SPEC, CHANNEL13_A, O>;
impl<'a, const O: u8> CHANNEL13_W<'a, O> {
    #[doc = "Channel13 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL13_A::SECURE)
    }
    #[doc = "Channel13 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(CHANNEL13_A::NON_SECURE)
    }
}
#[doc = "Field `CHANNEL14` reader - Select secure attribute."]
pub type CHANNEL14_R = crate::BitReader<CHANNEL14_A>;
#[doc = "Select secure attribute.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL14_A {
    #[doc = "1: Channel14 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Channel14 has its non-secure attribute set"]
    NON_SECURE = 0,
}
impl From<CHANNEL14_A> for bool {
    #[inline(always)]
    fn from(variant: CHANNEL14_A) -> Self {
        variant as u8 != 0
    }
}
impl CHANNEL14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHANNEL14_A {
        match self.bits {
            true => CHANNEL14_A::SECURE,
            false => CHANNEL14_A::NON_SECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == CHANNEL14_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NON_SECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == CHANNEL14_A::NON_SECURE
    }
}
#[doc = "Field `CHANNEL14` writer - Select secure attribute."]
pub type CHANNEL14_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERM_SPEC, CHANNEL14_A, O>;
impl<'a, const O: u8> CHANNEL14_W<'a, O> {
    #[doc = "Channel14 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL14_A::SECURE)
    }
    #[doc = "Channel14 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(CHANNEL14_A::NON_SECURE)
    }
}
#[doc = "Field `CHANNEL15` reader - Select secure attribute."]
pub type CHANNEL15_R = crate::BitReader<CHANNEL15_A>;
#[doc = "Select secure attribute.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL15_A {
    #[doc = "1: Channel15 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Channel15 has its non-secure attribute set"]
    NON_SECURE = 0,
}
impl From<CHANNEL15_A> for bool {
    #[inline(always)]
    fn from(variant: CHANNEL15_A) -> Self {
        variant as u8 != 0
    }
}
impl CHANNEL15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHANNEL15_A {
        match self.bits {
            true => CHANNEL15_A::SECURE,
            false => CHANNEL15_A::NON_SECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == CHANNEL15_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NON_SECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == CHANNEL15_A::NON_SECURE
    }
}
#[doc = "Field `CHANNEL15` writer - Select secure attribute."]
pub type CHANNEL15_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERM_SPEC, CHANNEL15_A, O>;
impl<'a, const O: u8> CHANNEL15_W<'a, O> {
    #[doc = "Channel15 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL15_A::SECURE)
    }
    #[doc = "Channel15 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(CHANNEL15_A::NON_SECURE)
    }
}
impl R {
    #[doc = "Bit 0 - Select secure attribute."]
    #[inline(always)]
    pub fn channel0(&self) -> CHANNEL0_R {
        CHANNEL0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Select secure attribute."]
    #[inline(always)]
    pub fn channel1(&self) -> CHANNEL1_R {
        CHANNEL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Select secure attribute."]
    #[inline(always)]
    pub fn channel2(&self) -> CHANNEL2_R {
        CHANNEL2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Select secure attribute."]
    #[inline(always)]
    pub fn channel3(&self) -> CHANNEL3_R {
        CHANNEL3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Select secure attribute."]
    #[inline(always)]
    pub fn channel4(&self) -> CHANNEL4_R {
        CHANNEL4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Select secure attribute."]
    #[inline(always)]
    pub fn channel5(&self) -> CHANNEL5_R {
        CHANNEL5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Select secure attribute."]
    #[inline(always)]
    pub fn channel6(&self) -> CHANNEL6_R {
        CHANNEL6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Select secure attribute."]
    #[inline(always)]
    pub fn channel7(&self) -> CHANNEL7_R {
        CHANNEL7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Select secure attribute."]
    #[inline(always)]
    pub fn channel8(&self) -> CHANNEL8_R {
        CHANNEL8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Select secure attribute."]
    #[inline(always)]
    pub fn channel9(&self) -> CHANNEL9_R {
        CHANNEL9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Select secure attribute."]
    #[inline(always)]
    pub fn channel10(&self) -> CHANNEL10_R {
        CHANNEL10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Select secure attribute."]
    #[inline(always)]
    pub fn channel11(&self) -> CHANNEL11_R {
        CHANNEL11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Select secure attribute."]
    #[inline(always)]
    pub fn channel12(&self) -> CHANNEL12_R {
        CHANNEL12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Select secure attribute."]
    #[inline(always)]
    pub fn channel13(&self) -> CHANNEL13_R {
        CHANNEL13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Select secure attribute."]
    #[inline(always)]
    pub fn channel14(&self) -> CHANNEL14_R {
        CHANNEL14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Select secure attribute."]
    #[inline(always)]
    pub fn channel15(&self) -> CHANNEL15_R {
        CHANNEL15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Select secure attribute."]
    #[inline(always)]
    pub fn channel0(&mut self) -> CHANNEL0_W<0> {
        CHANNEL0_W::new(self)
    }
    #[doc = "Bit 1 - Select secure attribute."]
    #[inline(always)]
    pub fn channel1(&mut self) -> CHANNEL1_W<1> {
        CHANNEL1_W::new(self)
    }
    #[doc = "Bit 2 - Select secure attribute."]
    #[inline(always)]
    pub fn channel2(&mut self) -> CHANNEL2_W<2> {
        CHANNEL2_W::new(self)
    }
    #[doc = "Bit 3 - Select secure attribute."]
    #[inline(always)]
    pub fn channel3(&mut self) -> CHANNEL3_W<3> {
        CHANNEL3_W::new(self)
    }
    #[doc = "Bit 4 - Select secure attribute."]
    #[inline(always)]
    pub fn channel4(&mut self) -> CHANNEL4_W<4> {
        CHANNEL4_W::new(self)
    }
    #[doc = "Bit 5 - Select secure attribute."]
    #[inline(always)]
    pub fn channel5(&mut self) -> CHANNEL5_W<5> {
        CHANNEL5_W::new(self)
    }
    #[doc = "Bit 6 - Select secure attribute."]
    #[inline(always)]
    pub fn channel6(&mut self) -> CHANNEL6_W<6> {
        CHANNEL6_W::new(self)
    }
    #[doc = "Bit 7 - Select secure attribute."]
    #[inline(always)]
    pub fn channel7(&mut self) -> CHANNEL7_W<7> {
        CHANNEL7_W::new(self)
    }
    #[doc = "Bit 8 - Select secure attribute."]
    #[inline(always)]
    pub fn channel8(&mut self) -> CHANNEL8_W<8> {
        CHANNEL8_W::new(self)
    }
    #[doc = "Bit 9 - Select secure attribute."]
    #[inline(always)]
    pub fn channel9(&mut self) -> CHANNEL9_W<9> {
        CHANNEL9_W::new(self)
    }
    #[doc = "Bit 10 - Select secure attribute."]
    #[inline(always)]
    pub fn channel10(&mut self) -> CHANNEL10_W<10> {
        CHANNEL10_W::new(self)
    }
    #[doc = "Bit 11 - Select secure attribute."]
    #[inline(always)]
    pub fn channel11(&mut self) -> CHANNEL11_W<11> {
        CHANNEL11_W::new(self)
    }
    #[doc = "Bit 12 - Select secure attribute."]
    #[inline(always)]
    pub fn channel12(&mut self) -> CHANNEL12_W<12> {
        CHANNEL12_W::new(self)
    }
    #[doc = "Bit 13 - Select secure attribute."]
    #[inline(always)]
    pub fn channel13(&mut self) -> CHANNEL13_W<13> {
        CHANNEL13_W::new(self)
    }
    #[doc = "Bit 14 - Select secure attribute."]
    #[inline(always)]
    pub fn channel14(&mut self) -> CHANNEL14_W<14> {
        CHANNEL14_W::new(self)
    }
    #[doc = "Bit 15 - Select secure attribute."]
    #[inline(always)]
    pub fn channel15(&mut self) -> CHANNEL15_W<15> {
        CHANNEL15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description cluster: Select between secure and non-secure attribute for the DPPI channels.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perm](index.html) module"]
pub struct PERM_SPEC;
impl crate::RegisterSpec for PERM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [perm::R](R) reader structure"]
impl crate::Readable for PERM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [perm::W](W) writer structure"]
impl crate::Writable for PERM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PERM to value 0xffff"]
impl crate::Resettable for PERM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
