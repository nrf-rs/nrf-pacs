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
#[doc = "Select secure attribute attribute for PIN 0.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN0_A {
    #[doc = "1: Pin 0 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Pin 0 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<PIN0_A> for bool {
    #[inline(always)]
    fn from(variant: PIN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN0` reader - Select secure attribute attribute for PIN 0."]
pub struct PIN0_R(crate::FieldReader<bool, PIN0_A>);
impl PIN0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIN0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN0_A {
        match self.bits {
            true => PIN0_A::SECURE,
            false => PIN0_A::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        **self == PIN0_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        **self == PIN0_A::NONSECURE
    }
}
impl core::ops::Deref for PIN0_R {
    type Target = crate::FieldReader<bool, PIN0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN0` writer - Select secure attribute attribute for PIN 0."]
pub struct PIN0_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin 0 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN0_A::SECURE)
    }
    #[doc = "Pin 0 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN0_A::NONSECURE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Select secure attribute attribute for PIN 1.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN1_A {
    #[doc = "1: Pin 1 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Pin 1 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<PIN1_A> for bool {
    #[inline(always)]
    fn from(variant: PIN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN1` reader - Select secure attribute attribute for PIN 1."]
pub struct PIN1_R(crate::FieldReader<bool, PIN1_A>);
impl PIN1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIN1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN1_A {
        match self.bits {
            true => PIN1_A::SECURE,
            false => PIN1_A::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        **self == PIN1_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        **self == PIN1_A::NONSECURE
    }
}
impl core::ops::Deref for PIN1_R {
    type Target = crate::FieldReader<bool, PIN1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN1` writer - Select secure attribute attribute for PIN 1."]
pub struct PIN1_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin 1 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN1_A::SECURE)
    }
    #[doc = "Pin 1 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN1_A::NONSECURE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Select secure attribute attribute for PIN 2.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN2_A {
    #[doc = "1: Pin 2 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Pin 2 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<PIN2_A> for bool {
    #[inline(always)]
    fn from(variant: PIN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN2` reader - Select secure attribute attribute for PIN 2."]
pub struct PIN2_R(crate::FieldReader<bool, PIN2_A>);
impl PIN2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIN2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN2_A {
        match self.bits {
            true => PIN2_A::SECURE,
            false => PIN2_A::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        **self == PIN2_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        **self == PIN2_A::NONSECURE
    }
}
impl core::ops::Deref for PIN2_R {
    type Target = crate::FieldReader<bool, PIN2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN2` writer - Select secure attribute attribute for PIN 2."]
pub struct PIN2_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin 2 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN2_A::SECURE)
    }
    #[doc = "Pin 2 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN2_A::NONSECURE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Select secure attribute attribute for PIN 3.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN3_A {
    #[doc = "1: Pin 3 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Pin 3 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<PIN3_A> for bool {
    #[inline(always)]
    fn from(variant: PIN3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN3` reader - Select secure attribute attribute for PIN 3."]
pub struct PIN3_R(crate::FieldReader<bool, PIN3_A>);
impl PIN3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIN3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN3_A {
        match self.bits {
            true => PIN3_A::SECURE,
            false => PIN3_A::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        **self == PIN3_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        **self == PIN3_A::NONSECURE
    }
}
impl core::ops::Deref for PIN3_R {
    type Target = crate::FieldReader<bool, PIN3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN3` writer - Select secure attribute attribute for PIN 3."]
pub struct PIN3_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin 3 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN3_A::SECURE)
    }
    #[doc = "Pin 3 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN3_A::NONSECURE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Select secure attribute attribute for PIN 4.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN4_A {
    #[doc = "1: Pin 4 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Pin 4 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<PIN4_A> for bool {
    #[inline(always)]
    fn from(variant: PIN4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN4` reader - Select secure attribute attribute for PIN 4."]
pub struct PIN4_R(crate::FieldReader<bool, PIN4_A>);
impl PIN4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIN4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN4_A {
        match self.bits {
            true => PIN4_A::SECURE,
            false => PIN4_A::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        **self == PIN4_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        **self == PIN4_A::NONSECURE
    }
}
impl core::ops::Deref for PIN4_R {
    type Target = crate::FieldReader<bool, PIN4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN4` writer - Select secure attribute attribute for PIN 4."]
pub struct PIN4_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin 4 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN4_A::SECURE)
    }
    #[doc = "Pin 4 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN4_A::NONSECURE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Select secure attribute attribute for PIN 5.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN5_A {
    #[doc = "1: Pin 5 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Pin 5 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<PIN5_A> for bool {
    #[inline(always)]
    fn from(variant: PIN5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN5` reader - Select secure attribute attribute for PIN 5."]
pub struct PIN5_R(crate::FieldReader<bool, PIN5_A>);
impl PIN5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIN5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN5_A {
        match self.bits {
            true => PIN5_A::SECURE,
            false => PIN5_A::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        **self == PIN5_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        **self == PIN5_A::NONSECURE
    }
}
impl core::ops::Deref for PIN5_R {
    type Target = crate::FieldReader<bool, PIN5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN5` writer - Select secure attribute attribute for PIN 5."]
pub struct PIN5_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin 5 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN5_A::SECURE)
    }
    #[doc = "Pin 5 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN5_A::NONSECURE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Select secure attribute attribute for PIN 6.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN6_A {
    #[doc = "1: Pin 6 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Pin 6 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<PIN6_A> for bool {
    #[inline(always)]
    fn from(variant: PIN6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN6` reader - Select secure attribute attribute for PIN 6."]
pub struct PIN6_R(crate::FieldReader<bool, PIN6_A>);
impl PIN6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIN6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN6_A {
        match self.bits {
            true => PIN6_A::SECURE,
            false => PIN6_A::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        **self == PIN6_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        **self == PIN6_A::NONSECURE
    }
}
impl core::ops::Deref for PIN6_R {
    type Target = crate::FieldReader<bool, PIN6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN6` writer - Select secure attribute attribute for PIN 6."]
pub struct PIN6_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin 6 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN6_A::SECURE)
    }
    #[doc = "Pin 6 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN6_A::NONSECURE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Select secure attribute attribute for PIN 7.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN7_A {
    #[doc = "1: Pin 7 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Pin 7 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<PIN7_A> for bool {
    #[inline(always)]
    fn from(variant: PIN7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN7` reader - Select secure attribute attribute for PIN 7."]
pub struct PIN7_R(crate::FieldReader<bool, PIN7_A>);
impl PIN7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIN7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN7_A {
        match self.bits {
            true => PIN7_A::SECURE,
            false => PIN7_A::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        **self == PIN7_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        **self == PIN7_A::NONSECURE
    }
}
impl core::ops::Deref for PIN7_R {
    type Target = crate::FieldReader<bool, PIN7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN7` writer - Select secure attribute attribute for PIN 7."]
pub struct PIN7_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin 7 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN7_A::SECURE)
    }
    #[doc = "Pin 7 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN7_A::NONSECURE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Select secure attribute attribute for PIN 8.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN8_A {
    #[doc = "1: Pin 8 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Pin 8 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<PIN8_A> for bool {
    #[inline(always)]
    fn from(variant: PIN8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN8` reader - Select secure attribute attribute for PIN 8."]
pub struct PIN8_R(crate::FieldReader<bool, PIN8_A>);
impl PIN8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIN8_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN8_A {
        match self.bits {
            true => PIN8_A::SECURE,
            false => PIN8_A::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        **self == PIN8_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        **self == PIN8_A::NONSECURE
    }
}
impl core::ops::Deref for PIN8_R {
    type Target = crate::FieldReader<bool, PIN8_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN8` writer - Select secure attribute attribute for PIN 8."]
pub struct PIN8_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN8_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin 8 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN8_A::SECURE)
    }
    #[doc = "Pin 8 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN8_A::NONSECURE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Select secure attribute attribute for PIN 9.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN9_A {
    #[doc = "1: Pin 9 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Pin 9 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<PIN9_A> for bool {
    #[inline(always)]
    fn from(variant: PIN9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN9` reader - Select secure attribute attribute for PIN 9."]
pub struct PIN9_R(crate::FieldReader<bool, PIN9_A>);
impl PIN9_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIN9_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN9_A {
        match self.bits {
            true => PIN9_A::SECURE,
            false => PIN9_A::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        **self == PIN9_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        **self == PIN9_A::NONSECURE
    }
}
impl core::ops::Deref for PIN9_R {
    type Target = crate::FieldReader<bool, PIN9_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN9` writer - Select secure attribute attribute for PIN 9."]
pub struct PIN9_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN9_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin 9 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN9_A::SECURE)
    }
    #[doc = "Pin 9 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN9_A::NONSECURE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Select secure attribute attribute for PIN 10.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN10_A {
    #[doc = "1: Pin 10 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Pin 10 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<PIN10_A> for bool {
    #[inline(always)]
    fn from(variant: PIN10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN10` reader - Select secure attribute attribute for PIN 10."]
pub struct PIN10_R(crate::FieldReader<bool, PIN10_A>);
impl PIN10_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIN10_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN10_A {
        match self.bits {
            true => PIN10_A::SECURE,
            false => PIN10_A::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        **self == PIN10_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        **self == PIN10_A::NONSECURE
    }
}
impl core::ops::Deref for PIN10_R {
    type Target = crate::FieldReader<bool, PIN10_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN10` writer - Select secure attribute attribute for PIN 10."]
pub struct PIN10_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN10_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin 10 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN10_A::SECURE)
    }
    #[doc = "Pin 10 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN10_A::NONSECURE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Select secure attribute attribute for PIN 11.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN11_A {
    #[doc = "1: Pin 11 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Pin 11 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<PIN11_A> for bool {
    #[inline(always)]
    fn from(variant: PIN11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN11` reader - Select secure attribute attribute for PIN 11."]
pub struct PIN11_R(crate::FieldReader<bool, PIN11_A>);
impl PIN11_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIN11_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN11_A {
        match self.bits {
            true => PIN11_A::SECURE,
            false => PIN11_A::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        **self == PIN11_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        **self == PIN11_A::NONSECURE
    }
}
impl core::ops::Deref for PIN11_R {
    type Target = crate::FieldReader<bool, PIN11_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN11` writer - Select secure attribute attribute for PIN 11."]
pub struct PIN11_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN11_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin 11 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN11_A::SECURE)
    }
    #[doc = "Pin 11 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN11_A::NONSECURE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Select secure attribute attribute for PIN 12.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN12_A {
    #[doc = "1: Pin 12 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Pin 12 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<PIN12_A> for bool {
    #[inline(always)]
    fn from(variant: PIN12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN12` reader - Select secure attribute attribute for PIN 12."]
pub struct PIN12_R(crate::FieldReader<bool, PIN12_A>);
impl PIN12_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIN12_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN12_A {
        match self.bits {
            true => PIN12_A::SECURE,
            false => PIN12_A::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        **self == PIN12_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        **self == PIN12_A::NONSECURE
    }
}
impl core::ops::Deref for PIN12_R {
    type Target = crate::FieldReader<bool, PIN12_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN12` writer - Select secure attribute attribute for PIN 12."]
pub struct PIN12_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN12_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin 12 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN12_A::SECURE)
    }
    #[doc = "Pin 12 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN12_A::NONSECURE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Select secure attribute attribute for PIN 13.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN13_A {
    #[doc = "1: Pin 13 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Pin 13 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<PIN13_A> for bool {
    #[inline(always)]
    fn from(variant: PIN13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN13` reader - Select secure attribute attribute for PIN 13."]
pub struct PIN13_R(crate::FieldReader<bool, PIN13_A>);
impl PIN13_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIN13_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN13_A {
        match self.bits {
            true => PIN13_A::SECURE,
            false => PIN13_A::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        **self == PIN13_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        **self == PIN13_A::NONSECURE
    }
}
impl core::ops::Deref for PIN13_R {
    type Target = crate::FieldReader<bool, PIN13_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN13` writer - Select secure attribute attribute for PIN 13."]
pub struct PIN13_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN13_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin 13 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN13_A::SECURE)
    }
    #[doc = "Pin 13 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN13_A::NONSECURE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Select secure attribute attribute for PIN 14.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN14_A {
    #[doc = "1: Pin 14 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Pin 14 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<PIN14_A> for bool {
    #[inline(always)]
    fn from(variant: PIN14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN14` reader - Select secure attribute attribute for PIN 14."]
pub struct PIN14_R(crate::FieldReader<bool, PIN14_A>);
impl PIN14_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIN14_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN14_A {
        match self.bits {
            true => PIN14_A::SECURE,
            false => PIN14_A::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        **self == PIN14_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        **self == PIN14_A::NONSECURE
    }
}
impl core::ops::Deref for PIN14_R {
    type Target = crate::FieldReader<bool, PIN14_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN14` writer - Select secure attribute attribute for PIN 14."]
pub struct PIN14_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN14_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin 14 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN14_A::SECURE)
    }
    #[doc = "Pin 14 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN14_A::NONSECURE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Select secure attribute attribute for PIN 15.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN15_A {
    #[doc = "1: Pin 15 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Pin 15 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<PIN15_A> for bool {
    #[inline(always)]
    fn from(variant: PIN15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN15` reader - Select secure attribute attribute for PIN 15."]
pub struct PIN15_R(crate::FieldReader<bool, PIN15_A>);
impl PIN15_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIN15_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN15_A {
        match self.bits {
            true => PIN15_A::SECURE,
            false => PIN15_A::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        **self == PIN15_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        **self == PIN15_A::NONSECURE
    }
}
impl core::ops::Deref for PIN15_R {
    type Target = crate::FieldReader<bool, PIN15_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN15` writer - Select secure attribute attribute for PIN 15."]
pub struct PIN15_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN15_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin 15 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN15_A::SECURE)
    }
    #[doc = "Pin 15 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN15_A::NONSECURE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Select secure attribute attribute for PIN 16.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN16_A {
    #[doc = "1: Pin 16 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Pin 16 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<PIN16_A> for bool {
    #[inline(always)]
    fn from(variant: PIN16_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN16` reader - Select secure attribute attribute for PIN 16."]
pub struct PIN16_R(crate::FieldReader<bool, PIN16_A>);
impl PIN16_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIN16_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN16_A {
        match self.bits {
            true => PIN16_A::SECURE,
            false => PIN16_A::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        **self == PIN16_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        **self == PIN16_A::NONSECURE
    }
}
impl core::ops::Deref for PIN16_R {
    type Target = crate::FieldReader<bool, PIN16_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN16` writer - Select secure attribute attribute for PIN 16."]
pub struct PIN16_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN16_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin 16 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN16_A::SECURE)
    }
    #[doc = "Pin 16 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN16_A::NONSECURE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Select secure attribute attribute for PIN 17.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN17_A {
    #[doc = "1: Pin 17 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Pin 17 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<PIN17_A> for bool {
    #[inline(always)]
    fn from(variant: PIN17_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN17` reader - Select secure attribute attribute for PIN 17."]
pub struct PIN17_R(crate::FieldReader<bool, PIN17_A>);
impl PIN17_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIN17_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN17_A {
        match self.bits {
            true => PIN17_A::SECURE,
            false => PIN17_A::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        **self == PIN17_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        **self == PIN17_A::NONSECURE
    }
}
impl core::ops::Deref for PIN17_R {
    type Target = crate::FieldReader<bool, PIN17_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN17` writer - Select secure attribute attribute for PIN 17."]
pub struct PIN17_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN17_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin 17 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN17_A::SECURE)
    }
    #[doc = "Pin 17 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN17_A::NONSECURE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Select secure attribute attribute for PIN 18.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN18_A {
    #[doc = "1: Pin 18 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Pin 18 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<PIN18_A> for bool {
    #[inline(always)]
    fn from(variant: PIN18_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN18` reader - Select secure attribute attribute for PIN 18."]
pub struct PIN18_R(crate::FieldReader<bool, PIN18_A>);
impl PIN18_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIN18_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN18_A {
        match self.bits {
            true => PIN18_A::SECURE,
            false => PIN18_A::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        **self == PIN18_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        **self == PIN18_A::NONSECURE
    }
}
impl core::ops::Deref for PIN18_R {
    type Target = crate::FieldReader<bool, PIN18_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN18` writer - Select secure attribute attribute for PIN 18."]
pub struct PIN18_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN18_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN18_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin 18 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN18_A::SECURE)
    }
    #[doc = "Pin 18 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN18_A::NONSECURE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Select secure attribute attribute for PIN 19.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN19_A {
    #[doc = "1: Pin 19 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Pin 19 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<PIN19_A> for bool {
    #[inline(always)]
    fn from(variant: PIN19_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN19` reader - Select secure attribute attribute for PIN 19."]
pub struct PIN19_R(crate::FieldReader<bool, PIN19_A>);
impl PIN19_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIN19_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN19_A {
        match self.bits {
            true => PIN19_A::SECURE,
            false => PIN19_A::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        **self == PIN19_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        **self == PIN19_A::NONSECURE
    }
}
impl core::ops::Deref for PIN19_R {
    type Target = crate::FieldReader<bool, PIN19_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN19` writer - Select secure attribute attribute for PIN 19."]
pub struct PIN19_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN19_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN19_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin 19 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN19_A::SECURE)
    }
    #[doc = "Pin 19 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN19_A::NONSECURE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Select secure attribute attribute for PIN 20.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN20_A {
    #[doc = "1: Pin 20 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Pin 20 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<PIN20_A> for bool {
    #[inline(always)]
    fn from(variant: PIN20_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN20` reader - Select secure attribute attribute for PIN 20."]
pub struct PIN20_R(crate::FieldReader<bool, PIN20_A>);
impl PIN20_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIN20_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN20_A {
        match self.bits {
            true => PIN20_A::SECURE,
            false => PIN20_A::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        **self == PIN20_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        **self == PIN20_A::NONSECURE
    }
}
impl core::ops::Deref for PIN20_R {
    type Target = crate::FieldReader<bool, PIN20_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN20` writer - Select secure attribute attribute for PIN 20."]
pub struct PIN20_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN20_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN20_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin 20 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN20_A::SECURE)
    }
    #[doc = "Pin 20 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN20_A::NONSECURE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Select secure attribute attribute for PIN 21.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN21_A {
    #[doc = "1: Pin 21 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Pin 21 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<PIN21_A> for bool {
    #[inline(always)]
    fn from(variant: PIN21_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN21` reader - Select secure attribute attribute for PIN 21."]
pub struct PIN21_R(crate::FieldReader<bool, PIN21_A>);
impl PIN21_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIN21_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN21_A {
        match self.bits {
            true => PIN21_A::SECURE,
            false => PIN21_A::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        **self == PIN21_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        **self == PIN21_A::NONSECURE
    }
}
impl core::ops::Deref for PIN21_R {
    type Target = crate::FieldReader<bool, PIN21_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN21` writer - Select secure attribute attribute for PIN 21."]
pub struct PIN21_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN21_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN21_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin 21 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN21_A::SECURE)
    }
    #[doc = "Pin 21 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN21_A::NONSECURE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Select secure attribute attribute for PIN 22.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN22_A {
    #[doc = "1: Pin 22 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Pin 22 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<PIN22_A> for bool {
    #[inline(always)]
    fn from(variant: PIN22_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN22` reader - Select secure attribute attribute for PIN 22."]
pub struct PIN22_R(crate::FieldReader<bool, PIN22_A>);
impl PIN22_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIN22_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN22_A {
        match self.bits {
            true => PIN22_A::SECURE,
            false => PIN22_A::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        **self == PIN22_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        **self == PIN22_A::NONSECURE
    }
}
impl core::ops::Deref for PIN22_R {
    type Target = crate::FieldReader<bool, PIN22_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN22` writer - Select secure attribute attribute for PIN 22."]
pub struct PIN22_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN22_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN22_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin 22 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN22_A::SECURE)
    }
    #[doc = "Pin 22 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN22_A::NONSECURE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Select secure attribute attribute for PIN 23.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN23_A {
    #[doc = "1: Pin 23 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Pin 23 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<PIN23_A> for bool {
    #[inline(always)]
    fn from(variant: PIN23_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN23` reader - Select secure attribute attribute for PIN 23."]
pub struct PIN23_R(crate::FieldReader<bool, PIN23_A>);
impl PIN23_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIN23_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN23_A {
        match self.bits {
            true => PIN23_A::SECURE,
            false => PIN23_A::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        **self == PIN23_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        **self == PIN23_A::NONSECURE
    }
}
impl core::ops::Deref for PIN23_R {
    type Target = crate::FieldReader<bool, PIN23_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN23` writer - Select secure attribute attribute for PIN 23."]
pub struct PIN23_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN23_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin 23 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN23_A::SECURE)
    }
    #[doc = "Pin 23 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN23_A::NONSECURE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Select secure attribute attribute for PIN 24.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN24_A {
    #[doc = "1: Pin 24 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Pin 24 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<PIN24_A> for bool {
    #[inline(always)]
    fn from(variant: PIN24_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN24` reader - Select secure attribute attribute for PIN 24."]
pub struct PIN24_R(crate::FieldReader<bool, PIN24_A>);
impl PIN24_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIN24_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN24_A {
        match self.bits {
            true => PIN24_A::SECURE,
            false => PIN24_A::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        **self == PIN24_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        **self == PIN24_A::NONSECURE
    }
}
impl core::ops::Deref for PIN24_R {
    type Target = crate::FieldReader<bool, PIN24_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN24` writer - Select secure attribute attribute for PIN 24."]
pub struct PIN24_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN24_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN24_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin 24 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN24_A::SECURE)
    }
    #[doc = "Pin 24 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN24_A::NONSECURE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Select secure attribute attribute for PIN 25.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN25_A {
    #[doc = "1: Pin 25 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Pin 25 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<PIN25_A> for bool {
    #[inline(always)]
    fn from(variant: PIN25_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN25` reader - Select secure attribute attribute for PIN 25."]
pub struct PIN25_R(crate::FieldReader<bool, PIN25_A>);
impl PIN25_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIN25_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN25_A {
        match self.bits {
            true => PIN25_A::SECURE,
            false => PIN25_A::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        **self == PIN25_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        **self == PIN25_A::NONSECURE
    }
}
impl core::ops::Deref for PIN25_R {
    type Target = crate::FieldReader<bool, PIN25_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN25` writer - Select secure attribute attribute for PIN 25."]
pub struct PIN25_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN25_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN25_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin 25 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN25_A::SECURE)
    }
    #[doc = "Pin 25 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN25_A::NONSECURE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Select secure attribute attribute for PIN 26.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN26_A {
    #[doc = "1: Pin 26 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Pin 26 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<PIN26_A> for bool {
    #[inline(always)]
    fn from(variant: PIN26_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN26` reader - Select secure attribute attribute for PIN 26."]
pub struct PIN26_R(crate::FieldReader<bool, PIN26_A>);
impl PIN26_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIN26_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN26_A {
        match self.bits {
            true => PIN26_A::SECURE,
            false => PIN26_A::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        **self == PIN26_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        **self == PIN26_A::NONSECURE
    }
}
impl core::ops::Deref for PIN26_R {
    type Target = crate::FieldReader<bool, PIN26_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN26` writer - Select secure attribute attribute for PIN 26."]
pub struct PIN26_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN26_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN26_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin 26 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN26_A::SECURE)
    }
    #[doc = "Pin 26 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN26_A::NONSECURE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Select secure attribute attribute for PIN 27.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN27_A {
    #[doc = "1: Pin 27 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Pin 27 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<PIN27_A> for bool {
    #[inline(always)]
    fn from(variant: PIN27_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN27` reader - Select secure attribute attribute for PIN 27."]
pub struct PIN27_R(crate::FieldReader<bool, PIN27_A>);
impl PIN27_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIN27_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN27_A {
        match self.bits {
            true => PIN27_A::SECURE,
            false => PIN27_A::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        **self == PIN27_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        **self == PIN27_A::NONSECURE
    }
}
impl core::ops::Deref for PIN27_R {
    type Target = crate::FieldReader<bool, PIN27_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN27` writer - Select secure attribute attribute for PIN 27."]
pub struct PIN27_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN27_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN27_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin 27 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN27_A::SECURE)
    }
    #[doc = "Pin 27 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN27_A::NONSECURE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Select secure attribute attribute for PIN 28.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN28_A {
    #[doc = "1: Pin 28 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Pin 28 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<PIN28_A> for bool {
    #[inline(always)]
    fn from(variant: PIN28_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN28` reader - Select secure attribute attribute for PIN 28."]
pub struct PIN28_R(crate::FieldReader<bool, PIN28_A>);
impl PIN28_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIN28_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN28_A {
        match self.bits {
            true => PIN28_A::SECURE,
            false => PIN28_A::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        **self == PIN28_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        **self == PIN28_A::NONSECURE
    }
}
impl core::ops::Deref for PIN28_R {
    type Target = crate::FieldReader<bool, PIN28_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN28` writer - Select secure attribute attribute for PIN 28."]
pub struct PIN28_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN28_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN28_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin 28 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN28_A::SECURE)
    }
    #[doc = "Pin 28 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN28_A::NONSECURE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Select secure attribute attribute for PIN 29.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN29_A {
    #[doc = "1: Pin 29 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Pin 29 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<PIN29_A> for bool {
    #[inline(always)]
    fn from(variant: PIN29_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN29` reader - Select secure attribute attribute for PIN 29."]
pub struct PIN29_R(crate::FieldReader<bool, PIN29_A>);
impl PIN29_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIN29_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN29_A {
        match self.bits {
            true => PIN29_A::SECURE,
            false => PIN29_A::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        **self == PIN29_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        **self == PIN29_A::NONSECURE
    }
}
impl core::ops::Deref for PIN29_R {
    type Target = crate::FieldReader<bool, PIN29_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN29` writer - Select secure attribute attribute for PIN 29."]
pub struct PIN29_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN29_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN29_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin 29 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN29_A::SECURE)
    }
    #[doc = "Pin 29 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN29_A::NONSECURE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Select secure attribute attribute for PIN 30.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN30_A {
    #[doc = "1: Pin 30 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Pin 30 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<PIN30_A> for bool {
    #[inline(always)]
    fn from(variant: PIN30_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN30` reader - Select secure attribute attribute for PIN 30."]
pub struct PIN30_R(crate::FieldReader<bool, PIN30_A>);
impl PIN30_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIN30_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN30_A {
        match self.bits {
            true => PIN30_A::SECURE,
            false => PIN30_A::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        **self == PIN30_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        **self == PIN30_A::NONSECURE
    }
}
impl core::ops::Deref for PIN30_R {
    type Target = crate::FieldReader<bool, PIN30_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN30` writer - Select secure attribute attribute for PIN 30."]
pub struct PIN30_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN30_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN30_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin 30 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN30_A::SECURE)
    }
    #[doc = "Pin 30 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN30_A::NONSECURE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Select secure attribute attribute for PIN 31.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN31_A {
    #[doc = "1: Pin 31 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Pin 31 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<PIN31_A> for bool {
    #[inline(always)]
    fn from(variant: PIN31_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN31` reader - Select secure attribute attribute for PIN 31."]
pub struct PIN31_R(crate::FieldReader<bool, PIN31_A>);
impl PIN31_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIN31_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN31_A {
        match self.bits {
            true => PIN31_A::SECURE,
            false => PIN31_A::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        **self == PIN31_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        **self == PIN31_A::NONSECURE
    }
}
impl core::ops::Deref for PIN31_R {
    type Target = crate::FieldReader<bool, PIN31_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN31` writer - Select secure attribute attribute for PIN 31."]
pub struct PIN31_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN31_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN31_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin 31 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN31_A::SECURE)
    }
    #[doc = "Pin 31 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN31_A::NONSECURE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Select secure attribute attribute for PIN 0."]
    #[inline(always)]
    pub fn pin0(&self) -> PIN0_R {
        PIN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Select secure attribute attribute for PIN 1."]
    #[inline(always)]
    pub fn pin1(&self) -> PIN1_R {
        PIN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Select secure attribute attribute for PIN 2."]
    #[inline(always)]
    pub fn pin2(&self) -> PIN2_R {
        PIN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Select secure attribute attribute for PIN 3."]
    #[inline(always)]
    pub fn pin3(&self) -> PIN3_R {
        PIN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Select secure attribute attribute for PIN 4."]
    #[inline(always)]
    pub fn pin4(&self) -> PIN4_R {
        PIN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Select secure attribute attribute for PIN 5."]
    #[inline(always)]
    pub fn pin5(&self) -> PIN5_R {
        PIN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Select secure attribute attribute for PIN 6."]
    #[inline(always)]
    pub fn pin6(&self) -> PIN6_R {
        PIN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Select secure attribute attribute for PIN 7."]
    #[inline(always)]
    pub fn pin7(&self) -> PIN7_R {
        PIN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Select secure attribute attribute for PIN 8."]
    #[inline(always)]
    pub fn pin8(&self) -> PIN8_R {
        PIN8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Select secure attribute attribute for PIN 9."]
    #[inline(always)]
    pub fn pin9(&self) -> PIN9_R {
        PIN9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Select secure attribute attribute for PIN 10."]
    #[inline(always)]
    pub fn pin10(&self) -> PIN10_R {
        PIN10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Select secure attribute attribute for PIN 11."]
    #[inline(always)]
    pub fn pin11(&self) -> PIN11_R {
        PIN11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Select secure attribute attribute for PIN 12."]
    #[inline(always)]
    pub fn pin12(&self) -> PIN12_R {
        PIN12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Select secure attribute attribute for PIN 13."]
    #[inline(always)]
    pub fn pin13(&self) -> PIN13_R {
        PIN13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Select secure attribute attribute for PIN 14."]
    #[inline(always)]
    pub fn pin14(&self) -> PIN14_R {
        PIN14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Select secure attribute attribute for PIN 15."]
    #[inline(always)]
    pub fn pin15(&self) -> PIN15_R {
        PIN15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Select secure attribute attribute for PIN 16."]
    #[inline(always)]
    pub fn pin16(&self) -> PIN16_R {
        PIN16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Select secure attribute attribute for PIN 17."]
    #[inline(always)]
    pub fn pin17(&self) -> PIN17_R {
        PIN17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Select secure attribute attribute for PIN 18."]
    #[inline(always)]
    pub fn pin18(&self) -> PIN18_R {
        PIN18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Select secure attribute attribute for PIN 19."]
    #[inline(always)]
    pub fn pin19(&self) -> PIN19_R {
        PIN19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Select secure attribute attribute for PIN 20."]
    #[inline(always)]
    pub fn pin20(&self) -> PIN20_R {
        PIN20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Select secure attribute attribute for PIN 21."]
    #[inline(always)]
    pub fn pin21(&self) -> PIN21_R {
        PIN21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Select secure attribute attribute for PIN 22."]
    #[inline(always)]
    pub fn pin22(&self) -> PIN22_R {
        PIN22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Select secure attribute attribute for PIN 23."]
    #[inline(always)]
    pub fn pin23(&self) -> PIN23_R {
        PIN23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Select secure attribute attribute for PIN 24."]
    #[inline(always)]
    pub fn pin24(&self) -> PIN24_R {
        PIN24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Select secure attribute attribute for PIN 25."]
    #[inline(always)]
    pub fn pin25(&self) -> PIN25_R {
        PIN25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Select secure attribute attribute for PIN 26."]
    #[inline(always)]
    pub fn pin26(&self) -> PIN26_R {
        PIN26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Select secure attribute attribute for PIN 27."]
    #[inline(always)]
    pub fn pin27(&self) -> PIN27_R {
        PIN27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Select secure attribute attribute for PIN 28."]
    #[inline(always)]
    pub fn pin28(&self) -> PIN28_R {
        PIN28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Select secure attribute attribute for PIN 29."]
    #[inline(always)]
    pub fn pin29(&self) -> PIN29_R {
        PIN29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Select secure attribute attribute for PIN 30."]
    #[inline(always)]
    pub fn pin30(&self) -> PIN30_R {
        PIN30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Select secure attribute attribute for PIN 31."]
    #[inline(always)]
    pub fn pin31(&self) -> PIN31_R {
        PIN31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Select secure attribute attribute for PIN 0."]
    #[inline(always)]
    pub fn pin0(&mut self) -> PIN0_W {
        PIN0_W { w: self }
    }
    #[doc = "Bit 1 - Select secure attribute attribute for PIN 1."]
    #[inline(always)]
    pub fn pin1(&mut self) -> PIN1_W {
        PIN1_W { w: self }
    }
    #[doc = "Bit 2 - Select secure attribute attribute for PIN 2."]
    #[inline(always)]
    pub fn pin2(&mut self) -> PIN2_W {
        PIN2_W { w: self }
    }
    #[doc = "Bit 3 - Select secure attribute attribute for PIN 3."]
    #[inline(always)]
    pub fn pin3(&mut self) -> PIN3_W {
        PIN3_W { w: self }
    }
    #[doc = "Bit 4 - Select secure attribute attribute for PIN 4."]
    #[inline(always)]
    pub fn pin4(&mut self) -> PIN4_W {
        PIN4_W { w: self }
    }
    #[doc = "Bit 5 - Select secure attribute attribute for PIN 5."]
    #[inline(always)]
    pub fn pin5(&mut self) -> PIN5_W {
        PIN5_W { w: self }
    }
    #[doc = "Bit 6 - Select secure attribute attribute for PIN 6."]
    #[inline(always)]
    pub fn pin6(&mut self) -> PIN6_W {
        PIN6_W { w: self }
    }
    #[doc = "Bit 7 - Select secure attribute attribute for PIN 7."]
    #[inline(always)]
    pub fn pin7(&mut self) -> PIN7_W {
        PIN7_W { w: self }
    }
    #[doc = "Bit 8 - Select secure attribute attribute for PIN 8."]
    #[inline(always)]
    pub fn pin8(&mut self) -> PIN8_W {
        PIN8_W { w: self }
    }
    #[doc = "Bit 9 - Select secure attribute attribute for PIN 9."]
    #[inline(always)]
    pub fn pin9(&mut self) -> PIN9_W {
        PIN9_W { w: self }
    }
    #[doc = "Bit 10 - Select secure attribute attribute for PIN 10."]
    #[inline(always)]
    pub fn pin10(&mut self) -> PIN10_W {
        PIN10_W { w: self }
    }
    #[doc = "Bit 11 - Select secure attribute attribute for PIN 11."]
    #[inline(always)]
    pub fn pin11(&mut self) -> PIN11_W {
        PIN11_W { w: self }
    }
    #[doc = "Bit 12 - Select secure attribute attribute for PIN 12."]
    #[inline(always)]
    pub fn pin12(&mut self) -> PIN12_W {
        PIN12_W { w: self }
    }
    #[doc = "Bit 13 - Select secure attribute attribute for PIN 13."]
    #[inline(always)]
    pub fn pin13(&mut self) -> PIN13_W {
        PIN13_W { w: self }
    }
    #[doc = "Bit 14 - Select secure attribute attribute for PIN 14."]
    #[inline(always)]
    pub fn pin14(&mut self) -> PIN14_W {
        PIN14_W { w: self }
    }
    #[doc = "Bit 15 - Select secure attribute attribute for PIN 15."]
    #[inline(always)]
    pub fn pin15(&mut self) -> PIN15_W {
        PIN15_W { w: self }
    }
    #[doc = "Bit 16 - Select secure attribute attribute for PIN 16."]
    #[inline(always)]
    pub fn pin16(&mut self) -> PIN16_W {
        PIN16_W { w: self }
    }
    #[doc = "Bit 17 - Select secure attribute attribute for PIN 17."]
    #[inline(always)]
    pub fn pin17(&mut self) -> PIN17_W {
        PIN17_W { w: self }
    }
    #[doc = "Bit 18 - Select secure attribute attribute for PIN 18."]
    #[inline(always)]
    pub fn pin18(&mut self) -> PIN18_W {
        PIN18_W { w: self }
    }
    #[doc = "Bit 19 - Select secure attribute attribute for PIN 19."]
    #[inline(always)]
    pub fn pin19(&mut self) -> PIN19_W {
        PIN19_W { w: self }
    }
    #[doc = "Bit 20 - Select secure attribute attribute for PIN 20."]
    #[inline(always)]
    pub fn pin20(&mut self) -> PIN20_W {
        PIN20_W { w: self }
    }
    #[doc = "Bit 21 - Select secure attribute attribute for PIN 21."]
    #[inline(always)]
    pub fn pin21(&mut self) -> PIN21_W {
        PIN21_W { w: self }
    }
    #[doc = "Bit 22 - Select secure attribute attribute for PIN 22."]
    #[inline(always)]
    pub fn pin22(&mut self) -> PIN22_W {
        PIN22_W { w: self }
    }
    #[doc = "Bit 23 - Select secure attribute attribute for PIN 23."]
    #[inline(always)]
    pub fn pin23(&mut self) -> PIN23_W {
        PIN23_W { w: self }
    }
    #[doc = "Bit 24 - Select secure attribute attribute for PIN 24."]
    #[inline(always)]
    pub fn pin24(&mut self) -> PIN24_W {
        PIN24_W { w: self }
    }
    #[doc = "Bit 25 - Select secure attribute attribute for PIN 25."]
    #[inline(always)]
    pub fn pin25(&mut self) -> PIN25_W {
        PIN25_W { w: self }
    }
    #[doc = "Bit 26 - Select secure attribute attribute for PIN 26."]
    #[inline(always)]
    pub fn pin26(&mut self) -> PIN26_W {
        PIN26_W { w: self }
    }
    #[doc = "Bit 27 - Select secure attribute attribute for PIN 27."]
    #[inline(always)]
    pub fn pin27(&mut self) -> PIN27_W {
        PIN27_W { w: self }
    }
    #[doc = "Bit 28 - Select secure attribute attribute for PIN 28."]
    #[inline(always)]
    pub fn pin28(&mut self) -> PIN28_W {
        PIN28_W { w: self }
    }
    #[doc = "Bit 29 - Select secure attribute attribute for PIN 29."]
    #[inline(always)]
    pub fn pin29(&mut self) -> PIN29_W {
        PIN29_W { w: self }
    }
    #[doc = "Bit 30 - Select secure attribute attribute for PIN 30."]
    #[inline(always)]
    pub fn pin30(&mut self) -> PIN30_W {
        PIN30_W { w: self }
    }
    #[doc = "Bit 31 - Select secure attribute attribute for PIN 31."]
    #[inline(always)]
    pub fn pin31(&mut self) -> PIN31_W {
        PIN31_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description cluster: Select between secure and non-secure attribute for pins 0 to 31 of port n.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perm](index.html) module"]
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
