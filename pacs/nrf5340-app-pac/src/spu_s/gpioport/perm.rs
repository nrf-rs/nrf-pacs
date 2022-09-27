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
#[doc = "Field `PIN0` reader - Select secure attribute attribute for PIN 0."]
pub type PIN0_R = crate::BitReader<PIN0_A>;
#[doc = "Select secure attribute attribute for PIN 0.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN0_A {
    #[doc = "1: Pin 0 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Pin 0 has its non-secure attribute set"]
    NON_SECURE = 0,
}
impl From<PIN0_A> for bool {
    #[inline(always)]
    fn from(variant: PIN0_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN0_A {
        match self.bits {
            true => PIN0_A::SECURE,
            false => PIN0_A::NON_SECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == PIN0_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NON_SECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == PIN0_A::NON_SECURE
    }
}
#[doc = "Field `PIN0` writer - Select secure attribute attribute for PIN 0."]
pub type PIN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERM_SPEC, PIN0_A, O>;
impl<'a, const O: u8> PIN0_W<'a, O> {
    #[doc = "Pin 0 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN0_A::SECURE)
    }
    #[doc = "Pin 0 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN0_A::NON_SECURE)
    }
}
#[doc = "Field `PIN1` reader - Select secure attribute attribute for PIN 1."]
pub type PIN1_R = crate::BitReader<PIN1_A>;
#[doc = "Select secure attribute attribute for PIN 1.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN1_A {
    #[doc = "1: Pin 1 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Pin 1 has its non-secure attribute set"]
    NON_SECURE = 0,
}
impl From<PIN1_A> for bool {
    #[inline(always)]
    fn from(variant: PIN1_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN1_A {
        match self.bits {
            true => PIN1_A::SECURE,
            false => PIN1_A::NON_SECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == PIN1_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NON_SECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == PIN1_A::NON_SECURE
    }
}
#[doc = "Field `PIN1` writer - Select secure attribute attribute for PIN 1."]
pub type PIN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERM_SPEC, PIN1_A, O>;
impl<'a, const O: u8> PIN1_W<'a, O> {
    #[doc = "Pin 1 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN1_A::SECURE)
    }
    #[doc = "Pin 1 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN1_A::NON_SECURE)
    }
}
#[doc = "Field `PIN2` reader - Select secure attribute attribute for PIN 2."]
pub type PIN2_R = crate::BitReader<PIN2_A>;
#[doc = "Select secure attribute attribute for PIN 2.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN2_A {
    #[doc = "1: Pin 2 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Pin 2 has its non-secure attribute set"]
    NON_SECURE = 0,
}
impl From<PIN2_A> for bool {
    #[inline(always)]
    fn from(variant: PIN2_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN2_A {
        match self.bits {
            true => PIN2_A::SECURE,
            false => PIN2_A::NON_SECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == PIN2_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NON_SECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == PIN2_A::NON_SECURE
    }
}
#[doc = "Field `PIN2` writer - Select secure attribute attribute for PIN 2."]
pub type PIN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERM_SPEC, PIN2_A, O>;
impl<'a, const O: u8> PIN2_W<'a, O> {
    #[doc = "Pin 2 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN2_A::SECURE)
    }
    #[doc = "Pin 2 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN2_A::NON_SECURE)
    }
}
#[doc = "Field `PIN3` reader - Select secure attribute attribute for PIN 3."]
pub type PIN3_R = crate::BitReader<PIN3_A>;
#[doc = "Select secure attribute attribute for PIN 3.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN3_A {
    #[doc = "1: Pin 3 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Pin 3 has its non-secure attribute set"]
    NON_SECURE = 0,
}
impl From<PIN3_A> for bool {
    #[inline(always)]
    fn from(variant: PIN3_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN3_A {
        match self.bits {
            true => PIN3_A::SECURE,
            false => PIN3_A::NON_SECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == PIN3_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NON_SECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == PIN3_A::NON_SECURE
    }
}
#[doc = "Field `PIN3` writer - Select secure attribute attribute for PIN 3."]
pub type PIN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERM_SPEC, PIN3_A, O>;
impl<'a, const O: u8> PIN3_W<'a, O> {
    #[doc = "Pin 3 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN3_A::SECURE)
    }
    #[doc = "Pin 3 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN3_A::NON_SECURE)
    }
}
#[doc = "Field `PIN4` reader - Select secure attribute attribute for PIN 4."]
pub type PIN4_R = crate::BitReader<PIN4_A>;
#[doc = "Select secure attribute attribute for PIN 4.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN4_A {
    #[doc = "1: Pin 4 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Pin 4 has its non-secure attribute set"]
    NON_SECURE = 0,
}
impl From<PIN4_A> for bool {
    #[inline(always)]
    fn from(variant: PIN4_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN4_A {
        match self.bits {
            true => PIN4_A::SECURE,
            false => PIN4_A::NON_SECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == PIN4_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NON_SECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == PIN4_A::NON_SECURE
    }
}
#[doc = "Field `PIN4` writer - Select secure attribute attribute for PIN 4."]
pub type PIN4_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERM_SPEC, PIN4_A, O>;
impl<'a, const O: u8> PIN4_W<'a, O> {
    #[doc = "Pin 4 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN4_A::SECURE)
    }
    #[doc = "Pin 4 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN4_A::NON_SECURE)
    }
}
#[doc = "Field `PIN5` reader - Select secure attribute attribute for PIN 5."]
pub type PIN5_R = crate::BitReader<PIN5_A>;
#[doc = "Select secure attribute attribute for PIN 5.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN5_A {
    #[doc = "1: Pin 5 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Pin 5 has its non-secure attribute set"]
    NON_SECURE = 0,
}
impl From<PIN5_A> for bool {
    #[inline(always)]
    fn from(variant: PIN5_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN5_A {
        match self.bits {
            true => PIN5_A::SECURE,
            false => PIN5_A::NON_SECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == PIN5_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NON_SECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == PIN5_A::NON_SECURE
    }
}
#[doc = "Field `PIN5` writer - Select secure attribute attribute for PIN 5."]
pub type PIN5_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERM_SPEC, PIN5_A, O>;
impl<'a, const O: u8> PIN5_W<'a, O> {
    #[doc = "Pin 5 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN5_A::SECURE)
    }
    #[doc = "Pin 5 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN5_A::NON_SECURE)
    }
}
#[doc = "Field `PIN6` reader - Select secure attribute attribute for PIN 6."]
pub type PIN6_R = crate::BitReader<PIN6_A>;
#[doc = "Select secure attribute attribute for PIN 6.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN6_A {
    #[doc = "1: Pin 6 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Pin 6 has its non-secure attribute set"]
    NON_SECURE = 0,
}
impl From<PIN6_A> for bool {
    #[inline(always)]
    fn from(variant: PIN6_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN6_A {
        match self.bits {
            true => PIN6_A::SECURE,
            false => PIN6_A::NON_SECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == PIN6_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NON_SECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == PIN6_A::NON_SECURE
    }
}
#[doc = "Field `PIN6` writer - Select secure attribute attribute for PIN 6."]
pub type PIN6_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERM_SPEC, PIN6_A, O>;
impl<'a, const O: u8> PIN6_W<'a, O> {
    #[doc = "Pin 6 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN6_A::SECURE)
    }
    #[doc = "Pin 6 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN6_A::NON_SECURE)
    }
}
#[doc = "Field `PIN7` reader - Select secure attribute attribute for PIN 7."]
pub type PIN7_R = crate::BitReader<PIN7_A>;
#[doc = "Select secure attribute attribute for PIN 7.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN7_A {
    #[doc = "1: Pin 7 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Pin 7 has its non-secure attribute set"]
    NON_SECURE = 0,
}
impl From<PIN7_A> for bool {
    #[inline(always)]
    fn from(variant: PIN7_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN7_A {
        match self.bits {
            true => PIN7_A::SECURE,
            false => PIN7_A::NON_SECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == PIN7_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NON_SECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == PIN7_A::NON_SECURE
    }
}
#[doc = "Field `PIN7` writer - Select secure attribute attribute for PIN 7."]
pub type PIN7_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERM_SPEC, PIN7_A, O>;
impl<'a, const O: u8> PIN7_W<'a, O> {
    #[doc = "Pin 7 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN7_A::SECURE)
    }
    #[doc = "Pin 7 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN7_A::NON_SECURE)
    }
}
#[doc = "Field `PIN8` reader - Select secure attribute attribute for PIN 8."]
pub type PIN8_R = crate::BitReader<PIN8_A>;
#[doc = "Select secure attribute attribute for PIN 8.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN8_A {
    #[doc = "1: Pin 8 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Pin 8 has its non-secure attribute set"]
    NON_SECURE = 0,
}
impl From<PIN8_A> for bool {
    #[inline(always)]
    fn from(variant: PIN8_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN8_A {
        match self.bits {
            true => PIN8_A::SECURE,
            false => PIN8_A::NON_SECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == PIN8_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NON_SECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == PIN8_A::NON_SECURE
    }
}
#[doc = "Field `PIN8` writer - Select secure attribute attribute for PIN 8."]
pub type PIN8_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERM_SPEC, PIN8_A, O>;
impl<'a, const O: u8> PIN8_W<'a, O> {
    #[doc = "Pin 8 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN8_A::SECURE)
    }
    #[doc = "Pin 8 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN8_A::NON_SECURE)
    }
}
#[doc = "Field `PIN9` reader - Select secure attribute attribute for PIN 9."]
pub type PIN9_R = crate::BitReader<PIN9_A>;
#[doc = "Select secure attribute attribute for PIN 9.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN9_A {
    #[doc = "1: Pin 9 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Pin 9 has its non-secure attribute set"]
    NON_SECURE = 0,
}
impl From<PIN9_A> for bool {
    #[inline(always)]
    fn from(variant: PIN9_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN9_A {
        match self.bits {
            true => PIN9_A::SECURE,
            false => PIN9_A::NON_SECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == PIN9_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NON_SECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == PIN9_A::NON_SECURE
    }
}
#[doc = "Field `PIN9` writer - Select secure attribute attribute for PIN 9."]
pub type PIN9_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERM_SPEC, PIN9_A, O>;
impl<'a, const O: u8> PIN9_W<'a, O> {
    #[doc = "Pin 9 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN9_A::SECURE)
    }
    #[doc = "Pin 9 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN9_A::NON_SECURE)
    }
}
#[doc = "Field `PIN10` reader - Select secure attribute attribute for PIN 10."]
pub type PIN10_R = crate::BitReader<PIN10_A>;
#[doc = "Select secure attribute attribute for PIN 10.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN10_A {
    #[doc = "1: Pin 10 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Pin 10 has its non-secure attribute set"]
    NON_SECURE = 0,
}
impl From<PIN10_A> for bool {
    #[inline(always)]
    fn from(variant: PIN10_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN10_A {
        match self.bits {
            true => PIN10_A::SECURE,
            false => PIN10_A::NON_SECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == PIN10_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NON_SECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == PIN10_A::NON_SECURE
    }
}
#[doc = "Field `PIN10` writer - Select secure attribute attribute for PIN 10."]
pub type PIN10_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERM_SPEC, PIN10_A, O>;
impl<'a, const O: u8> PIN10_W<'a, O> {
    #[doc = "Pin 10 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN10_A::SECURE)
    }
    #[doc = "Pin 10 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN10_A::NON_SECURE)
    }
}
#[doc = "Field `PIN11` reader - Select secure attribute attribute for PIN 11."]
pub type PIN11_R = crate::BitReader<PIN11_A>;
#[doc = "Select secure attribute attribute for PIN 11.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN11_A {
    #[doc = "1: Pin 11 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Pin 11 has its non-secure attribute set"]
    NON_SECURE = 0,
}
impl From<PIN11_A> for bool {
    #[inline(always)]
    fn from(variant: PIN11_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN11_A {
        match self.bits {
            true => PIN11_A::SECURE,
            false => PIN11_A::NON_SECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == PIN11_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NON_SECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == PIN11_A::NON_SECURE
    }
}
#[doc = "Field `PIN11` writer - Select secure attribute attribute for PIN 11."]
pub type PIN11_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERM_SPEC, PIN11_A, O>;
impl<'a, const O: u8> PIN11_W<'a, O> {
    #[doc = "Pin 11 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN11_A::SECURE)
    }
    #[doc = "Pin 11 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN11_A::NON_SECURE)
    }
}
#[doc = "Field `PIN12` reader - Select secure attribute attribute for PIN 12."]
pub type PIN12_R = crate::BitReader<PIN12_A>;
#[doc = "Select secure attribute attribute for PIN 12.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN12_A {
    #[doc = "1: Pin 12 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Pin 12 has its non-secure attribute set"]
    NON_SECURE = 0,
}
impl From<PIN12_A> for bool {
    #[inline(always)]
    fn from(variant: PIN12_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN12_A {
        match self.bits {
            true => PIN12_A::SECURE,
            false => PIN12_A::NON_SECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == PIN12_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NON_SECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == PIN12_A::NON_SECURE
    }
}
#[doc = "Field `PIN12` writer - Select secure attribute attribute for PIN 12."]
pub type PIN12_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERM_SPEC, PIN12_A, O>;
impl<'a, const O: u8> PIN12_W<'a, O> {
    #[doc = "Pin 12 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN12_A::SECURE)
    }
    #[doc = "Pin 12 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN12_A::NON_SECURE)
    }
}
#[doc = "Field `PIN13` reader - Select secure attribute attribute for PIN 13."]
pub type PIN13_R = crate::BitReader<PIN13_A>;
#[doc = "Select secure attribute attribute for PIN 13.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN13_A {
    #[doc = "1: Pin 13 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Pin 13 has its non-secure attribute set"]
    NON_SECURE = 0,
}
impl From<PIN13_A> for bool {
    #[inline(always)]
    fn from(variant: PIN13_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN13_A {
        match self.bits {
            true => PIN13_A::SECURE,
            false => PIN13_A::NON_SECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == PIN13_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NON_SECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == PIN13_A::NON_SECURE
    }
}
#[doc = "Field `PIN13` writer - Select secure attribute attribute for PIN 13."]
pub type PIN13_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERM_SPEC, PIN13_A, O>;
impl<'a, const O: u8> PIN13_W<'a, O> {
    #[doc = "Pin 13 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN13_A::SECURE)
    }
    #[doc = "Pin 13 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN13_A::NON_SECURE)
    }
}
#[doc = "Field `PIN14` reader - Select secure attribute attribute for PIN 14."]
pub type PIN14_R = crate::BitReader<PIN14_A>;
#[doc = "Select secure attribute attribute for PIN 14.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN14_A {
    #[doc = "1: Pin 14 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Pin 14 has its non-secure attribute set"]
    NON_SECURE = 0,
}
impl From<PIN14_A> for bool {
    #[inline(always)]
    fn from(variant: PIN14_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN14_A {
        match self.bits {
            true => PIN14_A::SECURE,
            false => PIN14_A::NON_SECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == PIN14_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NON_SECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == PIN14_A::NON_SECURE
    }
}
#[doc = "Field `PIN14` writer - Select secure attribute attribute for PIN 14."]
pub type PIN14_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERM_SPEC, PIN14_A, O>;
impl<'a, const O: u8> PIN14_W<'a, O> {
    #[doc = "Pin 14 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN14_A::SECURE)
    }
    #[doc = "Pin 14 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN14_A::NON_SECURE)
    }
}
#[doc = "Field `PIN15` reader - Select secure attribute attribute for PIN 15."]
pub type PIN15_R = crate::BitReader<PIN15_A>;
#[doc = "Select secure attribute attribute for PIN 15.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN15_A {
    #[doc = "1: Pin 15 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Pin 15 has its non-secure attribute set"]
    NON_SECURE = 0,
}
impl From<PIN15_A> for bool {
    #[inline(always)]
    fn from(variant: PIN15_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN15_A {
        match self.bits {
            true => PIN15_A::SECURE,
            false => PIN15_A::NON_SECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == PIN15_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NON_SECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == PIN15_A::NON_SECURE
    }
}
#[doc = "Field `PIN15` writer - Select secure attribute attribute for PIN 15."]
pub type PIN15_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERM_SPEC, PIN15_A, O>;
impl<'a, const O: u8> PIN15_W<'a, O> {
    #[doc = "Pin 15 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN15_A::SECURE)
    }
    #[doc = "Pin 15 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN15_A::NON_SECURE)
    }
}
#[doc = "Field `PIN16` reader - Select secure attribute attribute for PIN 16."]
pub type PIN16_R = crate::BitReader<PIN16_A>;
#[doc = "Select secure attribute attribute for PIN 16.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN16_A {
    #[doc = "1: Pin 16 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Pin 16 has its non-secure attribute set"]
    NON_SECURE = 0,
}
impl From<PIN16_A> for bool {
    #[inline(always)]
    fn from(variant: PIN16_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN16_A {
        match self.bits {
            true => PIN16_A::SECURE,
            false => PIN16_A::NON_SECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == PIN16_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NON_SECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == PIN16_A::NON_SECURE
    }
}
#[doc = "Field `PIN16` writer - Select secure attribute attribute for PIN 16."]
pub type PIN16_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERM_SPEC, PIN16_A, O>;
impl<'a, const O: u8> PIN16_W<'a, O> {
    #[doc = "Pin 16 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN16_A::SECURE)
    }
    #[doc = "Pin 16 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN16_A::NON_SECURE)
    }
}
#[doc = "Field `PIN17` reader - Select secure attribute attribute for PIN 17."]
pub type PIN17_R = crate::BitReader<PIN17_A>;
#[doc = "Select secure attribute attribute for PIN 17.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN17_A {
    #[doc = "1: Pin 17 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Pin 17 has its non-secure attribute set"]
    NON_SECURE = 0,
}
impl From<PIN17_A> for bool {
    #[inline(always)]
    fn from(variant: PIN17_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN17_A {
        match self.bits {
            true => PIN17_A::SECURE,
            false => PIN17_A::NON_SECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == PIN17_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NON_SECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == PIN17_A::NON_SECURE
    }
}
#[doc = "Field `PIN17` writer - Select secure attribute attribute for PIN 17."]
pub type PIN17_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERM_SPEC, PIN17_A, O>;
impl<'a, const O: u8> PIN17_W<'a, O> {
    #[doc = "Pin 17 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN17_A::SECURE)
    }
    #[doc = "Pin 17 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN17_A::NON_SECURE)
    }
}
#[doc = "Field `PIN18` reader - Select secure attribute attribute for PIN 18."]
pub type PIN18_R = crate::BitReader<PIN18_A>;
#[doc = "Select secure attribute attribute for PIN 18.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN18_A {
    #[doc = "1: Pin 18 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Pin 18 has its non-secure attribute set"]
    NON_SECURE = 0,
}
impl From<PIN18_A> for bool {
    #[inline(always)]
    fn from(variant: PIN18_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN18_A {
        match self.bits {
            true => PIN18_A::SECURE,
            false => PIN18_A::NON_SECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == PIN18_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NON_SECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == PIN18_A::NON_SECURE
    }
}
#[doc = "Field `PIN18` writer - Select secure attribute attribute for PIN 18."]
pub type PIN18_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERM_SPEC, PIN18_A, O>;
impl<'a, const O: u8> PIN18_W<'a, O> {
    #[doc = "Pin 18 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN18_A::SECURE)
    }
    #[doc = "Pin 18 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN18_A::NON_SECURE)
    }
}
#[doc = "Field `PIN19` reader - Select secure attribute attribute for PIN 19."]
pub type PIN19_R = crate::BitReader<PIN19_A>;
#[doc = "Select secure attribute attribute for PIN 19.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN19_A {
    #[doc = "1: Pin 19 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Pin 19 has its non-secure attribute set"]
    NON_SECURE = 0,
}
impl From<PIN19_A> for bool {
    #[inline(always)]
    fn from(variant: PIN19_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN19_A {
        match self.bits {
            true => PIN19_A::SECURE,
            false => PIN19_A::NON_SECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == PIN19_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NON_SECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == PIN19_A::NON_SECURE
    }
}
#[doc = "Field `PIN19` writer - Select secure attribute attribute for PIN 19."]
pub type PIN19_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERM_SPEC, PIN19_A, O>;
impl<'a, const O: u8> PIN19_W<'a, O> {
    #[doc = "Pin 19 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN19_A::SECURE)
    }
    #[doc = "Pin 19 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN19_A::NON_SECURE)
    }
}
#[doc = "Field `PIN20` reader - Select secure attribute attribute for PIN 20."]
pub type PIN20_R = crate::BitReader<PIN20_A>;
#[doc = "Select secure attribute attribute for PIN 20.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN20_A {
    #[doc = "1: Pin 20 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Pin 20 has its non-secure attribute set"]
    NON_SECURE = 0,
}
impl From<PIN20_A> for bool {
    #[inline(always)]
    fn from(variant: PIN20_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN20_A {
        match self.bits {
            true => PIN20_A::SECURE,
            false => PIN20_A::NON_SECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == PIN20_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NON_SECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == PIN20_A::NON_SECURE
    }
}
#[doc = "Field `PIN20` writer - Select secure attribute attribute for PIN 20."]
pub type PIN20_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERM_SPEC, PIN20_A, O>;
impl<'a, const O: u8> PIN20_W<'a, O> {
    #[doc = "Pin 20 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN20_A::SECURE)
    }
    #[doc = "Pin 20 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN20_A::NON_SECURE)
    }
}
#[doc = "Field `PIN21` reader - Select secure attribute attribute for PIN 21."]
pub type PIN21_R = crate::BitReader<PIN21_A>;
#[doc = "Select secure attribute attribute for PIN 21.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN21_A {
    #[doc = "1: Pin 21 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Pin 21 has its non-secure attribute set"]
    NON_SECURE = 0,
}
impl From<PIN21_A> for bool {
    #[inline(always)]
    fn from(variant: PIN21_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN21_A {
        match self.bits {
            true => PIN21_A::SECURE,
            false => PIN21_A::NON_SECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == PIN21_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NON_SECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == PIN21_A::NON_SECURE
    }
}
#[doc = "Field `PIN21` writer - Select secure attribute attribute for PIN 21."]
pub type PIN21_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERM_SPEC, PIN21_A, O>;
impl<'a, const O: u8> PIN21_W<'a, O> {
    #[doc = "Pin 21 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN21_A::SECURE)
    }
    #[doc = "Pin 21 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN21_A::NON_SECURE)
    }
}
#[doc = "Field `PIN22` reader - Select secure attribute attribute for PIN 22."]
pub type PIN22_R = crate::BitReader<PIN22_A>;
#[doc = "Select secure attribute attribute for PIN 22.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN22_A {
    #[doc = "1: Pin 22 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Pin 22 has its non-secure attribute set"]
    NON_SECURE = 0,
}
impl From<PIN22_A> for bool {
    #[inline(always)]
    fn from(variant: PIN22_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN22_A {
        match self.bits {
            true => PIN22_A::SECURE,
            false => PIN22_A::NON_SECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == PIN22_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NON_SECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == PIN22_A::NON_SECURE
    }
}
#[doc = "Field `PIN22` writer - Select secure attribute attribute for PIN 22."]
pub type PIN22_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERM_SPEC, PIN22_A, O>;
impl<'a, const O: u8> PIN22_W<'a, O> {
    #[doc = "Pin 22 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN22_A::SECURE)
    }
    #[doc = "Pin 22 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN22_A::NON_SECURE)
    }
}
#[doc = "Field `PIN23` reader - Select secure attribute attribute for PIN 23."]
pub type PIN23_R = crate::BitReader<PIN23_A>;
#[doc = "Select secure attribute attribute for PIN 23.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN23_A {
    #[doc = "1: Pin 23 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Pin 23 has its non-secure attribute set"]
    NON_SECURE = 0,
}
impl From<PIN23_A> for bool {
    #[inline(always)]
    fn from(variant: PIN23_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN23_A {
        match self.bits {
            true => PIN23_A::SECURE,
            false => PIN23_A::NON_SECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == PIN23_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NON_SECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == PIN23_A::NON_SECURE
    }
}
#[doc = "Field `PIN23` writer - Select secure attribute attribute for PIN 23."]
pub type PIN23_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERM_SPEC, PIN23_A, O>;
impl<'a, const O: u8> PIN23_W<'a, O> {
    #[doc = "Pin 23 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN23_A::SECURE)
    }
    #[doc = "Pin 23 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN23_A::NON_SECURE)
    }
}
#[doc = "Field `PIN24` reader - Select secure attribute attribute for PIN 24."]
pub type PIN24_R = crate::BitReader<PIN24_A>;
#[doc = "Select secure attribute attribute for PIN 24.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN24_A {
    #[doc = "1: Pin 24 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Pin 24 has its non-secure attribute set"]
    NON_SECURE = 0,
}
impl From<PIN24_A> for bool {
    #[inline(always)]
    fn from(variant: PIN24_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN24_A {
        match self.bits {
            true => PIN24_A::SECURE,
            false => PIN24_A::NON_SECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == PIN24_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NON_SECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == PIN24_A::NON_SECURE
    }
}
#[doc = "Field `PIN24` writer - Select secure attribute attribute for PIN 24."]
pub type PIN24_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERM_SPEC, PIN24_A, O>;
impl<'a, const O: u8> PIN24_W<'a, O> {
    #[doc = "Pin 24 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN24_A::SECURE)
    }
    #[doc = "Pin 24 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN24_A::NON_SECURE)
    }
}
#[doc = "Field `PIN25` reader - Select secure attribute attribute for PIN 25."]
pub type PIN25_R = crate::BitReader<PIN25_A>;
#[doc = "Select secure attribute attribute for PIN 25.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN25_A {
    #[doc = "1: Pin 25 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Pin 25 has its non-secure attribute set"]
    NON_SECURE = 0,
}
impl From<PIN25_A> for bool {
    #[inline(always)]
    fn from(variant: PIN25_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN25_A {
        match self.bits {
            true => PIN25_A::SECURE,
            false => PIN25_A::NON_SECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == PIN25_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NON_SECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == PIN25_A::NON_SECURE
    }
}
#[doc = "Field `PIN25` writer - Select secure attribute attribute for PIN 25."]
pub type PIN25_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERM_SPEC, PIN25_A, O>;
impl<'a, const O: u8> PIN25_W<'a, O> {
    #[doc = "Pin 25 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN25_A::SECURE)
    }
    #[doc = "Pin 25 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN25_A::NON_SECURE)
    }
}
#[doc = "Field `PIN26` reader - Select secure attribute attribute for PIN 26."]
pub type PIN26_R = crate::BitReader<PIN26_A>;
#[doc = "Select secure attribute attribute for PIN 26.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN26_A {
    #[doc = "1: Pin 26 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Pin 26 has its non-secure attribute set"]
    NON_SECURE = 0,
}
impl From<PIN26_A> for bool {
    #[inline(always)]
    fn from(variant: PIN26_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN26_A {
        match self.bits {
            true => PIN26_A::SECURE,
            false => PIN26_A::NON_SECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == PIN26_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NON_SECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == PIN26_A::NON_SECURE
    }
}
#[doc = "Field `PIN26` writer - Select secure attribute attribute for PIN 26."]
pub type PIN26_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERM_SPEC, PIN26_A, O>;
impl<'a, const O: u8> PIN26_W<'a, O> {
    #[doc = "Pin 26 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN26_A::SECURE)
    }
    #[doc = "Pin 26 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN26_A::NON_SECURE)
    }
}
#[doc = "Field `PIN27` reader - Select secure attribute attribute for PIN 27."]
pub type PIN27_R = crate::BitReader<PIN27_A>;
#[doc = "Select secure attribute attribute for PIN 27.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN27_A {
    #[doc = "1: Pin 27 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Pin 27 has its non-secure attribute set"]
    NON_SECURE = 0,
}
impl From<PIN27_A> for bool {
    #[inline(always)]
    fn from(variant: PIN27_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN27_A {
        match self.bits {
            true => PIN27_A::SECURE,
            false => PIN27_A::NON_SECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == PIN27_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NON_SECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == PIN27_A::NON_SECURE
    }
}
#[doc = "Field `PIN27` writer - Select secure attribute attribute for PIN 27."]
pub type PIN27_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERM_SPEC, PIN27_A, O>;
impl<'a, const O: u8> PIN27_W<'a, O> {
    #[doc = "Pin 27 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN27_A::SECURE)
    }
    #[doc = "Pin 27 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN27_A::NON_SECURE)
    }
}
#[doc = "Field `PIN28` reader - Select secure attribute attribute for PIN 28."]
pub type PIN28_R = crate::BitReader<PIN28_A>;
#[doc = "Select secure attribute attribute for PIN 28.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN28_A {
    #[doc = "1: Pin 28 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Pin 28 has its non-secure attribute set"]
    NON_SECURE = 0,
}
impl From<PIN28_A> for bool {
    #[inline(always)]
    fn from(variant: PIN28_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN28_A {
        match self.bits {
            true => PIN28_A::SECURE,
            false => PIN28_A::NON_SECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == PIN28_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NON_SECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == PIN28_A::NON_SECURE
    }
}
#[doc = "Field `PIN28` writer - Select secure attribute attribute for PIN 28."]
pub type PIN28_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERM_SPEC, PIN28_A, O>;
impl<'a, const O: u8> PIN28_W<'a, O> {
    #[doc = "Pin 28 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN28_A::SECURE)
    }
    #[doc = "Pin 28 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN28_A::NON_SECURE)
    }
}
#[doc = "Field `PIN29` reader - Select secure attribute attribute for PIN 29."]
pub type PIN29_R = crate::BitReader<PIN29_A>;
#[doc = "Select secure attribute attribute for PIN 29.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN29_A {
    #[doc = "1: Pin 29 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Pin 29 has its non-secure attribute set"]
    NON_SECURE = 0,
}
impl From<PIN29_A> for bool {
    #[inline(always)]
    fn from(variant: PIN29_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN29_A {
        match self.bits {
            true => PIN29_A::SECURE,
            false => PIN29_A::NON_SECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == PIN29_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NON_SECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == PIN29_A::NON_SECURE
    }
}
#[doc = "Field `PIN29` writer - Select secure attribute attribute for PIN 29."]
pub type PIN29_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERM_SPEC, PIN29_A, O>;
impl<'a, const O: u8> PIN29_W<'a, O> {
    #[doc = "Pin 29 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN29_A::SECURE)
    }
    #[doc = "Pin 29 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN29_A::NON_SECURE)
    }
}
#[doc = "Field `PIN30` reader - Select secure attribute attribute for PIN 30."]
pub type PIN30_R = crate::BitReader<PIN30_A>;
#[doc = "Select secure attribute attribute for PIN 30.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN30_A {
    #[doc = "1: Pin 30 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Pin 30 has its non-secure attribute set"]
    NON_SECURE = 0,
}
impl From<PIN30_A> for bool {
    #[inline(always)]
    fn from(variant: PIN30_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN30_A {
        match self.bits {
            true => PIN30_A::SECURE,
            false => PIN30_A::NON_SECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == PIN30_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NON_SECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == PIN30_A::NON_SECURE
    }
}
#[doc = "Field `PIN30` writer - Select secure attribute attribute for PIN 30."]
pub type PIN30_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERM_SPEC, PIN30_A, O>;
impl<'a, const O: u8> PIN30_W<'a, O> {
    #[doc = "Pin 30 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN30_A::SECURE)
    }
    #[doc = "Pin 30 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN30_A::NON_SECURE)
    }
}
#[doc = "Field `PIN31` reader - Select secure attribute attribute for PIN 31."]
pub type PIN31_R = crate::BitReader<PIN31_A>;
#[doc = "Select secure attribute attribute for PIN 31.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN31_A {
    #[doc = "1: Pin 31 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Pin 31 has its non-secure attribute set"]
    NON_SECURE = 0,
}
impl From<PIN31_A> for bool {
    #[inline(always)]
    fn from(variant: PIN31_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN31_A {
        match self.bits {
            true => PIN31_A::SECURE,
            false => PIN31_A::NON_SECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == PIN31_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NON_SECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == PIN31_A::NON_SECURE
    }
}
#[doc = "Field `PIN31` writer - Select secure attribute attribute for PIN 31."]
pub type PIN31_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERM_SPEC, PIN31_A, O>;
impl<'a, const O: u8> PIN31_W<'a, O> {
    #[doc = "Pin 31 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN31_A::SECURE)
    }
    #[doc = "Pin 31 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN31_A::NON_SECURE)
    }
}
impl R {
    #[doc = "Bit 0 - Select secure attribute attribute for PIN 0."]
    #[inline(always)]
    pub fn pin0(&self) -> PIN0_R {
        PIN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Select secure attribute attribute for PIN 1."]
    #[inline(always)]
    pub fn pin1(&self) -> PIN1_R {
        PIN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Select secure attribute attribute for PIN 2."]
    #[inline(always)]
    pub fn pin2(&self) -> PIN2_R {
        PIN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Select secure attribute attribute for PIN 3."]
    #[inline(always)]
    pub fn pin3(&self) -> PIN3_R {
        PIN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Select secure attribute attribute for PIN 4."]
    #[inline(always)]
    pub fn pin4(&self) -> PIN4_R {
        PIN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Select secure attribute attribute for PIN 5."]
    #[inline(always)]
    pub fn pin5(&self) -> PIN5_R {
        PIN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Select secure attribute attribute for PIN 6."]
    #[inline(always)]
    pub fn pin6(&self) -> PIN6_R {
        PIN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Select secure attribute attribute for PIN 7."]
    #[inline(always)]
    pub fn pin7(&self) -> PIN7_R {
        PIN7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Select secure attribute attribute for PIN 8."]
    #[inline(always)]
    pub fn pin8(&self) -> PIN8_R {
        PIN8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Select secure attribute attribute for PIN 9."]
    #[inline(always)]
    pub fn pin9(&self) -> PIN9_R {
        PIN9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Select secure attribute attribute for PIN 10."]
    #[inline(always)]
    pub fn pin10(&self) -> PIN10_R {
        PIN10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Select secure attribute attribute for PIN 11."]
    #[inline(always)]
    pub fn pin11(&self) -> PIN11_R {
        PIN11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Select secure attribute attribute for PIN 12."]
    #[inline(always)]
    pub fn pin12(&self) -> PIN12_R {
        PIN12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Select secure attribute attribute for PIN 13."]
    #[inline(always)]
    pub fn pin13(&self) -> PIN13_R {
        PIN13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Select secure attribute attribute for PIN 14."]
    #[inline(always)]
    pub fn pin14(&self) -> PIN14_R {
        PIN14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Select secure attribute attribute for PIN 15."]
    #[inline(always)]
    pub fn pin15(&self) -> PIN15_R {
        PIN15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Select secure attribute attribute for PIN 16."]
    #[inline(always)]
    pub fn pin16(&self) -> PIN16_R {
        PIN16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Select secure attribute attribute for PIN 17."]
    #[inline(always)]
    pub fn pin17(&self) -> PIN17_R {
        PIN17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Select secure attribute attribute for PIN 18."]
    #[inline(always)]
    pub fn pin18(&self) -> PIN18_R {
        PIN18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Select secure attribute attribute for PIN 19."]
    #[inline(always)]
    pub fn pin19(&self) -> PIN19_R {
        PIN19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Select secure attribute attribute for PIN 20."]
    #[inline(always)]
    pub fn pin20(&self) -> PIN20_R {
        PIN20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Select secure attribute attribute for PIN 21."]
    #[inline(always)]
    pub fn pin21(&self) -> PIN21_R {
        PIN21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Select secure attribute attribute for PIN 22."]
    #[inline(always)]
    pub fn pin22(&self) -> PIN22_R {
        PIN22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Select secure attribute attribute for PIN 23."]
    #[inline(always)]
    pub fn pin23(&self) -> PIN23_R {
        PIN23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Select secure attribute attribute for PIN 24."]
    #[inline(always)]
    pub fn pin24(&self) -> PIN24_R {
        PIN24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Select secure attribute attribute for PIN 25."]
    #[inline(always)]
    pub fn pin25(&self) -> PIN25_R {
        PIN25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Select secure attribute attribute for PIN 26."]
    #[inline(always)]
    pub fn pin26(&self) -> PIN26_R {
        PIN26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Select secure attribute attribute for PIN 27."]
    #[inline(always)]
    pub fn pin27(&self) -> PIN27_R {
        PIN27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Select secure attribute attribute for PIN 28."]
    #[inline(always)]
    pub fn pin28(&self) -> PIN28_R {
        PIN28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Select secure attribute attribute for PIN 29."]
    #[inline(always)]
    pub fn pin29(&self) -> PIN29_R {
        PIN29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Select secure attribute attribute for PIN 30."]
    #[inline(always)]
    pub fn pin30(&self) -> PIN30_R {
        PIN30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Select secure attribute attribute for PIN 31."]
    #[inline(always)]
    pub fn pin31(&self) -> PIN31_R {
        PIN31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Select secure attribute attribute for PIN 0."]
    #[inline(always)]
    pub fn pin0(&mut self) -> PIN0_W<0> {
        PIN0_W::new(self)
    }
    #[doc = "Bit 1 - Select secure attribute attribute for PIN 1."]
    #[inline(always)]
    pub fn pin1(&mut self) -> PIN1_W<1> {
        PIN1_W::new(self)
    }
    #[doc = "Bit 2 - Select secure attribute attribute for PIN 2."]
    #[inline(always)]
    pub fn pin2(&mut self) -> PIN2_W<2> {
        PIN2_W::new(self)
    }
    #[doc = "Bit 3 - Select secure attribute attribute for PIN 3."]
    #[inline(always)]
    pub fn pin3(&mut self) -> PIN3_W<3> {
        PIN3_W::new(self)
    }
    #[doc = "Bit 4 - Select secure attribute attribute for PIN 4."]
    #[inline(always)]
    pub fn pin4(&mut self) -> PIN4_W<4> {
        PIN4_W::new(self)
    }
    #[doc = "Bit 5 - Select secure attribute attribute for PIN 5."]
    #[inline(always)]
    pub fn pin5(&mut self) -> PIN5_W<5> {
        PIN5_W::new(self)
    }
    #[doc = "Bit 6 - Select secure attribute attribute for PIN 6."]
    #[inline(always)]
    pub fn pin6(&mut self) -> PIN6_W<6> {
        PIN6_W::new(self)
    }
    #[doc = "Bit 7 - Select secure attribute attribute for PIN 7."]
    #[inline(always)]
    pub fn pin7(&mut self) -> PIN7_W<7> {
        PIN7_W::new(self)
    }
    #[doc = "Bit 8 - Select secure attribute attribute for PIN 8."]
    #[inline(always)]
    pub fn pin8(&mut self) -> PIN8_W<8> {
        PIN8_W::new(self)
    }
    #[doc = "Bit 9 - Select secure attribute attribute for PIN 9."]
    #[inline(always)]
    pub fn pin9(&mut self) -> PIN9_W<9> {
        PIN9_W::new(self)
    }
    #[doc = "Bit 10 - Select secure attribute attribute for PIN 10."]
    #[inline(always)]
    pub fn pin10(&mut self) -> PIN10_W<10> {
        PIN10_W::new(self)
    }
    #[doc = "Bit 11 - Select secure attribute attribute for PIN 11."]
    #[inline(always)]
    pub fn pin11(&mut self) -> PIN11_W<11> {
        PIN11_W::new(self)
    }
    #[doc = "Bit 12 - Select secure attribute attribute for PIN 12."]
    #[inline(always)]
    pub fn pin12(&mut self) -> PIN12_W<12> {
        PIN12_W::new(self)
    }
    #[doc = "Bit 13 - Select secure attribute attribute for PIN 13."]
    #[inline(always)]
    pub fn pin13(&mut self) -> PIN13_W<13> {
        PIN13_W::new(self)
    }
    #[doc = "Bit 14 - Select secure attribute attribute for PIN 14."]
    #[inline(always)]
    pub fn pin14(&mut self) -> PIN14_W<14> {
        PIN14_W::new(self)
    }
    #[doc = "Bit 15 - Select secure attribute attribute for PIN 15."]
    #[inline(always)]
    pub fn pin15(&mut self) -> PIN15_W<15> {
        PIN15_W::new(self)
    }
    #[doc = "Bit 16 - Select secure attribute attribute for PIN 16."]
    #[inline(always)]
    pub fn pin16(&mut self) -> PIN16_W<16> {
        PIN16_W::new(self)
    }
    #[doc = "Bit 17 - Select secure attribute attribute for PIN 17."]
    #[inline(always)]
    pub fn pin17(&mut self) -> PIN17_W<17> {
        PIN17_W::new(self)
    }
    #[doc = "Bit 18 - Select secure attribute attribute for PIN 18."]
    #[inline(always)]
    pub fn pin18(&mut self) -> PIN18_W<18> {
        PIN18_W::new(self)
    }
    #[doc = "Bit 19 - Select secure attribute attribute for PIN 19."]
    #[inline(always)]
    pub fn pin19(&mut self) -> PIN19_W<19> {
        PIN19_W::new(self)
    }
    #[doc = "Bit 20 - Select secure attribute attribute for PIN 20."]
    #[inline(always)]
    pub fn pin20(&mut self) -> PIN20_W<20> {
        PIN20_W::new(self)
    }
    #[doc = "Bit 21 - Select secure attribute attribute for PIN 21."]
    #[inline(always)]
    pub fn pin21(&mut self) -> PIN21_W<21> {
        PIN21_W::new(self)
    }
    #[doc = "Bit 22 - Select secure attribute attribute for PIN 22."]
    #[inline(always)]
    pub fn pin22(&mut self) -> PIN22_W<22> {
        PIN22_W::new(self)
    }
    #[doc = "Bit 23 - Select secure attribute attribute for PIN 23."]
    #[inline(always)]
    pub fn pin23(&mut self) -> PIN23_W<23> {
        PIN23_W::new(self)
    }
    #[doc = "Bit 24 - Select secure attribute attribute for PIN 24."]
    #[inline(always)]
    pub fn pin24(&mut self) -> PIN24_W<24> {
        PIN24_W::new(self)
    }
    #[doc = "Bit 25 - Select secure attribute attribute for PIN 25."]
    #[inline(always)]
    pub fn pin25(&mut self) -> PIN25_W<25> {
        PIN25_W::new(self)
    }
    #[doc = "Bit 26 - Select secure attribute attribute for PIN 26."]
    #[inline(always)]
    pub fn pin26(&mut self) -> PIN26_W<26> {
        PIN26_W::new(self)
    }
    #[doc = "Bit 27 - Select secure attribute attribute for PIN 27."]
    #[inline(always)]
    pub fn pin27(&mut self) -> PIN27_W<27> {
        PIN27_W::new(self)
    }
    #[doc = "Bit 28 - Select secure attribute attribute for PIN 28."]
    #[inline(always)]
    pub fn pin28(&mut self) -> PIN28_W<28> {
        PIN28_W::new(self)
    }
    #[doc = "Bit 29 - Select secure attribute attribute for PIN 29."]
    #[inline(always)]
    pub fn pin29(&mut self) -> PIN29_W<29> {
        PIN29_W::new(self)
    }
    #[doc = "Bit 30 - Select secure attribute attribute for PIN 30."]
    #[inline(always)]
    pub fn pin30(&mut self) -> PIN30_W<30> {
        PIN30_W::new(self)
    }
    #[doc = "Bit 31 - Select secure attribute attribute for PIN 31."]
    #[inline(always)]
    pub fn pin31(&mut self) -> PIN31_W<31> {
        PIN31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description cluster: Select between secure and non-secure attribute for pins 0 to 31 of port n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perm](index.html) module"]
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
#[doc = "`reset()` method sets PERM to value 0xffff_ffff"]
impl crate::Resettable for PERM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
