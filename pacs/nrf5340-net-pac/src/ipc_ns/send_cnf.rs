#[doc = "Register `SEND_CNF[%s]` reader"]
pub struct R(crate::R<SEND_CNF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEND_CNF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEND_CNF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEND_CNF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEND_CNF[%s]` writer"]
pub struct W(crate::W<SEND_CNF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEND_CNF_SPEC>;
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
impl From<crate::W<SEND_CNF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEND_CNF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHEN0` reader - Enable broadcasting on IPC channel 0"]
pub type CHEN0_R = crate::BitReader<CHEN0_A>;
#[doc = "Enable broadcasting on IPC channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN0_A {
    #[doc = "0: Disable broadcast"]
    DISABLE = 0,
    #[doc = "1: Enable broadcast"]
    ENABLE = 1,
}
impl From<CHEN0_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN0_A) -> Self {
        variant as u8 != 0
    }
}
impl CHEN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN0_A {
        match self.bits {
            false => CHEN0_A::DISABLE,
            true => CHEN0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHEN0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CHEN0_A::ENABLE
    }
}
#[doc = "Field `CHEN0` writer - Enable broadcasting on IPC channel 0"]
pub type CHEN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SEND_CNF_SPEC, CHEN0_A, O>;
impl<'a, const O: u8> CHEN0_W<'a, O> {
    #[doc = "Disable broadcast"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN0_A::DISABLE)
    }
    #[doc = "Enable broadcast"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHEN0_A::ENABLE)
    }
}
#[doc = "Field `CHEN1` reader - Enable broadcasting on IPC channel 1"]
pub type CHEN1_R = crate::BitReader<CHEN1_A>;
#[doc = "Enable broadcasting on IPC channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN1_A {
    #[doc = "0: Disable broadcast"]
    DISABLE = 0,
    #[doc = "1: Enable broadcast"]
    ENABLE = 1,
}
impl From<CHEN1_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN1_A) -> Self {
        variant as u8 != 0
    }
}
impl CHEN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN1_A {
        match self.bits {
            false => CHEN1_A::DISABLE,
            true => CHEN1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHEN1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CHEN1_A::ENABLE
    }
}
#[doc = "Field `CHEN1` writer - Enable broadcasting on IPC channel 1"]
pub type CHEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SEND_CNF_SPEC, CHEN1_A, O>;
impl<'a, const O: u8> CHEN1_W<'a, O> {
    #[doc = "Disable broadcast"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN1_A::DISABLE)
    }
    #[doc = "Enable broadcast"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHEN1_A::ENABLE)
    }
}
#[doc = "Field `CHEN2` reader - Enable broadcasting on IPC channel 2"]
pub type CHEN2_R = crate::BitReader<CHEN2_A>;
#[doc = "Enable broadcasting on IPC channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN2_A {
    #[doc = "0: Disable broadcast"]
    DISABLE = 0,
    #[doc = "1: Enable broadcast"]
    ENABLE = 1,
}
impl From<CHEN2_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN2_A) -> Self {
        variant as u8 != 0
    }
}
impl CHEN2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN2_A {
        match self.bits {
            false => CHEN2_A::DISABLE,
            true => CHEN2_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHEN2_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CHEN2_A::ENABLE
    }
}
#[doc = "Field `CHEN2` writer - Enable broadcasting on IPC channel 2"]
pub type CHEN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SEND_CNF_SPEC, CHEN2_A, O>;
impl<'a, const O: u8> CHEN2_W<'a, O> {
    #[doc = "Disable broadcast"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN2_A::DISABLE)
    }
    #[doc = "Enable broadcast"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHEN2_A::ENABLE)
    }
}
#[doc = "Field `CHEN3` reader - Enable broadcasting on IPC channel 3"]
pub type CHEN3_R = crate::BitReader<CHEN3_A>;
#[doc = "Enable broadcasting on IPC channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN3_A {
    #[doc = "0: Disable broadcast"]
    DISABLE = 0,
    #[doc = "1: Enable broadcast"]
    ENABLE = 1,
}
impl From<CHEN3_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN3_A) -> Self {
        variant as u8 != 0
    }
}
impl CHEN3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN3_A {
        match self.bits {
            false => CHEN3_A::DISABLE,
            true => CHEN3_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHEN3_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CHEN3_A::ENABLE
    }
}
#[doc = "Field `CHEN3` writer - Enable broadcasting on IPC channel 3"]
pub type CHEN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, SEND_CNF_SPEC, CHEN3_A, O>;
impl<'a, const O: u8> CHEN3_W<'a, O> {
    #[doc = "Disable broadcast"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN3_A::DISABLE)
    }
    #[doc = "Enable broadcast"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHEN3_A::ENABLE)
    }
}
#[doc = "Field `CHEN4` reader - Enable broadcasting on IPC channel 4"]
pub type CHEN4_R = crate::BitReader<CHEN4_A>;
#[doc = "Enable broadcasting on IPC channel 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN4_A {
    #[doc = "0: Disable broadcast"]
    DISABLE = 0,
    #[doc = "1: Enable broadcast"]
    ENABLE = 1,
}
impl From<CHEN4_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN4_A) -> Self {
        variant as u8 != 0
    }
}
impl CHEN4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN4_A {
        match self.bits {
            false => CHEN4_A::DISABLE,
            true => CHEN4_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHEN4_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CHEN4_A::ENABLE
    }
}
#[doc = "Field `CHEN4` writer - Enable broadcasting on IPC channel 4"]
pub type CHEN4_W<'a, const O: u8> = crate::BitWriter<'a, u32, SEND_CNF_SPEC, CHEN4_A, O>;
impl<'a, const O: u8> CHEN4_W<'a, O> {
    #[doc = "Disable broadcast"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN4_A::DISABLE)
    }
    #[doc = "Enable broadcast"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHEN4_A::ENABLE)
    }
}
#[doc = "Field `CHEN5` reader - Enable broadcasting on IPC channel 5"]
pub type CHEN5_R = crate::BitReader<CHEN5_A>;
#[doc = "Enable broadcasting on IPC channel 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN5_A {
    #[doc = "0: Disable broadcast"]
    DISABLE = 0,
    #[doc = "1: Enable broadcast"]
    ENABLE = 1,
}
impl From<CHEN5_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN5_A) -> Self {
        variant as u8 != 0
    }
}
impl CHEN5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN5_A {
        match self.bits {
            false => CHEN5_A::DISABLE,
            true => CHEN5_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHEN5_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CHEN5_A::ENABLE
    }
}
#[doc = "Field `CHEN5` writer - Enable broadcasting on IPC channel 5"]
pub type CHEN5_W<'a, const O: u8> = crate::BitWriter<'a, u32, SEND_CNF_SPEC, CHEN5_A, O>;
impl<'a, const O: u8> CHEN5_W<'a, O> {
    #[doc = "Disable broadcast"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN5_A::DISABLE)
    }
    #[doc = "Enable broadcast"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHEN5_A::ENABLE)
    }
}
#[doc = "Field `CHEN6` reader - Enable broadcasting on IPC channel 6"]
pub type CHEN6_R = crate::BitReader<CHEN6_A>;
#[doc = "Enable broadcasting on IPC channel 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN6_A {
    #[doc = "0: Disable broadcast"]
    DISABLE = 0,
    #[doc = "1: Enable broadcast"]
    ENABLE = 1,
}
impl From<CHEN6_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN6_A) -> Self {
        variant as u8 != 0
    }
}
impl CHEN6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN6_A {
        match self.bits {
            false => CHEN6_A::DISABLE,
            true => CHEN6_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHEN6_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CHEN6_A::ENABLE
    }
}
#[doc = "Field `CHEN6` writer - Enable broadcasting on IPC channel 6"]
pub type CHEN6_W<'a, const O: u8> = crate::BitWriter<'a, u32, SEND_CNF_SPEC, CHEN6_A, O>;
impl<'a, const O: u8> CHEN6_W<'a, O> {
    #[doc = "Disable broadcast"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN6_A::DISABLE)
    }
    #[doc = "Enable broadcast"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHEN6_A::ENABLE)
    }
}
#[doc = "Field `CHEN7` reader - Enable broadcasting on IPC channel 7"]
pub type CHEN7_R = crate::BitReader<CHEN7_A>;
#[doc = "Enable broadcasting on IPC channel 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN7_A {
    #[doc = "0: Disable broadcast"]
    DISABLE = 0,
    #[doc = "1: Enable broadcast"]
    ENABLE = 1,
}
impl From<CHEN7_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN7_A) -> Self {
        variant as u8 != 0
    }
}
impl CHEN7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN7_A {
        match self.bits {
            false => CHEN7_A::DISABLE,
            true => CHEN7_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHEN7_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CHEN7_A::ENABLE
    }
}
#[doc = "Field `CHEN7` writer - Enable broadcasting on IPC channel 7"]
pub type CHEN7_W<'a, const O: u8> = crate::BitWriter<'a, u32, SEND_CNF_SPEC, CHEN7_A, O>;
impl<'a, const O: u8> CHEN7_W<'a, O> {
    #[doc = "Disable broadcast"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN7_A::DISABLE)
    }
    #[doc = "Enable broadcast"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHEN7_A::ENABLE)
    }
}
#[doc = "Field `CHEN8` reader - Enable broadcasting on IPC channel 8"]
pub type CHEN8_R = crate::BitReader<CHEN8_A>;
#[doc = "Enable broadcasting on IPC channel 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN8_A {
    #[doc = "0: Disable broadcast"]
    DISABLE = 0,
    #[doc = "1: Enable broadcast"]
    ENABLE = 1,
}
impl From<CHEN8_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN8_A) -> Self {
        variant as u8 != 0
    }
}
impl CHEN8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN8_A {
        match self.bits {
            false => CHEN8_A::DISABLE,
            true => CHEN8_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHEN8_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CHEN8_A::ENABLE
    }
}
#[doc = "Field `CHEN8` writer - Enable broadcasting on IPC channel 8"]
pub type CHEN8_W<'a, const O: u8> = crate::BitWriter<'a, u32, SEND_CNF_SPEC, CHEN8_A, O>;
impl<'a, const O: u8> CHEN8_W<'a, O> {
    #[doc = "Disable broadcast"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN8_A::DISABLE)
    }
    #[doc = "Enable broadcast"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHEN8_A::ENABLE)
    }
}
#[doc = "Field `CHEN9` reader - Enable broadcasting on IPC channel 9"]
pub type CHEN9_R = crate::BitReader<CHEN9_A>;
#[doc = "Enable broadcasting on IPC channel 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN9_A {
    #[doc = "0: Disable broadcast"]
    DISABLE = 0,
    #[doc = "1: Enable broadcast"]
    ENABLE = 1,
}
impl From<CHEN9_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN9_A) -> Self {
        variant as u8 != 0
    }
}
impl CHEN9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN9_A {
        match self.bits {
            false => CHEN9_A::DISABLE,
            true => CHEN9_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHEN9_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CHEN9_A::ENABLE
    }
}
#[doc = "Field `CHEN9` writer - Enable broadcasting on IPC channel 9"]
pub type CHEN9_W<'a, const O: u8> = crate::BitWriter<'a, u32, SEND_CNF_SPEC, CHEN9_A, O>;
impl<'a, const O: u8> CHEN9_W<'a, O> {
    #[doc = "Disable broadcast"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN9_A::DISABLE)
    }
    #[doc = "Enable broadcast"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHEN9_A::ENABLE)
    }
}
#[doc = "Field `CHEN10` reader - Enable broadcasting on IPC channel 10"]
pub type CHEN10_R = crate::BitReader<CHEN10_A>;
#[doc = "Enable broadcasting on IPC channel 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN10_A {
    #[doc = "0: Disable broadcast"]
    DISABLE = 0,
    #[doc = "1: Enable broadcast"]
    ENABLE = 1,
}
impl From<CHEN10_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN10_A) -> Self {
        variant as u8 != 0
    }
}
impl CHEN10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN10_A {
        match self.bits {
            false => CHEN10_A::DISABLE,
            true => CHEN10_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHEN10_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CHEN10_A::ENABLE
    }
}
#[doc = "Field `CHEN10` writer - Enable broadcasting on IPC channel 10"]
pub type CHEN10_W<'a, const O: u8> = crate::BitWriter<'a, u32, SEND_CNF_SPEC, CHEN10_A, O>;
impl<'a, const O: u8> CHEN10_W<'a, O> {
    #[doc = "Disable broadcast"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN10_A::DISABLE)
    }
    #[doc = "Enable broadcast"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHEN10_A::ENABLE)
    }
}
#[doc = "Field `CHEN11` reader - Enable broadcasting on IPC channel 11"]
pub type CHEN11_R = crate::BitReader<CHEN11_A>;
#[doc = "Enable broadcasting on IPC channel 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN11_A {
    #[doc = "0: Disable broadcast"]
    DISABLE = 0,
    #[doc = "1: Enable broadcast"]
    ENABLE = 1,
}
impl From<CHEN11_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN11_A) -> Self {
        variant as u8 != 0
    }
}
impl CHEN11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN11_A {
        match self.bits {
            false => CHEN11_A::DISABLE,
            true => CHEN11_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHEN11_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CHEN11_A::ENABLE
    }
}
#[doc = "Field `CHEN11` writer - Enable broadcasting on IPC channel 11"]
pub type CHEN11_W<'a, const O: u8> = crate::BitWriter<'a, u32, SEND_CNF_SPEC, CHEN11_A, O>;
impl<'a, const O: u8> CHEN11_W<'a, O> {
    #[doc = "Disable broadcast"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN11_A::DISABLE)
    }
    #[doc = "Enable broadcast"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHEN11_A::ENABLE)
    }
}
#[doc = "Field `CHEN12` reader - Enable broadcasting on IPC channel 12"]
pub type CHEN12_R = crate::BitReader<CHEN12_A>;
#[doc = "Enable broadcasting on IPC channel 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN12_A {
    #[doc = "0: Disable broadcast"]
    DISABLE = 0,
    #[doc = "1: Enable broadcast"]
    ENABLE = 1,
}
impl From<CHEN12_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN12_A) -> Self {
        variant as u8 != 0
    }
}
impl CHEN12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN12_A {
        match self.bits {
            false => CHEN12_A::DISABLE,
            true => CHEN12_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHEN12_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CHEN12_A::ENABLE
    }
}
#[doc = "Field `CHEN12` writer - Enable broadcasting on IPC channel 12"]
pub type CHEN12_W<'a, const O: u8> = crate::BitWriter<'a, u32, SEND_CNF_SPEC, CHEN12_A, O>;
impl<'a, const O: u8> CHEN12_W<'a, O> {
    #[doc = "Disable broadcast"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN12_A::DISABLE)
    }
    #[doc = "Enable broadcast"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHEN12_A::ENABLE)
    }
}
#[doc = "Field `CHEN13` reader - Enable broadcasting on IPC channel 13"]
pub type CHEN13_R = crate::BitReader<CHEN13_A>;
#[doc = "Enable broadcasting on IPC channel 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN13_A {
    #[doc = "0: Disable broadcast"]
    DISABLE = 0,
    #[doc = "1: Enable broadcast"]
    ENABLE = 1,
}
impl From<CHEN13_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN13_A) -> Self {
        variant as u8 != 0
    }
}
impl CHEN13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN13_A {
        match self.bits {
            false => CHEN13_A::DISABLE,
            true => CHEN13_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHEN13_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CHEN13_A::ENABLE
    }
}
#[doc = "Field `CHEN13` writer - Enable broadcasting on IPC channel 13"]
pub type CHEN13_W<'a, const O: u8> = crate::BitWriter<'a, u32, SEND_CNF_SPEC, CHEN13_A, O>;
impl<'a, const O: u8> CHEN13_W<'a, O> {
    #[doc = "Disable broadcast"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN13_A::DISABLE)
    }
    #[doc = "Enable broadcast"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHEN13_A::ENABLE)
    }
}
#[doc = "Field `CHEN14` reader - Enable broadcasting on IPC channel 14"]
pub type CHEN14_R = crate::BitReader<CHEN14_A>;
#[doc = "Enable broadcasting on IPC channel 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN14_A {
    #[doc = "0: Disable broadcast"]
    DISABLE = 0,
    #[doc = "1: Enable broadcast"]
    ENABLE = 1,
}
impl From<CHEN14_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN14_A) -> Self {
        variant as u8 != 0
    }
}
impl CHEN14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN14_A {
        match self.bits {
            false => CHEN14_A::DISABLE,
            true => CHEN14_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHEN14_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CHEN14_A::ENABLE
    }
}
#[doc = "Field `CHEN14` writer - Enable broadcasting on IPC channel 14"]
pub type CHEN14_W<'a, const O: u8> = crate::BitWriter<'a, u32, SEND_CNF_SPEC, CHEN14_A, O>;
impl<'a, const O: u8> CHEN14_W<'a, O> {
    #[doc = "Disable broadcast"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN14_A::DISABLE)
    }
    #[doc = "Enable broadcast"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHEN14_A::ENABLE)
    }
}
#[doc = "Field `CHEN15` reader - Enable broadcasting on IPC channel 15"]
pub type CHEN15_R = crate::BitReader<CHEN15_A>;
#[doc = "Enable broadcasting on IPC channel 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN15_A {
    #[doc = "0: Disable broadcast"]
    DISABLE = 0,
    #[doc = "1: Enable broadcast"]
    ENABLE = 1,
}
impl From<CHEN15_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN15_A) -> Self {
        variant as u8 != 0
    }
}
impl CHEN15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN15_A {
        match self.bits {
            false => CHEN15_A::DISABLE,
            true => CHEN15_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHEN15_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CHEN15_A::ENABLE
    }
}
#[doc = "Field `CHEN15` writer - Enable broadcasting on IPC channel 15"]
pub type CHEN15_W<'a, const O: u8> = crate::BitWriter<'a, u32, SEND_CNF_SPEC, CHEN15_A, O>;
impl<'a, const O: u8> CHEN15_W<'a, O> {
    #[doc = "Disable broadcast"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN15_A::DISABLE)
    }
    #[doc = "Enable broadcast"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHEN15_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - Enable broadcasting on IPC channel 0"]
    #[inline(always)]
    pub fn chen0(&self) -> CHEN0_R {
        CHEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable broadcasting on IPC channel 1"]
    #[inline(always)]
    pub fn chen1(&self) -> CHEN1_R {
        CHEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable broadcasting on IPC channel 2"]
    #[inline(always)]
    pub fn chen2(&self) -> CHEN2_R {
        CHEN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable broadcasting on IPC channel 3"]
    #[inline(always)]
    pub fn chen3(&self) -> CHEN3_R {
        CHEN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable broadcasting on IPC channel 4"]
    #[inline(always)]
    pub fn chen4(&self) -> CHEN4_R {
        CHEN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable broadcasting on IPC channel 5"]
    #[inline(always)]
    pub fn chen5(&self) -> CHEN5_R {
        CHEN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable broadcasting on IPC channel 6"]
    #[inline(always)]
    pub fn chen6(&self) -> CHEN6_R {
        CHEN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable broadcasting on IPC channel 7"]
    #[inline(always)]
    pub fn chen7(&self) -> CHEN7_R {
        CHEN7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable broadcasting on IPC channel 8"]
    #[inline(always)]
    pub fn chen8(&self) -> CHEN8_R {
        CHEN8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable broadcasting on IPC channel 9"]
    #[inline(always)]
    pub fn chen9(&self) -> CHEN9_R {
        CHEN9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable broadcasting on IPC channel 10"]
    #[inline(always)]
    pub fn chen10(&self) -> CHEN10_R {
        CHEN10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable broadcasting on IPC channel 11"]
    #[inline(always)]
    pub fn chen11(&self) -> CHEN11_R {
        CHEN11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable broadcasting on IPC channel 12"]
    #[inline(always)]
    pub fn chen12(&self) -> CHEN12_R {
        CHEN12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable broadcasting on IPC channel 13"]
    #[inline(always)]
    pub fn chen13(&self) -> CHEN13_R {
        CHEN13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable broadcasting on IPC channel 14"]
    #[inline(always)]
    pub fn chen14(&self) -> CHEN14_R {
        CHEN14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable broadcasting on IPC channel 15"]
    #[inline(always)]
    pub fn chen15(&self) -> CHEN15_R {
        CHEN15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable broadcasting on IPC channel 0"]
    #[inline(always)]
    pub fn chen0(&mut self) -> CHEN0_W<0> {
        CHEN0_W::new(self)
    }
    #[doc = "Bit 1 - Enable broadcasting on IPC channel 1"]
    #[inline(always)]
    pub fn chen1(&mut self) -> CHEN1_W<1> {
        CHEN1_W::new(self)
    }
    #[doc = "Bit 2 - Enable broadcasting on IPC channel 2"]
    #[inline(always)]
    pub fn chen2(&mut self) -> CHEN2_W<2> {
        CHEN2_W::new(self)
    }
    #[doc = "Bit 3 - Enable broadcasting on IPC channel 3"]
    #[inline(always)]
    pub fn chen3(&mut self) -> CHEN3_W<3> {
        CHEN3_W::new(self)
    }
    #[doc = "Bit 4 - Enable broadcasting on IPC channel 4"]
    #[inline(always)]
    pub fn chen4(&mut self) -> CHEN4_W<4> {
        CHEN4_W::new(self)
    }
    #[doc = "Bit 5 - Enable broadcasting on IPC channel 5"]
    #[inline(always)]
    pub fn chen5(&mut self) -> CHEN5_W<5> {
        CHEN5_W::new(self)
    }
    #[doc = "Bit 6 - Enable broadcasting on IPC channel 6"]
    #[inline(always)]
    pub fn chen6(&mut self) -> CHEN6_W<6> {
        CHEN6_W::new(self)
    }
    #[doc = "Bit 7 - Enable broadcasting on IPC channel 7"]
    #[inline(always)]
    pub fn chen7(&mut self) -> CHEN7_W<7> {
        CHEN7_W::new(self)
    }
    #[doc = "Bit 8 - Enable broadcasting on IPC channel 8"]
    #[inline(always)]
    pub fn chen8(&mut self) -> CHEN8_W<8> {
        CHEN8_W::new(self)
    }
    #[doc = "Bit 9 - Enable broadcasting on IPC channel 9"]
    #[inline(always)]
    pub fn chen9(&mut self) -> CHEN9_W<9> {
        CHEN9_W::new(self)
    }
    #[doc = "Bit 10 - Enable broadcasting on IPC channel 10"]
    #[inline(always)]
    pub fn chen10(&mut self) -> CHEN10_W<10> {
        CHEN10_W::new(self)
    }
    #[doc = "Bit 11 - Enable broadcasting on IPC channel 11"]
    #[inline(always)]
    pub fn chen11(&mut self) -> CHEN11_W<11> {
        CHEN11_W::new(self)
    }
    #[doc = "Bit 12 - Enable broadcasting on IPC channel 12"]
    #[inline(always)]
    pub fn chen12(&mut self) -> CHEN12_W<12> {
        CHEN12_W::new(self)
    }
    #[doc = "Bit 13 - Enable broadcasting on IPC channel 13"]
    #[inline(always)]
    pub fn chen13(&mut self) -> CHEN13_W<13> {
        CHEN13_W::new(self)
    }
    #[doc = "Bit 14 - Enable broadcasting on IPC channel 14"]
    #[inline(always)]
    pub fn chen14(&mut self) -> CHEN14_W<14> {
        CHEN14_W::new(self)
    }
    #[doc = "Bit 15 - Enable broadcasting on IPC channel 15"]
    #[inline(always)]
    pub fn chen15(&mut self) -> CHEN15_W<15> {
        CHEN15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description collection: Send event configuration for TASKS_SEND\\[n\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [send_cnf](index.html) module"]
pub struct SEND_CNF_SPEC;
impl crate::RegisterSpec for SEND_CNF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [send_cnf::R](R) reader structure"]
impl crate::Readable for SEND_CNF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [send_cnf::W](W) writer structure"]
impl crate::Writable for SEND_CNF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SEND_CNF[%s]
to value 0"]
impl crate::Resettable for SEND_CNF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
