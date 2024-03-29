#[doc = "Register `POWER` reader"]
pub struct R(crate::R<POWER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<POWER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<POWER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<POWER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `POWER` writer"]
pub struct W(crate::W<POWER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<POWER_SPEC>;
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
impl From<crate::W<POWER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<POWER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `S0POWER` reader - Keep RAM section S0 on or off in System ON mode."]
pub type S0POWER_R = crate::BitReader<S0POWER_A>;
#[doc = "Keep RAM section S0 on or off in System ON mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S0POWER_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: On"]
    ON = 1,
}
impl From<S0POWER_A> for bool {
    #[inline(always)]
    fn from(variant: S0POWER_A) -> Self {
        variant as u8 != 0
    }
}
impl S0POWER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S0POWER_A {
        match self.bits {
            false => S0POWER_A::OFF,
            true => S0POWER_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S0POWER_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S0POWER_A::ON
    }
}
#[doc = "Field `S0POWER` writer - Keep RAM section S0 on or off in System ON mode."]
pub type S0POWER_W<'a, const O: u8> = crate::BitWriter<'a, u32, POWER_SPEC, S0POWER_A, O>;
impl<'a, const O: u8> S0POWER_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S0POWER_A::OFF)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S0POWER_A::ON)
    }
}
#[doc = "Field `S1POWER` reader - Keep RAM section S1 on or off in System ON mode."]
pub type S1POWER_R = crate::BitReader<S1POWER_A>;
#[doc = "Keep RAM section S1 on or off in System ON mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S1POWER_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: On"]
    ON = 1,
}
impl From<S1POWER_A> for bool {
    #[inline(always)]
    fn from(variant: S1POWER_A) -> Self {
        variant as u8 != 0
    }
}
impl S1POWER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S1POWER_A {
        match self.bits {
            false => S1POWER_A::OFF,
            true => S1POWER_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S1POWER_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S1POWER_A::ON
    }
}
#[doc = "Field `S1POWER` writer - Keep RAM section S1 on or off in System ON mode."]
pub type S1POWER_W<'a, const O: u8> = crate::BitWriter<'a, u32, POWER_SPEC, S1POWER_A, O>;
impl<'a, const O: u8> S1POWER_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S1POWER_A::OFF)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S1POWER_A::ON)
    }
}
#[doc = "Field `S2POWER` reader - Keep RAM section S2 on or off in System ON mode."]
pub type S2POWER_R = crate::BitReader<S2POWER_A>;
#[doc = "Keep RAM section S2 on or off in System ON mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S2POWER_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: On"]
    ON = 1,
}
impl From<S2POWER_A> for bool {
    #[inline(always)]
    fn from(variant: S2POWER_A) -> Self {
        variant as u8 != 0
    }
}
impl S2POWER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S2POWER_A {
        match self.bits {
            false => S2POWER_A::OFF,
            true => S2POWER_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S2POWER_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S2POWER_A::ON
    }
}
#[doc = "Field `S2POWER` writer - Keep RAM section S2 on or off in System ON mode."]
pub type S2POWER_W<'a, const O: u8> = crate::BitWriter<'a, u32, POWER_SPEC, S2POWER_A, O>;
impl<'a, const O: u8> S2POWER_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S2POWER_A::OFF)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S2POWER_A::ON)
    }
}
#[doc = "Field `S3POWER` reader - Keep RAM section S3 on or off in System ON mode."]
pub type S3POWER_R = crate::BitReader<S3POWER_A>;
#[doc = "Keep RAM section S3 on or off in System ON mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S3POWER_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: On"]
    ON = 1,
}
impl From<S3POWER_A> for bool {
    #[inline(always)]
    fn from(variant: S3POWER_A) -> Self {
        variant as u8 != 0
    }
}
impl S3POWER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S3POWER_A {
        match self.bits {
            false => S3POWER_A::OFF,
            true => S3POWER_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S3POWER_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S3POWER_A::ON
    }
}
#[doc = "Field `S3POWER` writer - Keep RAM section S3 on or off in System ON mode."]
pub type S3POWER_W<'a, const O: u8> = crate::BitWriter<'a, u32, POWER_SPEC, S3POWER_A, O>;
impl<'a, const O: u8> S3POWER_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S3POWER_A::OFF)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S3POWER_A::ON)
    }
}
#[doc = "Field `S4POWER` reader - Keep RAM section S4 on or off in System ON mode."]
pub type S4POWER_R = crate::BitReader<S4POWER_A>;
#[doc = "Keep RAM section S4 on or off in System ON mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S4POWER_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: On"]
    ON = 1,
}
impl From<S4POWER_A> for bool {
    #[inline(always)]
    fn from(variant: S4POWER_A) -> Self {
        variant as u8 != 0
    }
}
impl S4POWER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S4POWER_A {
        match self.bits {
            false => S4POWER_A::OFF,
            true => S4POWER_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S4POWER_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S4POWER_A::ON
    }
}
#[doc = "Field `S4POWER` writer - Keep RAM section S4 on or off in System ON mode."]
pub type S4POWER_W<'a, const O: u8> = crate::BitWriter<'a, u32, POWER_SPEC, S4POWER_A, O>;
impl<'a, const O: u8> S4POWER_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S4POWER_A::OFF)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S4POWER_A::ON)
    }
}
#[doc = "Field `S5POWER` reader - Keep RAM section S5 on or off in System ON mode."]
pub type S5POWER_R = crate::BitReader<S5POWER_A>;
#[doc = "Keep RAM section S5 on or off in System ON mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S5POWER_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: On"]
    ON = 1,
}
impl From<S5POWER_A> for bool {
    #[inline(always)]
    fn from(variant: S5POWER_A) -> Self {
        variant as u8 != 0
    }
}
impl S5POWER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S5POWER_A {
        match self.bits {
            false => S5POWER_A::OFF,
            true => S5POWER_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S5POWER_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S5POWER_A::ON
    }
}
#[doc = "Field `S5POWER` writer - Keep RAM section S5 on or off in System ON mode."]
pub type S5POWER_W<'a, const O: u8> = crate::BitWriter<'a, u32, POWER_SPEC, S5POWER_A, O>;
impl<'a, const O: u8> S5POWER_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S5POWER_A::OFF)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S5POWER_A::ON)
    }
}
#[doc = "Field `S6POWER` reader - Keep RAM section S6 on or off in System ON mode."]
pub type S6POWER_R = crate::BitReader<S6POWER_A>;
#[doc = "Keep RAM section S6 on or off in System ON mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S6POWER_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: On"]
    ON = 1,
}
impl From<S6POWER_A> for bool {
    #[inline(always)]
    fn from(variant: S6POWER_A) -> Self {
        variant as u8 != 0
    }
}
impl S6POWER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S6POWER_A {
        match self.bits {
            false => S6POWER_A::OFF,
            true => S6POWER_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S6POWER_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S6POWER_A::ON
    }
}
#[doc = "Field `S6POWER` writer - Keep RAM section S6 on or off in System ON mode."]
pub type S6POWER_W<'a, const O: u8> = crate::BitWriter<'a, u32, POWER_SPEC, S6POWER_A, O>;
impl<'a, const O: u8> S6POWER_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S6POWER_A::OFF)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S6POWER_A::ON)
    }
}
#[doc = "Field `S7POWER` reader - Keep RAM section S7 on or off in System ON mode."]
pub type S7POWER_R = crate::BitReader<S7POWER_A>;
#[doc = "Keep RAM section S7 on or off in System ON mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S7POWER_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: On"]
    ON = 1,
}
impl From<S7POWER_A> for bool {
    #[inline(always)]
    fn from(variant: S7POWER_A) -> Self {
        variant as u8 != 0
    }
}
impl S7POWER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S7POWER_A {
        match self.bits {
            false => S7POWER_A::OFF,
            true => S7POWER_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S7POWER_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S7POWER_A::ON
    }
}
#[doc = "Field `S7POWER` writer - Keep RAM section S7 on or off in System ON mode."]
pub type S7POWER_W<'a, const O: u8> = crate::BitWriter<'a, u32, POWER_SPEC, S7POWER_A, O>;
impl<'a, const O: u8> S7POWER_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S7POWER_A::OFF)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S7POWER_A::ON)
    }
}
#[doc = "Field `S8POWER` reader - Keep RAM section S8 on or off in System ON mode."]
pub type S8POWER_R = crate::BitReader<S8POWER_A>;
#[doc = "Keep RAM section S8 on or off in System ON mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S8POWER_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: On"]
    ON = 1,
}
impl From<S8POWER_A> for bool {
    #[inline(always)]
    fn from(variant: S8POWER_A) -> Self {
        variant as u8 != 0
    }
}
impl S8POWER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S8POWER_A {
        match self.bits {
            false => S8POWER_A::OFF,
            true => S8POWER_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S8POWER_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S8POWER_A::ON
    }
}
#[doc = "Field `S8POWER` writer - Keep RAM section S8 on or off in System ON mode."]
pub type S8POWER_W<'a, const O: u8> = crate::BitWriter<'a, u32, POWER_SPEC, S8POWER_A, O>;
impl<'a, const O: u8> S8POWER_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S8POWER_A::OFF)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S8POWER_A::ON)
    }
}
#[doc = "Field `S9POWER` reader - Keep RAM section S9 on or off in System ON mode."]
pub type S9POWER_R = crate::BitReader<S9POWER_A>;
#[doc = "Keep RAM section S9 on or off in System ON mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S9POWER_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: On"]
    ON = 1,
}
impl From<S9POWER_A> for bool {
    #[inline(always)]
    fn from(variant: S9POWER_A) -> Self {
        variant as u8 != 0
    }
}
impl S9POWER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S9POWER_A {
        match self.bits {
            false => S9POWER_A::OFF,
            true => S9POWER_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S9POWER_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S9POWER_A::ON
    }
}
#[doc = "Field `S9POWER` writer - Keep RAM section S9 on or off in System ON mode."]
pub type S9POWER_W<'a, const O: u8> = crate::BitWriter<'a, u32, POWER_SPEC, S9POWER_A, O>;
impl<'a, const O: u8> S9POWER_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S9POWER_A::OFF)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S9POWER_A::ON)
    }
}
#[doc = "Field `S10POWER` reader - Keep RAM section S10 on or off in System ON mode."]
pub type S10POWER_R = crate::BitReader<S10POWER_A>;
#[doc = "Keep RAM section S10 on or off in System ON mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S10POWER_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: On"]
    ON = 1,
}
impl From<S10POWER_A> for bool {
    #[inline(always)]
    fn from(variant: S10POWER_A) -> Self {
        variant as u8 != 0
    }
}
impl S10POWER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S10POWER_A {
        match self.bits {
            false => S10POWER_A::OFF,
            true => S10POWER_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S10POWER_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S10POWER_A::ON
    }
}
#[doc = "Field `S10POWER` writer - Keep RAM section S10 on or off in System ON mode."]
pub type S10POWER_W<'a, const O: u8> = crate::BitWriter<'a, u32, POWER_SPEC, S10POWER_A, O>;
impl<'a, const O: u8> S10POWER_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S10POWER_A::OFF)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S10POWER_A::ON)
    }
}
#[doc = "Field `S11POWER` reader - Keep RAM section S11 on or off in System ON mode."]
pub type S11POWER_R = crate::BitReader<S11POWER_A>;
#[doc = "Keep RAM section S11 on or off in System ON mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S11POWER_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: On"]
    ON = 1,
}
impl From<S11POWER_A> for bool {
    #[inline(always)]
    fn from(variant: S11POWER_A) -> Self {
        variant as u8 != 0
    }
}
impl S11POWER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S11POWER_A {
        match self.bits {
            false => S11POWER_A::OFF,
            true => S11POWER_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S11POWER_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S11POWER_A::ON
    }
}
#[doc = "Field `S11POWER` writer - Keep RAM section S11 on or off in System ON mode."]
pub type S11POWER_W<'a, const O: u8> = crate::BitWriter<'a, u32, POWER_SPEC, S11POWER_A, O>;
impl<'a, const O: u8> S11POWER_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S11POWER_A::OFF)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S11POWER_A::ON)
    }
}
#[doc = "Field `S12POWER` reader - Keep RAM section S12 on or off in System ON mode."]
pub type S12POWER_R = crate::BitReader<S12POWER_A>;
#[doc = "Keep RAM section S12 on or off in System ON mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S12POWER_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: On"]
    ON = 1,
}
impl From<S12POWER_A> for bool {
    #[inline(always)]
    fn from(variant: S12POWER_A) -> Self {
        variant as u8 != 0
    }
}
impl S12POWER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S12POWER_A {
        match self.bits {
            false => S12POWER_A::OFF,
            true => S12POWER_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S12POWER_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S12POWER_A::ON
    }
}
#[doc = "Field `S12POWER` writer - Keep RAM section S12 on or off in System ON mode."]
pub type S12POWER_W<'a, const O: u8> = crate::BitWriter<'a, u32, POWER_SPEC, S12POWER_A, O>;
impl<'a, const O: u8> S12POWER_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S12POWER_A::OFF)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S12POWER_A::ON)
    }
}
#[doc = "Field `S13POWER` reader - Keep RAM section S13 on or off in System ON mode."]
pub type S13POWER_R = crate::BitReader<S13POWER_A>;
#[doc = "Keep RAM section S13 on or off in System ON mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S13POWER_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: On"]
    ON = 1,
}
impl From<S13POWER_A> for bool {
    #[inline(always)]
    fn from(variant: S13POWER_A) -> Self {
        variant as u8 != 0
    }
}
impl S13POWER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S13POWER_A {
        match self.bits {
            false => S13POWER_A::OFF,
            true => S13POWER_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S13POWER_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S13POWER_A::ON
    }
}
#[doc = "Field `S13POWER` writer - Keep RAM section S13 on or off in System ON mode."]
pub type S13POWER_W<'a, const O: u8> = crate::BitWriter<'a, u32, POWER_SPEC, S13POWER_A, O>;
impl<'a, const O: u8> S13POWER_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S13POWER_A::OFF)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S13POWER_A::ON)
    }
}
#[doc = "Field `S14POWER` reader - Keep RAM section S14 on or off in System ON mode."]
pub type S14POWER_R = crate::BitReader<S14POWER_A>;
#[doc = "Keep RAM section S14 on or off in System ON mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S14POWER_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: On"]
    ON = 1,
}
impl From<S14POWER_A> for bool {
    #[inline(always)]
    fn from(variant: S14POWER_A) -> Self {
        variant as u8 != 0
    }
}
impl S14POWER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S14POWER_A {
        match self.bits {
            false => S14POWER_A::OFF,
            true => S14POWER_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S14POWER_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S14POWER_A::ON
    }
}
#[doc = "Field `S14POWER` writer - Keep RAM section S14 on or off in System ON mode."]
pub type S14POWER_W<'a, const O: u8> = crate::BitWriter<'a, u32, POWER_SPEC, S14POWER_A, O>;
impl<'a, const O: u8> S14POWER_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S14POWER_A::OFF)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S14POWER_A::ON)
    }
}
#[doc = "Field `S15POWER` reader - Keep RAM section S15 on or off in System ON mode."]
pub type S15POWER_R = crate::BitReader<S15POWER_A>;
#[doc = "Keep RAM section S15 on or off in System ON mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S15POWER_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: On"]
    ON = 1,
}
impl From<S15POWER_A> for bool {
    #[inline(always)]
    fn from(variant: S15POWER_A) -> Self {
        variant as u8 != 0
    }
}
impl S15POWER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S15POWER_A {
        match self.bits {
            false => S15POWER_A::OFF,
            true => S15POWER_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S15POWER_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S15POWER_A::ON
    }
}
#[doc = "Field `S15POWER` writer - Keep RAM section S15 on or off in System ON mode."]
pub type S15POWER_W<'a, const O: u8> = crate::BitWriter<'a, u32, POWER_SPEC, S15POWER_A, O>;
impl<'a, const O: u8> S15POWER_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S15POWER_A::OFF)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S15POWER_A::ON)
    }
}
#[doc = "Field `S0RETENTION` reader - Keep retention on RAM section S0 when RAM section is off"]
pub type S0RETENTION_R = crate::BitReader<S0RETENTION_A>;
#[doc = "Keep retention on RAM section S0 when RAM section is off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S0RETENTION_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: On"]
    ON = 1,
}
impl From<S0RETENTION_A> for bool {
    #[inline(always)]
    fn from(variant: S0RETENTION_A) -> Self {
        variant as u8 != 0
    }
}
impl S0RETENTION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S0RETENTION_A {
        match self.bits {
            false => S0RETENTION_A::OFF,
            true => S0RETENTION_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S0RETENTION_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S0RETENTION_A::ON
    }
}
#[doc = "Field `S0RETENTION` writer - Keep retention on RAM section S0 when RAM section is off"]
pub type S0RETENTION_W<'a, const O: u8> = crate::BitWriter<'a, u32, POWER_SPEC, S0RETENTION_A, O>;
impl<'a, const O: u8> S0RETENTION_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S0RETENTION_A::OFF)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S0RETENTION_A::ON)
    }
}
#[doc = "Field `S1RETENTION` reader - Keep retention on RAM section S1 when RAM section is off"]
pub type S1RETENTION_R = crate::BitReader<S1RETENTION_A>;
#[doc = "Keep retention on RAM section S1 when RAM section is off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S1RETENTION_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: On"]
    ON = 1,
}
impl From<S1RETENTION_A> for bool {
    #[inline(always)]
    fn from(variant: S1RETENTION_A) -> Self {
        variant as u8 != 0
    }
}
impl S1RETENTION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S1RETENTION_A {
        match self.bits {
            false => S1RETENTION_A::OFF,
            true => S1RETENTION_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S1RETENTION_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S1RETENTION_A::ON
    }
}
#[doc = "Field `S1RETENTION` writer - Keep retention on RAM section S1 when RAM section is off"]
pub type S1RETENTION_W<'a, const O: u8> = crate::BitWriter<'a, u32, POWER_SPEC, S1RETENTION_A, O>;
impl<'a, const O: u8> S1RETENTION_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S1RETENTION_A::OFF)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S1RETENTION_A::ON)
    }
}
#[doc = "Field `S2RETENTION` reader - Keep retention on RAM section S2 when RAM section is off"]
pub type S2RETENTION_R = crate::BitReader<S2RETENTION_A>;
#[doc = "Keep retention on RAM section S2 when RAM section is off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S2RETENTION_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: On"]
    ON = 1,
}
impl From<S2RETENTION_A> for bool {
    #[inline(always)]
    fn from(variant: S2RETENTION_A) -> Self {
        variant as u8 != 0
    }
}
impl S2RETENTION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S2RETENTION_A {
        match self.bits {
            false => S2RETENTION_A::OFF,
            true => S2RETENTION_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S2RETENTION_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S2RETENTION_A::ON
    }
}
#[doc = "Field `S2RETENTION` writer - Keep retention on RAM section S2 when RAM section is off"]
pub type S2RETENTION_W<'a, const O: u8> = crate::BitWriter<'a, u32, POWER_SPEC, S2RETENTION_A, O>;
impl<'a, const O: u8> S2RETENTION_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S2RETENTION_A::OFF)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S2RETENTION_A::ON)
    }
}
#[doc = "Field `S3RETENTION` reader - Keep retention on RAM section S3 when RAM section is off"]
pub type S3RETENTION_R = crate::BitReader<S3RETENTION_A>;
#[doc = "Keep retention on RAM section S3 when RAM section is off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S3RETENTION_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: On"]
    ON = 1,
}
impl From<S3RETENTION_A> for bool {
    #[inline(always)]
    fn from(variant: S3RETENTION_A) -> Self {
        variant as u8 != 0
    }
}
impl S3RETENTION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S3RETENTION_A {
        match self.bits {
            false => S3RETENTION_A::OFF,
            true => S3RETENTION_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S3RETENTION_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S3RETENTION_A::ON
    }
}
#[doc = "Field `S3RETENTION` writer - Keep retention on RAM section S3 when RAM section is off"]
pub type S3RETENTION_W<'a, const O: u8> = crate::BitWriter<'a, u32, POWER_SPEC, S3RETENTION_A, O>;
impl<'a, const O: u8> S3RETENTION_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S3RETENTION_A::OFF)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S3RETENTION_A::ON)
    }
}
#[doc = "Field `S4RETENTION` reader - Keep retention on RAM section S4 when RAM section is off"]
pub type S4RETENTION_R = crate::BitReader<S4RETENTION_A>;
#[doc = "Keep retention on RAM section S4 when RAM section is off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S4RETENTION_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: On"]
    ON = 1,
}
impl From<S4RETENTION_A> for bool {
    #[inline(always)]
    fn from(variant: S4RETENTION_A) -> Self {
        variant as u8 != 0
    }
}
impl S4RETENTION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S4RETENTION_A {
        match self.bits {
            false => S4RETENTION_A::OFF,
            true => S4RETENTION_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S4RETENTION_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S4RETENTION_A::ON
    }
}
#[doc = "Field `S4RETENTION` writer - Keep retention on RAM section S4 when RAM section is off"]
pub type S4RETENTION_W<'a, const O: u8> = crate::BitWriter<'a, u32, POWER_SPEC, S4RETENTION_A, O>;
impl<'a, const O: u8> S4RETENTION_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S4RETENTION_A::OFF)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S4RETENTION_A::ON)
    }
}
#[doc = "Field `S5RETENTION` reader - Keep retention on RAM section S5 when RAM section is off"]
pub type S5RETENTION_R = crate::BitReader<S5RETENTION_A>;
#[doc = "Keep retention on RAM section S5 when RAM section is off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S5RETENTION_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: On"]
    ON = 1,
}
impl From<S5RETENTION_A> for bool {
    #[inline(always)]
    fn from(variant: S5RETENTION_A) -> Self {
        variant as u8 != 0
    }
}
impl S5RETENTION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S5RETENTION_A {
        match self.bits {
            false => S5RETENTION_A::OFF,
            true => S5RETENTION_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S5RETENTION_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S5RETENTION_A::ON
    }
}
#[doc = "Field `S5RETENTION` writer - Keep retention on RAM section S5 when RAM section is off"]
pub type S5RETENTION_W<'a, const O: u8> = crate::BitWriter<'a, u32, POWER_SPEC, S5RETENTION_A, O>;
impl<'a, const O: u8> S5RETENTION_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S5RETENTION_A::OFF)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S5RETENTION_A::ON)
    }
}
#[doc = "Field `S6RETENTION` reader - Keep retention on RAM section S6 when RAM section is off"]
pub type S6RETENTION_R = crate::BitReader<S6RETENTION_A>;
#[doc = "Keep retention on RAM section S6 when RAM section is off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S6RETENTION_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: On"]
    ON = 1,
}
impl From<S6RETENTION_A> for bool {
    #[inline(always)]
    fn from(variant: S6RETENTION_A) -> Self {
        variant as u8 != 0
    }
}
impl S6RETENTION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S6RETENTION_A {
        match self.bits {
            false => S6RETENTION_A::OFF,
            true => S6RETENTION_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S6RETENTION_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S6RETENTION_A::ON
    }
}
#[doc = "Field `S6RETENTION` writer - Keep retention on RAM section S6 when RAM section is off"]
pub type S6RETENTION_W<'a, const O: u8> = crate::BitWriter<'a, u32, POWER_SPEC, S6RETENTION_A, O>;
impl<'a, const O: u8> S6RETENTION_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S6RETENTION_A::OFF)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S6RETENTION_A::ON)
    }
}
#[doc = "Field `S7RETENTION` reader - Keep retention on RAM section S7 when RAM section is off"]
pub type S7RETENTION_R = crate::BitReader<S7RETENTION_A>;
#[doc = "Keep retention on RAM section S7 when RAM section is off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S7RETENTION_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: On"]
    ON = 1,
}
impl From<S7RETENTION_A> for bool {
    #[inline(always)]
    fn from(variant: S7RETENTION_A) -> Self {
        variant as u8 != 0
    }
}
impl S7RETENTION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S7RETENTION_A {
        match self.bits {
            false => S7RETENTION_A::OFF,
            true => S7RETENTION_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S7RETENTION_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S7RETENTION_A::ON
    }
}
#[doc = "Field `S7RETENTION` writer - Keep retention on RAM section S7 when RAM section is off"]
pub type S7RETENTION_W<'a, const O: u8> = crate::BitWriter<'a, u32, POWER_SPEC, S7RETENTION_A, O>;
impl<'a, const O: u8> S7RETENTION_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S7RETENTION_A::OFF)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S7RETENTION_A::ON)
    }
}
#[doc = "Field `S8RETENTION` reader - Keep retention on RAM section S8 when RAM section is off"]
pub type S8RETENTION_R = crate::BitReader<S8RETENTION_A>;
#[doc = "Keep retention on RAM section S8 when RAM section is off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S8RETENTION_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: On"]
    ON = 1,
}
impl From<S8RETENTION_A> for bool {
    #[inline(always)]
    fn from(variant: S8RETENTION_A) -> Self {
        variant as u8 != 0
    }
}
impl S8RETENTION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S8RETENTION_A {
        match self.bits {
            false => S8RETENTION_A::OFF,
            true => S8RETENTION_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S8RETENTION_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S8RETENTION_A::ON
    }
}
#[doc = "Field `S8RETENTION` writer - Keep retention on RAM section S8 when RAM section is off"]
pub type S8RETENTION_W<'a, const O: u8> = crate::BitWriter<'a, u32, POWER_SPEC, S8RETENTION_A, O>;
impl<'a, const O: u8> S8RETENTION_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S8RETENTION_A::OFF)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S8RETENTION_A::ON)
    }
}
#[doc = "Field `S9RETENTION` reader - Keep retention on RAM section S9 when RAM section is off"]
pub type S9RETENTION_R = crate::BitReader<S9RETENTION_A>;
#[doc = "Keep retention on RAM section S9 when RAM section is off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S9RETENTION_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: On"]
    ON = 1,
}
impl From<S9RETENTION_A> for bool {
    #[inline(always)]
    fn from(variant: S9RETENTION_A) -> Self {
        variant as u8 != 0
    }
}
impl S9RETENTION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S9RETENTION_A {
        match self.bits {
            false => S9RETENTION_A::OFF,
            true => S9RETENTION_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S9RETENTION_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S9RETENTION_A::ON
    }
}
#[doc = "Field `S9RETENTION` writer - Keep retention on RAM section S9 when RAM section is off"]
pub type S9RETENTION_W<'a, const O: u8> = crate::BitWriter<'a, u32, POWER_SPEC, S9RETENTION_A, O>;
impl<'a, const O: u8> S9RETENTION_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S9RETENTION_A::OFF)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S9RETENTION_A::ON)
    }
}
#[doc = "Field `S10RETENTION` reader - Keep retention on RAM section S10 when RAM section is off"]
pub type S10RETENTION_R = crate::BitReader<S10RETENTION_A>;
#[doc = "Keep retention on RAM section S10 when RAM section is off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S10RETENTION_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: On"]
    ON = 1,
}
impl From<S10RETENTION_A> for bool {
    #[inline(always)]
    fn from(variant: S10RETENTION_A) -> Self {
        variant as u8 != 0
    }
}
impl S10RETENTION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S10RETENTION_A {
        match self.bits {
            false => S10RETENTION_A::OFF,
            true => S10RETENTION_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S10RETENTION_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S10RETENTION_A::ON
    }
}
#[doc = "Field `S10RETENTION` writer - Keep retention on RAM section S10 when RAM section is off"]
pub type S10RETENTION_W<'a, const O: u8> = crate::BitWriter<'a, u32, POWER_SPEC, S10RETENTION_A, O>;
impl<'a, const O: u8> S10RETENTION_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S10RETENTION_A::OFF)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S10RETENTION_A::ON)
    }
}
#[doc = "Field `S11RETENTION` reader - Keep retention on RAM section S11 when RAM section is off"]
pub type S11RETENTION_R = crate::BitReader<S11RETENTION_A>;
#[doc = "Keep retention on RAM section S11 when RAM section is off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S11RETENTION_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: On"]
    ON = 1,
}
impl From<S11RETENTION_A> for bool {
    #[inline(always)]
    fn from(variant: S11RETENTION_A) -> Self {
        variant as u8 != 0
    }
}
impl S11RETENTION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S11RETENTION_A {
        match self.bits {
            false => S11RETENTION_A::OFF,
            true => S11RETENTION_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S11RETENTION_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S11RETENTION_A::ON
    }
}
#[doc = "Field `S11RETENTION` writer - Keep retention on RAM section S11 when RAM section is off"]
pub type S11RETENTION_W<'a, const O: u8> = crate::BitWriter<'a, u32, POWER_SPEC, S11RETENTION_A, O>;
impl<'a, const O: u8> S11RETENTION_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S11RETENTION_A::OFF)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S11RETENTION_A::ON)
    }
}
#[doc = "Field `S12RETENTION` reader - Keep retention on RAM section S12 when RAM section is off"]
pub type S12RETENTION_R = crate::BitReader<S12RETENTION_A>;
#[doc = "Keep retention on RAM section S12 when RAM section is off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S12RETENTION_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: On"]
    ON = 1,
}
impl From<S12RETENTION_A> for bool {
    #[inline(always)]
    fn from(variant: S12RETENTION_A) -> Self {
        variant as u8 != 0
    }
}
impl S12RETENTION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S12RETENTION_A {
        match self.bits {
            false => S12RETENTION_A::OFF,
            true => S12RETENTION_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S12RETENTION_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S12RETENTION_A::ON
    }
}
#[doc = "Field `S12RETENTION` writer - Keep retention on RAM section S12 when RAM section is off"]
pub type S12RETENTION_W<'a, const O: u8> = crate::BitWriter<'a, u32, POWER_SPEC, S12RETENTION_A, O>;
impl<'a, const O: u8> S12RETENTION_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S12RETENTION_A::OFF)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S12RETENTION_A::ON)
    }
}
#[doc = "Field `S13RETENTION` reader - Keep retention on RAM section S13 when RAM section is off"]
pub type S13RETENTION_R = crate::BitReader<S13RETENTION_A>;
#[doc = "Keep retention on RAM section S13 when RAM section is off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S13RETENTION_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: On"]
    ON = 1,
}
impl From<S13RETENTION_A> for bool {
    #[inline(always)]
    fn from(variant: S13RETENTION_A) -> Self {
        variant as u8 != 0
    }
}
impl S13RETENTION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S13RETENTION_A {
        match self.bits {
            false => S13RETENTION_A::OFF,
            true => S13RETENTION_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S13RETENTION_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S13RETENTION_A::ON
    }
}
#[doc = "Field `S13RETENTION` writer - Keep retention on RAM section S13 when RAM section is off"]
pub type S13RETENTION_W<'a, const O: u8> = crate::BitWriter<'a, u32, POWER_SPEC, S13RETENTION_A, O>;
impl<'a, const O: u8> S13RETENTION_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S13RETENTION_A::OFF)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S13RETENTION_A::ON)
    }
}
#[doc = "Field `S14RETENTION` reader - Keep retention on RAM section S14 when RAM section is off"]
pub type S14RETENTION_R = crate::BitReader<S14RETENTION_A>;
#[doc = "Keep retention on RAM section S14 when RAM section is off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S14RETENTION_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: On"]
    ON = 1,
}
impl From<S14RETENTION_A> for bool {
    #[inline(always)]
    fn from(variant: S14RETENTION_A) -> Self {
        variant as u8 != 0
    }
}
impl S14RETENTION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S14RETENTION_A {
        match self.bits {
            false => S14RETENTION_A::OFF,
            true => S14RETENTION_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S14RETENTION_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S14RETENTION_A::ON
    }
}
#[doc = "Field `S14RETENTION` writer - Keep retention on RAM section S14 when RAM section is off"]
pub type S14RETENTION_W<'a, const O: u8> = crate::BitWriter<'a, u32, POWER_SPEC, S14RETENTION_A, O>;
impl<'a, const O: u8> S14RETENTION_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S14RETENTION_A::OFF)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S14RETENTION_A::ON)
    }
}
#[doc = "Field `S15RETENTION` reader - Keep retention on RAM section S15 when RAM section is off"]
pub type S15RETENTION_R = crate::BitReader<S15RETENTION_A>;
#[doc = "Keep retention on RAM section S15 when RAM section is off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S15RETENTION_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: On"]
    ON = 1,
}
impl From<S15RETENTION_A> for bool {
    #[inline(always)]
    fn from(variant: S15RETENTION_A) -> Self {
        variant as u8 != 0
    }
}
impl S15RETENTION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S15RETENTION_A {
        match self.bits {
            false => S15RETENTION_A::OFF,
            true => S15RETENTION_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S15RETENTION_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S15RETENTION_A::ON
    }
}
#[doc = "Field `S15RETENTION` writer - Keep retention on RAM section S15 when RAM section is off"]
pub type S15RETENTION_W<'a, const O: u8> = crate::BitWriter<'a, u32, POWER_SPEC, S15RETENTION_A, O>;
impl<'a, const O: u8> S15RETENTION_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S15RETENTION_A::OFF)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S15RETENTION_A::ON)
    }
}
impl R {
    #[doc = "Bit 0 - Keep RAM section S0 on or off in System ON mode."]
    #[inline(always)]
    pub fn s0power(&self) -> S0POWER_R {
        S0POWER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Keep RAM section S1 on or off in System ON mode."]
    #[inline(always)]
    pub fn s1power(&self) -> S1POWER_R {
        S1POWER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Keep RAM section S2 on or off in System ON mode."]
    #[inline(always)]
    pub fn s2power(&self) -> S2POWER_R {
        S2POWER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Keep RAM section S3 on or off in System ON mode."]
    #[inline(always)]
    pub fn s3power(&self) -> S3POWER_R {
        S3POWER_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Keep RAM section S4 on or off in System ON mode."]
    #[inline(always)]
    pub fn s4power(&self) -> S4POWER_R {
        S4POWER_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Keep RAM section S5 on or off in System ON mode."]
    #[inline(always)]
    pub fn s5power(&self) -> S5POWER_R {
        S5POWER_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Keep RAM section S6 on or off in System ON mode."]
    #[inline(always)]
    pub fn s6power(&self) -> S6POWER_R {
        S6POWER_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Keep RAM section S7 on or off in System ON mode."]
    #[inline(always)]
    pub fn s7power(&self) -> S7POWER_R {
        S7POWER_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Keep RAM section S8 on or off in System ON mode."]
    #[inline(always)]
    pub fn s8power(&self) -> S8POWER_R {
        S8POWER_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Keep RAM section S9 on or off in System ON mode."]
    #[inline(always)]
    pub fn s9power(&self) -> S9POWER_R {
        S9POWER_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Keep RAM section S10 on or off in System ON mode."]
    #[inline(always)]
    pub fn s10power(&self) -> S10POWER_R {
        S10POWER_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Keep RAM section S11 on or off in System ON mode."]
    #[inline(always)]
    pub fn s11power(&self) -> S11POWER_R {
        S11POWER_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Keep RAM section S12 on or off in System ON mode."]
    #[inline(always)]
    pub fn s12power(&self) -> S12POWER_R {
        S12POWER_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Keep RAM section S13 on or off in System ON mode."]
    #[inline(always)]
    pub fn s13power(&self) -> S13POWER_R {
        S13POWER_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Keep RAM section S14 on or off in System ON mode."]
    #[inline(always)]
    pub fn s14power(&self) -> S14POWER_R {
        S14POWER_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Keep RAM section S15 on or off in System ON mode."]
    #[inline(always)]
    pub fn s15power(&self) -> S15POWER_R {
        S15POWER_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Keep retention on RAM section S0 when RAM section is off"]
    #[inline(always)]
    pub fn s0retention(&self) -> S0RETENTION_R {
        S0RETENTION_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Keep retention on RAM section S1 when RAM section is off"]
    #[inline(always)]
    pub fn s1retention(&self) -> S1RETENTION_R {
        S1RETENTION_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Keep retention on RAM section S2 when RAM section is off"]
    #[inline(always)]
    pub fn s2retention(&self) -> S2RETENTION_R {
        S2RETENTION_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Keep retention on RAM section S3 when RAM section is off"]
    #[inline(always)]
    pub fn s3retention(&self) -> S3RETENTION_R {
        S3RETENTION_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Keep retention on RAM section S4 when RAM section is off"]
    #[inline(always)]
    pub fn s4retention(&self) -> S4RETENTION_R {
        S4RETENTION_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Keep retention on RAM section S5 when RAM section is off"]
    #[inline(always)]
    pub fn s5retention(&self) -> S5RETENTION_R {
        S5RETENTION_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Keep retention on RAM section S6 when RAM section is off"]
    #[inline(always)]
    pub fn s6retention(&self) -> S6RETENTION_R {
        S6RETENTION_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Keep retention on RAM section S7 when RAM section is off"]
    #[inline(always)]
    pub fn s7retention(&self) -> S7RETENTION_R {
        S7RETENTION_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Keep retention on RAM section S8 when RAM section is off"]
    #[inline(always)]
    pub fn s8retention(&self) -> S8RETENTION_R {
        S8RETENTION_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Keep retention on RAM section S9 when RAM section is off"]
    #[inline(always)]
    pub fn s9retention(&self) -> S9RETENTION_R {
        S9RETENTION_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Keep retention on RAM section S10 when RAM section is off"]
    #[inline(always)]
    pub fn s10retention(&self) -> S10RETENTION_R {
        S10RETENTION_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Keep retention on RAM section S11 when RAM section is off"]
    #[inline(always)]
    pub fn s11retention(&self) -> S11RETENTION_R {
        S11RETENTION_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Keep retention on RAM section S12 when RAM section is off"]
    #[inline(always)]
    pub fn s12retention(&self) -> S12RETENTION_R {
        S12RETENTION_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Keep retention on RAM section S13 when RAM section is off"]
    #[inline(always)]
    pub fn s13retention(&self) -> S13RETENTION_R {
        S13RETENTION_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Keep retention on RAM section S14 when RAM section is off"]
    #[inline(always)]
    pub fn s14retention(&self) -> S14RETENTION_R {
        S14RETENTION_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Keep retention on RAM section S15 when RAM section is off"]
    #[inline(always)]
    pub fn s15retention(&self) -> S15RETENTION_R {
        S15RETENTION_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Keep RAM section S0 on or off in System ON mode."]
    #[inline(always)]
    pub fn s0power(&mut self) -> S0POWER_W<0> {
        S0POWER_W::new(self)
    }
    #[doc = "Bit 1 - Keep RAM section S1 on or off in System ON mode."]
    #[inline(always)]
    pub fn s1power(&mut self) -> S1POWER_W<1> {
        S1POWER_W::new(self)
    }
    #[doc = "Bit 2 - Keep RAM section S2 on or off in System ON mode."]
    #[inline(always)]
    pub fn s2power(&mut self) -> S2POWER_W<2> {
        S2POWER_W::new(self)
    }
    #[doc = "Bit 3 - Keep RAM section S3 on or off in System ON mode."]
    #[inline(always)]
    pub fn s3power(&mut self) -> S3POWER_W<3> {
        S3POWER_W::new(self)
    }
    #[doc = "Bit 4 - Keep RAM section S4 on or off in System ON mode."]
    #[inline(always)]
    pub fn s4power(&mut self) -> S4POWER_W<4> {
        S4POWER_W::new(self)
    }
    #[doc = "Bit 5 - Keep RAM section S5 on or off in System ON mode."]
    #[inline(always)]
    pub fn s5power(&mut self) -> S5POWER_W<5> {
        S5POWER_W::new(self)
    }
    #[doc = "Bit 6 - Keep RAM section S6 on or off in System ON mode."]
    #[inline(always)]
    pub fn s6power(&mut self) -> S6POWER_W<6> {
        S6POWER_W::new(self)
    }
    #[doc = "Bit 7 - Keep RAM section S7 on or off in System ON mode."]
    #[inline(always)]
    pub fn s7power(&mut self) -> S7POWER_W<7> {
        S7POWER_W::new(self)
    }
    #[doc = "Bit 8 - Keep RAM section S8 on or off in System ON mode."]
    #[inline(always)]
    pub fn s8power(&mut self) -> S8POWER_W<8> {
        S8POWER_W::new(self)
    }
    #[doc = "Bit 9 - Keep RAM section S9 on or off in System ON mode."]
    #[inline(always)]
    pub fn s9power(&mut self) -> S9POWER_W<9> {
        S9POWER_W::new(self)
    }
    #[doc = "Bit 10 - Keep RAM section S10 on or off in System ON mode."]
    #[inline(always)]
    pub fn s10power(&mut self) -> S10POWER_W<10> {
        S10POWER_W::new(self)
    }
    #[doc = "Bit 11 - Keep RAM section S11 on or off in System ON mode."]
    #[inline(always)]
    pub fn s11power(&mut self) -> S11POWER_W<11> {
        S11POWER_W::new(self)
    }
    #[doc = "Bit 12 - Keep RAM section S12 on or off in System ON mode."]
    #[inline(always)]
    pub fn s12power(&mut self) -> S12POWER_W<12> {
        S12POWER_W::new(self)
    }
    #[doc = "Bit 13 - Keep RAM section S13 on or off in System ON mode."]
    #[inline(always)]
    pub fn s13power(&mut self) -> S13POWER_W<13> {
        S13POWER_W::new(self)
    }
    #[doc = "Bit 14 - Keep RAM section S14 on or off in System ON mode."]
    #[inline(always)]
    pub fn s14power(&mut self) -> S14POWER_W<14> {
        S14POWER_W::new(self)
    }
    #[doc = "Bit 15 - Keep RAM section S15 on or off in System ON mode."]
    #[inline(always)]
    pub fn s15power(&mut self) -> S15POWER_W<15> {
        S15POWER_W::new(self)
    }
    #[doc = "Bit 16 - Keep retention on RAM section S0 when RAM section is off"]
    #[inline(always)]
    pub fn s0retention(&mut self) -> S0RETENTION_W<16> {
        S0RETENTION_W::new(self)
    }
    #[doc = "Bit 17 - Keep retention on RAM section S1 when RAM section is off"]
    #[inline(always)]
    pub fn s1retention(&mut self) -> S1RETENTION_W<17> {
        S1RETENTION_W::new(self)
    }
    #[doc = "Bit 18 - Keep retention on RAM section S2 when RAM section is off"]
    #[inline(always)]
    pub fn s2retention(&mut self) -> S2RETENTION_W<18> {
        S2RETENTION_W::new(self)
    }
    #[doc = "Bit 19 - Keep retention on RAM section S3 when RAM section is off"]
    #[inline(always)]
    pub fn s3retention(&mut self) -> S3RETENTION_W<19> {
        S3RETENTION_W::new(self)
    }
    #[doc = "Bit 20 - Keep retention on RAM section S4 when RAM section is off"]
    #[inline(always)]
    pub fn s4retention(&mut self) -> S4RETENTION_W<20> {
        S4RETENTION_W::new(self)
    }
    #[doc = "Bit 21 - Keep retention on RAM section S5 when RAM section is off"]
    #[inline(always)]
    pub fn s5retention(&mut self) -> S5RETENTION_W<21> {
        S5RETENTION_W::new(self)
    }
    #[doc = "Bit 22 - Keep retention on RAM section S6 when RAM section is off"]
    #[inline(always)]
    pub fn s6retention(&mut self) -> S6RETENTION_W<22> {
        S6RETENTION_W::new(self)
    }
    #[doc = "Bit 23 - Keep retention on RAM section S7 when RAM section is off"]
    #[inline(always)]
    pub fn s7retention(&mut self) -> S7RETENTION_W<23> {
        S7RETENTION_W::new(self)
    }
    #[doc = "Bit 24 - Keep retention on RAM section S8 when RAM section is off"]
    #[inline(always)]
    pub fn s8retention(&mut self) -> S8RETENTION_W<24> {
        S8RETENTION_W::new(self)
    }
    #[doc = "Bit 25 - Keep retention on RAM section S9 when RAM section is off"]
    #[inline(always)]
    pub fn s9retention(&mut self) -> S9RETENTION_W<25> {
        S9RETENTION_W::new(self)
    }
    #[doc = "Bit 26 - Keep retention on RAM section S10 when RAM section is off"]
    #[inline(always)]
    pub fn s10retention(&mut self) -> S10RETENTION_W<26> {
        S10RETENTION_W::new(self)
    }
    #[doc = "Bit 27 - Keep retention on RAM section S11 when RAM section is off"]
    #[inline(always)]
    pub fn s11retention(&mut self) -> S11RETENTION_W<27> {
        S11RETENTION_W::new(self)
    }
    #[doc = "Bit 28 - Keep retention on RAM section S12 when RAM section is off"]
    #[inline(always)]
    pub fn s12retention(&mut self) -> S12RETENTION_W<28> {
        S12RETENTION_W::new(self)
    }
    #[doc = "Bit 29 - Keep retention on RAM section S13 when RAM section is off"]
    #[inline(always)]
    pub fn s13retention(&mut self) -> S13RETENTION_W<29> {
        S13RETENTION_W::new(self)
    }
    #[doc = "Bit 30 - Keep retention on RAM section S14 when RAM section is off"]
    #[inline(always)]
    pub fn s14retention(&mut self) -> S14RETENTION_W<30> {
        S14RETENTION_W::new(self)
    }
    #[doc = "Bit 31 - Keep retention on RAM section S15 when RAM section is off"]
    #[inline(always)]
    pub fn s15retention(&mut self) -> S15RETENTION_W<31> {
        S15RETENTION_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description cluster\\[n\\]: RAMn power control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [power](index.html) module"]
pub struct POWER_SPEC;
impl crate::RegisterSpec for POWER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [power::R](R) reader structure"]
impl crate::Readable for POWER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [power::W](W) writer structure"]
impl crate::Writable for POWER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets POWER to value 0xffff"]
impl crate::Resettable for POWER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
