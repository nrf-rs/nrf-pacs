#[doc = "Register `SUBSTATRA` reader"]
pub struct R(crate::R<SUBSTATRA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SUBSTATRA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SUBSTATRA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SUBSTATRA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SUBSTATRA` writer"]
pub struct W(crate::W<SUBSTATRA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SUBSTATRA_SPEC>;
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
impl From<crate::W<SUBSTATRA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SUBSTATRA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Subregion 0 in region n (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR0_A {
    #[doc = "0: No read access occurred in this subregion"]
    NOACCESS = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    ACCESS = 1,
}
impl From<SR0_A> for bool {
    #[inline(always)]
    fn from(variant: SR0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR0` reader - Subregion 0 in region n (write '1' to clear)"]
pub struct SR0_R(crate::FieldReader<bool, SR0_A>);
impl SR0_R {
    pub(crate) fn new(bits: bool) -> Self {
        SR0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR0_A {
        match self.bits {
            false => SR0_A::NOACCESS,
            true => SR0_A::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NOACCESS`"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        **self == SR0_A::NOACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        **self == SR0_A::ACCESS
    }
}
impl core::ops::Deref for SR0_R {
    type Target = crate::FieldReader<bool, SR0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SR0` writer - Subregion 0 in region n (write '1' to clear)"]
pub struct SR0_W<'a> {
    w: &'a mut W,
}
impl<'a> SR0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SR0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR0_A::NOACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut W {
        self.variant(SR0_A::ACCESS)
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
#[doc = "Subregion 1 in region n (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR1_A {
    #[doc = "0: No read access occurred in this subregion"]
    NOACCESS = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    ACCESS = 1,
}
impl From<SR1_A> for bool {
    #[inline(always)]
    fn from(variant: SR1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR1` reader - Subregion 1 in region n (write '1' to clear)"]
pub struct SR1_R(crate::FieldReader<bool, SR1_A>);
impl SR1_R {
    pub(crate) fn new(bits: bool) -> Self {
        SR1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR1_A {
        match self.bits {
            false => SR1_A::NOACCESS,
            true => SR1_A::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NOACCESS`"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        **self == SR1_A::NOACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        **self == SR1_A::ACCESS
    }
}
impl core::ops::Deref for SR1_R {
    type Target = crate::FieldReader<bool, SR1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SR1` writer - Subregion 1 in region n (write '1' to clear)"]
pub struct SR1_W<'a> {
    w: &'a mut W,
}
impl<'a> SR1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SR1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR1_A::NOACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut W {
        self.variant(SR1_A::ACCESS)
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
#[doc = "Subregion 2 in region n (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR2_A {
    #[doc = "0: No read access occurred in this subregion"]
    NOACCESS = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    ACCESS = 1,
}
impl From<SR2_A> for bool {
    #[inline(always)]
    fn from(variant: SR2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR2` reader - Subregion 2 in region n (write '1' to clear)"]
pub struct SR2_R(crate::FieldReader<bool, SR2_A>);
impl SR2_R {
    pub(crate) fn new(bits: bool) -> Self {
        SR2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR2_A {
        match self.bits {
            false => SR2_A::NOACCESS,
            true => SR2_A::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NOACCESS`"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        **self == SR2_A::NOACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        **self == SR2_A::ACCESS
    }
}
impl core::ops::Deref for SR2_R {
    type Target = crate::FieldReader<bool, SR2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SR2` writer - Subregion 2 in region n (write '1' to clear)"]
pub struct SR2_W<'a> {
    w: &'a mut W,
}
impl<'a> SR2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SR2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR2_A::NOACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut W {
        self.variant(SR2_A::ACCESS)
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
#[doc = "Subregion 3 in region n (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR3_A {
    #[doc = "0: No read access occurred in this subregion"]
    NOACCESS = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    ACCESS = 1,
}
impl From<SR3_A> for bool {
    #[inline(always)]
    fn from(variant: SR3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR3` reader - Subregion 3 in region n (write '1' to clear)"]
pub struct SR3_R(crate::FieldReader<bool, SR3_A>);
impl SR3_R {
    pub(crate) fn new(bits: bool) -> Self {
        SR3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR3_A {
        match self.bits {
            false => SR3_A::NOACCESS,
            true => SR3_A::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NOACCESS`"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        **self == SR3_A::NOACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        **self == SR3_A::ACCESS
    }
}
impl core::ops::Deref for SR3_R {
    type Target = crate::FieldReader<bool, SR3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SR3` writer - Subregion 3 in region n (write '1' to clear)"]
pub struct SR3_W<'a> {
    w: &'a mut W,
}
impl<'a> SR3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SR3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR3_A::NOACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut W {
        self.variant(SR3_A::ACCESS)
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
#[doc = "Subregion 4 in region n (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR4_A {
    #[doc = "0: No read access occurred in this subregion"]
    NOACCESS = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    ACCESS = 1,
}
impl From<SR4_A> for bool {
    #[inline(always)]
    fn from(variant: SR4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR4` reader - Subregion 4 in region n (write '1' to clear)"]
pub struct SR4_R(crate::FieldReader<bool, SR4_A>);
impl SR4_R {
    pub(crate) fn new(bits: bool) -> Self {
        SR4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR4_A {
        match self.bits {
            false => SR4_A::NOACCESS,
            true => SR4_A::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NOACCESS`"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        **self == SR4_A::NOACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        **self == SR4_A::ACCESS
    }
}
impl core::ops::Deref for SR4_R {
    type Target = crate::FieldReader<bool, SR4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SR4` writer - Subregion 4 in region n (write '1' to clear)"]
pub struct SR4_W<'a> {
    w: &'a mut W,
}
impl<'a> SR4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SR4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR4_A::NOACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut W {
        self.variant(SR4_A::ACCESS)
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
#[doc = "Subregion 5 in region n (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR5_A {
    #[doc = "0: No read access occurred in this subregion"]
    NOACCESS = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    ACCESS = 1,
}
impl From<SR5_A> for bool {
    #[inline(always)]
    fn from(variant: SR5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR5` reader - Subregion 5 in region n (write '1' to clear)"]
pub struct SR5_R(crate::FieldReader<bool, SR5_A>);
impl SR5_R {
    pub(crate) fn new(bits: bool) -> Self {
        SR5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR5_A {
        match self.bits {
            false => SR5_A::NOACCESS,
            true => SR5_A::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NOACCESS`"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        **self == SR5_A::NOACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        **self == SR5_A::ACCESS
    }
}
impl core::ops::Deref for SR5_R {
    type Target = crate::FieldReader<bool, SR5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SR5` writer - Subregion 5 in region n (write '1' to clear)"]
pub struct SR5_W<'a> {
    w: &'a mut W,
}
impl<'a> SR5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SR5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR5_A::NOACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut W {
        self.variant(SR5_A::ACCESS)
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
#[doc = "Subregion 6 in region n (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR6_A {
    #[doc = "0: No read access occurred in this subregion"]
    NOACCESS = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    ACCESS = 1,
}
impl From<SR6_A> for bool {
    #[inline(always)]
    fn from(variant: SR6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR6` reader - Subregion 6 in region n (write '1' to clear)"]
pub struct SR6_R(crate::FieldReader<bool, SR6_A>);
impl SR6_R {
    pub(crate) fn new(bits: bool) -> Self {
        SR6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR6_A {
        match self.bits {
            false => SR6_A::NOACCESS,
            true => SR6_A::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NOACCESS`"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        **self == SR6_A::NOACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        **self == SR6_A::ACCESS
    }
}
impl core::ops::Deref for SR6_R {
    type Target = crate::FieldReader<bool, SR6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SR6` writer - Subregion 6 in region n (write '1' to clear)"]
pub struct SR6_W<'a> {
    w: &'a mut W,
}
impl<'a> SR6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SR6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR6_A::NOACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut W {
        self.variant(SR6_A::ACCESS)
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
#[doc = "Subregion 7 in region n (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR7_A {
    #[doc = "0: No read access occurred in this subregion"]
    NOACCESS = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    ACCESS = 1,
}
impl From<SR7_A> for bool {
    #[inline(always)]
    fn from(variant: SR7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR7` reader - Subregion 7 in region n (write '1' to clear)"]
pub struct SR7_R(crate::FieldReader<bool, SR7_A>);
impl SR7_R {
    pub(crate) fn new(bits: bool) -> Self {
        SR7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR7_A {
        match self.bits {
            false => SR7_A::NOACCESS,
            true => SR7_A::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NOACCESS`"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        **self == SR7_A::NOACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        **self == SR7_A::ACCESS
    }
}
impl core::ops::Deref for SR7_R {
    type Target = crate::FieldReader<bool, SR7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SR7` writer - Subregion 7 in region n (write '1' to clear)"]
pub struct SR7_W<'a> {
    w: &'a mut W,
}
impl<'a> SR7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SR7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR7_A::NOACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut W {
        self.variant(SR7_A::ACCESS)
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
#[doc = "Subregion 8 in region n (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR8_A {
    #[doc = "0: No read access occurred in this subregion"]
    NOACCESS = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    ACCESS = 1,
}
impl From<SR8_A> for bool {
    #[inline(always)]
    fn from(variant: SR8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR8` reader - Subregion 8 in region n (write '1' to clear)"]
pub struct SR8_R(crate::FieldReader<bool, SR8_A>);
impl SR8_R {
    pub(crate) fn new(bits: bool) -> Self {
        SR8_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR8_A {
        match self.bits {
            false => SR8_A::NOACCESS,
            true => SR8_A::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NOACCESS`"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        **self == SR8_A::NOACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        **self == SR8_A::ACCESS
    }
}
impl core::ops::Deref for SR8_R {
    type Target = crate::FieldReader<bool, SR8_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SR8` writer - Subregion 8 in region n (write '1' to clear)"]
pub struct SR8_W<'a> {
    w: &'a mut W,
}
impl<'a> SR8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SR8_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR8_A::NOACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut W {
        self.variant(SR8_A::ACCESS)
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
#[doc = "Subregion 9 in region n (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR9_A {
    #[doc = "0: No read access occurred in this subregion"]
    NOACCESS = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    ACCESS = 1,
}
impl From<SR9_A> for bool {
    #[inline(always)]
    fn from(variant: SR9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR9` reader - Subregion 9 in region n (write '1' to clear)"]
pub struct SR9_R(crate::FieldReader<bool, SR9_A>);
impl SR9_R {
    pub(crate) fn new(bits: bool) -> Self {
        SR9_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR9_A {
        match self.bits {
            false => SR9_A::NOACCESS,
            true => SR9_A::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NOACCESS`"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        **self == SR9_A::NOACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        **self == SR9_A::ACCESS
    }
}
impl core::ops::Deref for SR9_R {
    type Target = crate::FieldReader<bool, SR9_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SR9` writer - Subregion 9 in region n (write '1' to clear)"]
pub struct SR9_W<'a> {
    w: &'a mut W,
}
impl<'a> SR9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SR9_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR9_A::NOACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut W {
        self.variant(SR9_A::ACCESS)
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
#[doc = "Subregion 10 in region n (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR10_A {
    #[doc = "0: No read access occurred in this subregion"]
    NOACCESS = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    ACCESS = 1,
}
impl From<SR10_A> for bool {
    #[inline(always)]
    fn from(variant: SR10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR10` reader - Subregion 10 in region n (write '1' to clear)"]
pub struct SR10_R(crate::FieldReader<bool, SR10_A>);
impl SR10_R {
    pub(crate) fn new(bits: bool) -> Self {
        SR10_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR10_A {
        match self.bits {
            false => SR10_A::NOACCESS,
            true => SR10_A::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NOACCESS`"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        **self == SR10_A::NOACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        **self == SR10_A::ACCESS
    }
}
impl core::ops::Deref for SR10_R {
    type Target = crate::FieldReader<bool, SR10_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SR10` writer - Subregion 10 in region n (write '1' to clear)"]
pub struct SR10_W<'a> {
    w: &'a mut W,
}
impl<'a> SR10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SR10_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR10_A::NOACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut W {
        self.variant(SR10_A::ACCESS)
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
#[doc = "Subregion 11 in region n (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR11_A {
    #[doc = "0: No read access occurred in this subregion"]
    NOACCESS = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    ACCESS = 1,
}
impl From<SR11_A> for bool {
    #[inline(always)]
    fn from(variant: SR11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR11` reader - Subregion 11 in region n (write '1' to clear)"]
pub struct SR11_R(crate::FieldReader<bool, SR11_A>);
impl SR11_R {
    pub(crate) fn new(bits: bool) -> Self {
        SR11_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR11_A {
        match self.bits {
            false => SR11_A::NOACCESS,
            true => SR11_A::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NOACCESS`"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        **self == SR11_A::NOACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        **self == SR11_A::ACCESS
    }
}
impl core::ops::Deref for SR11_R {
    type Target = crate::FieldReader<bool, SR11_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SR11` writer - Subregion 11 in region n (write '1' to clear)"]
pub struct SR11_W<'a> {
    w: &'a mut W,
}
impl<'a> SR11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SR11_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR11_A::NOACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut W {
        self.variant(SR11_A::ACCESS)
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
#[doc = "Subregion 12 in region n (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR12_A {
    #[doc = "0: No read access occurred in this subregion"]
    NOACCESS = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    ACCESS = 1,
}
impl From<SR12_A> for bool {
    #[inline(always)]
    fn from(variant: SR12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR12` reader - Subregion 12 in region n (write '1' to clear)"]
pub struct SR12_R(crate::FieldReader<bool, SR12_A>);
impl SR12_R {
    pub(crate) fn new(bits: bool) -> Self {
        SR12_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR12_A {
        match self.bits {
            false => SR12_A::NOACCESS,
            true => SR12_A::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NOACCESS`"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        **self == SR12_A::NOACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        **self == SR12_A::ACCESS
    }
}
impl core::ops::Deref for SR12_R {
    type Target = crate::FieldReader<bool, SR12_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SR12` writer - Subregion 12 in region n (write '1' to clear)"]
pub struct SR12_W<'a> {
    w: &'a mut W,
}
impl<'a> SR12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SR12_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR12_A::NOACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut W {
        self.variant(SR12_A::ACCESS)
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
#[doc = "Subregion 13 in region n (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR13_A {
    #[doc = "0: No read access occurred in this subregion"]
    NOACCESS = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    ACCESS = 1,
}
impl From<SR13_A> for bool {
    #[inline(always)]
    fn from(variant: SR13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR13` reader - Subregion 13 in region n (write '1' to clear)"]
pub struct SR13_R(crate::FieldReader<bool, SR13_A>);
impl SR13_R {
    pub(crate) fn new(bits: bool) -> Self {
        SR13_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR13_A {
        match self.bits {
            false => SR13_A::NOACCESS,
            true => SR13_A::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NOACCESS`"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        **self == SR13_A::NOACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        **self == SR13_A::ACCESS
    }
}
impl core::ops::Deref for SR13_R {
    type Target = crate::FieldReader<bool, SR13_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SR13` writer - Subregion 13 in region n (write '1' to clear)"]
pub struct SR13_W<'a> {
    w: &'a mut W,
}
impl<'a> SR13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SR13_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR13_A::NOACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut W {
        self.variant(SR13_A::ACCESS)
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
#[doc = "Subregion 14 in region n (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR14_A {
    #[doc = "0: No read access occurred in this subregion"]
    NOACCESS = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    ACCESS = 1,
}
impl From<SR14_A> for bool {
    #[inline(always)]
    fn from(variant: SR14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR14` reader - Subregion 14 in region n (write '1' to clear)"]
pub struct SR14_R(crate::FieldReader<bool, SR14_A>);
impl SR14_R {
    pub(crate) fn new(bits: bool) -> Self {
        SR14_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR14_A {
        match self.bits {
            false => SR14_A::NOACCESS,
            true => SR14_A::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NOACCESS`"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        **self == SR14_A::NOACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        **self == SR14_A::ACCESS
    }
}
impl core::ops::Deref for SR14_R {
    type Target = crate::FieldReader<bool, SR14_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SR14` writer - Subregion 14 in region n (write '1' to clear)"]
pub struct SR14_W<'a> {
    w: &'a mut W,
}
impl<'a> SR14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SR14_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR14_A::NOACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut W {
        self.variant(SR14_A::ACCESS)
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
#[doc = "Subregion 15 in region n (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR15_A {
    #[doc = "0: No read access occurred in this subregion"]
    NOACCESS = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    ACCESS = 1,
}
impl From<SR15_A> for bool {
    #[inline(always)]
    fn from(variant: SR15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR15` reader - Subregion 15 in region n (write '1' to clear)"]
pub struct SR15_R(crate::FieldReader<bool, SR15_A>);
impl SR15_R {
    pub(crate) fn new(bits: bool) -> Self {
        SR15_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR15_A {
        match self.bits {
            false => SR15_A::NOACCESS,
            true => SR15_A::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NOACCESS`"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        **self == SR15_A::NOACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        **self == SR15_A::ACCESS
    }
}
impl core::ops::Deref for SR15_R {
    type Target = crate::FieldReader<bool, SR15_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SR15` writer - Subregion 15 in region n (write '1' to clear)"]
pub struct SR15_W<'a> {
    w: &'a mut W,
}
impl<'a> SR15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SR15_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR15_A::NOACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut W {
        self.variant(SR15_A::ACCESS)
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
#[doc = "Subregion 16 in region n (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR16_A {
    #[doc = "0: No read access occurred in this subregion"]
    NOACCESS = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    ACCESS = 1,
}
impl From<SR16_A> for bool {
    #[inline(always)]
    fn from(variant: SR16_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR16` reader - Subregion 16 in region n (write '1' to clear)"]
pub struct SR16_R(crate::FieldReader<bool, SR16_A>);
impl SR16_R {
    pub(crate) fn new(bits: bool) -> Self {
        SR16_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR16_A {
        match self.bits {
            false => SR16_A::NOACCESS,
            true => SR16_A::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NOACCESS`"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        **self == SR16_A::NOACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        **self == SR16_A::ACCESS
    }
}
impl core::ops::Deref for SR16_R {
    type Target = crate::FieldReader<bool, SR16_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SR16` writer - Subregion 16 in region n (write '1' to clear)"]
pub struct SR16_W<'a> {
    w: &'a mut W,
}
impl<'a> SR16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SR16_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR16_A::NOACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut W {
        self.variant(SR16_A::ACCESS)
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
#[doc = "Subregion 17 in region n (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR17_A {
    #[doc = "0: No read access occurred in this subregion"]
    NOACCESS = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    ACCESS = 1,
}
impl From<SR17_A> for bool {
    #[inline(always)]
    fn from(variant: SR17_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR17` reader - Subregion 17 in region n (write '1' to clear)"]
pub struct SR17_R(crate::FieldReader<bool, SR17_A>);
impl SR17_R {
    pub(crate) fn new(bits: bool) -> Self {
        SR17_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR17_A {
        match self.bits {
            false => SR17_A::NOACCESS,
            true => SR17_A::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NOACCESS`"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        **self == SR17_A::NOACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        **self == SR17_A::ACCESS
    }
}
impl core::ops::Deref for SR17_R {
    type Target = crate::FieldReader<bool, SR17_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SR17` writer - Subregion 17 in region n (write '1' to clear)"]
pub struct SR17_W<'a> {
    w: &'a mut W,
}
impl<'a> SR17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SR17_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR17_A::NOACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut W {
        self.variant(SR17_A::ACCESS)
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
#[doc = "Subregion 18 in region n (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR18_A {
    #[doc = "0: No read access occurred in this subregion"]
    NOACCESS = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    ACCESS = 1,
}
impl From<SR18_A> for bool {
    #[inline(always)]
    fn from(variant: SR18_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR18` reader - Subregion 18 in region n (write '1' to clear)"]
pub struct SR18_R(crate::FieldReader<bool, SR18_A>);
impl SR18_R {
    pub(crate) fn new(bits: bool) -> Self {
        SR18_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR18_A {
        match self.bits {
            false => SR18_A::NOACCESS,
            true => SR18_A::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NOACCESS`"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        **self == SR18_A::NOACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        **self == SR18_A::ACCESS
    }
}
impl core::ops::Deref for SR18_R {
    type Target = crate::FieldReader<bool, SR18_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SR18` writer - Subregion 18 in region n (write '1' to clear)"]
pub struct SR18_W<'a> {
    w: &'a mut W,
}
impl<'a> SR18_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SR18_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR18_A::NOACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut W {
        self.variant(SR18_A::ACCESS)
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
#[doc = "Subregion 19 in region n (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR19_A {
    #[doc = "0: No read access occurred in this subregion"]
    NOACCESS = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    ACCESS = 1,
}
impl From<SR19_A> for bool {
    #[inline(always)]
    fn from(variant: SR19_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR19` reader - Subregion 19 in region n (write '1' to clear)"]
pub struct SR19_R(crate::FieldReader<bool, SR19_A>);
impl SR19_R {
    pub(crate) fn new(bits: bool) -> Self {
        SR19_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR19_A {
        match self.bits {
            false => SR19_A::NOACCESS,
            true => SR19_A::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NOACCESS`"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        **self == SR19_A::NOACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        **self == SR19_A::ACCESS
    }
}
impl core::ops::Deref for SR19_R {
    type Target = crate::FieldReader<bool, SR19_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SR19` writer - Subregion 19 in region n (write '1' to clear)"]
pub struct SR19_W<'a> {
    w: &'a mut W,
}
impl<'a> SR19_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SR19_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR19_A::NOACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut W {
        self.variant(SR19_A::ACCESS)
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
#[doc = "Subregion 20 in region n (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR20_A {
    #[doc = "0: No read access occurred in this subregion"]
    NOACCESS = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    ACCESS = 1,
}
impl From<SR20_A> for bool {
    #[inline(always)]
    fn from(variant: SR20_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR20` reader - Subregion 20 in region n (write '1' to clear)"]
pub struct SR20_R(crate::FieldReader<bool, SR20_A>);
impl SR20_R {
    pub(crate) fn new(bits: bool) -> Self {
        SR20_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR20_A {
        match self.bits {
            false => SR20_A::NOACCESS,
            true => SR20_A::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NOACCESS`"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        **self == SR20_A::NOACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        **self == SR20_A::ACCESS
    }
}
impl core::ops::Deref for SR20_R {
    type Target = crate::FieldReader<bool, SR20_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SR20` writer - Subregion 20 in region n (write '1' to clear)"]
pub struct SR20_W<'a> {
    w: &'a mut W,
}
impl<'a> SR20_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SR20_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR20_A::NOACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut W {
        self.variant(SR20_A::ACCESS)
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
#[doc = "Subregion 21 in region n (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR21_A {
    #[doc = "0: No read access occurred in this subregion"]
    NOACCESS = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    ACCESS = 1,
}
impl From<SR21_A> for bool {
    #[inline(always)]
    fn from(variant: SR21_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR21` reader - Subregion 21 in region n (write '1' to clear)"]
pub struct SR21_R(crate::FieldReader<bool, SR21_A>);
impl SR21_R {
    pub(crate) fn new(bits: bool) -> Self {
        SR21_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR21_A {
        match self.bits {
            false => SR21_A::NOACCESS,
            true => SR21_A::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NOACCESS`"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        **self == SR21_A::NOACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        **self == SR21_A::ACCESS
    }
}
impl core::ops::Deref for SR21_R {
    type Target = crate::FieldReader<bool, SR21_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SR21` writer - Subregion 21 in region n (write '1' to clear)"]
pub struct SR21_W<'a> {
    w: &'a mut W,
}
impl<'a> SR21_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SR21_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR21_A::NOACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut W {
        self.variant(SR21_A::ACCESS)
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
#[doc = "Subregion 22 in region n (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR22_A {
    #[doc = "0: No read access occurred in this subregion"]
    NOACCESS = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    ACCESS = 1,
}
impl From<SR22_A> for bool {
    #[inline(always)]
    fn from(variant: SR22_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR22` reader - Subregion 22 in region n (write '1' to clear)"]
pub struct SR22_R(crate::FieldReader<bool, SR22_A>);
impl SR22_R {
    pub(crate) fn new(bits: bool) -> Self {
        SR22_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR22_A {
        match self.bits {
            false => SR22_A::NOACCESS,
            true => SR22_A::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NOACCESS`"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        **self == SR22_A::NOACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        **self == SR22_A::ACCESS
    }
}
impl core::ops::Deref for SR22_R {
    type Target = crate::FieldReader<bool, SR22_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SR22` writer - Subregion 22 in region n (write '1' to clear)"]
pub struct SR22_W<'a> {
    w: &'a mut W,
}
impl<'a> SR22_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SR22_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR22_A::NOACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut W {
        self.variant(SR22_A::ACCESS)
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
#[doc = "Subregion 23 in region n (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR23_A {
    #[doc = "0: No read access occurred in this subregion"]
    NOACCESS = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    ACCESS = 1,
}
impl From<SR23_A> for bool {
    #[inline(always)]
    fn from(variant: SR23_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR23` reader - Subregion 23 in region n (write '1' to clear)"]
pub struct SR23_R(crate::FieldReader<bool, SR23_A>);
impl SR23_R {
    pub(crate) fn new(bits: bool) -> Self {
        SR23_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR23_A {
        match self.bits {
            false => SR23_A::NOACCESS,
            true => SR23_A::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NOACCESS`"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        **self == SR23_A::NOACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        **self == SR23_A::ACCESS
    }
}
impl core::ops::Deref for SR23_R {
    type Target = crate::FieldReader<bool, SR23_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SR23` writer - Subregion 23 in region n (write '1' to clear)"]
pub struct SR23_W<'a> {
    w: &'a mut W,
}
impl<'a> SR23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SR23_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR23_A::NOACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut W {
        self.variant(SR23_A::ACCESS)
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
#[doc = "Subregion 24 in region n (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR24_A {
    #[doc = "0: No read access occurred in this subregion"]
    NOACCESS = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    ACCESS = 1,
}
impl From<SR24_A> for bool {
    #[inline(always)]
    fn from(variant: SR24_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR24` reader - Subregion 24 in region n (write '1' to clear)"]
pub struct SR24_R(crate::FieldReader<bool, SR24_A>);
impl SR24_R {
    pub(crate) fn new(bits: bool) -> Self {
        SR24_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR24_A {
        match self.bits {
            false => SR24_A::NOACCESS,
            true => SR24_A::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NOACCESS`"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        **self == SR24_A::NOACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        **self == SR24_A::ACCESS
    }
}
impl core::ops::Deref for SR24_R {
    type Target = crate::FieldReader<bool, SR24_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SR24` writer - Subregion 24 in region n (write '1' to clear)"]
pub struct SR24_W<'a> {
    w: &'a mut W,
}
impl<'a> SR24_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SR24_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR24_A::NOACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut W {
        self.variant(SR24_A::ACCESS)
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
#[doc = "Subregion 25 in region n (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR25_A {
    #[doc = "0: No read access occurred in this subregion"]
    NOACCESS = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    ACCESS = 1,
}
impl From<SR25_A> for bool {
    #[inline(always)]
    fn from(variant: SR25_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR25` reader - Subregion 25 in region n (write '1' to clear)"]
pub struct SR25_R(crate::FieldReader<bool, SR25_A>);
impl SR25_R {
    pub(crate) fn new(bits: bool) -> Self {
        SR25_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR25_A {
        match self.bits {
            false => SR25_A::NOACCESS,
            true => SR25_A::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NOACCESS`"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        **self == SR25_A::NOACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        **self == SR25_A::ACCESS
    }
}
impl core::ops::Deref for SR25_R {
    type Target = crate::FieldReader<bool, SR25_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SR25` writer - Subregion 25 in region n (write '1' to clear)"]
pub struct SR25_W<'a> {
    w: &'a mut W,
}
impl<'a> SR25_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SR25_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR25_A::NOACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut W {
        self.variant(SR25_A::ACCESS)
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
#[doc = "Subregion 26 in region n (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR26_A {
    #[doc = "0: No read access occurred in this subregion"]
    NOACCESS = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    ACCESS = 1,
}
impl From<SR26_A> for bool {
    #[inline(always)]
    fn from(variant: SR26_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR26` reader - Subregion 26 in region n (write '1' to clear)"]
pub struct SR26_R(crate::FieldReader<bool, SR26_A>);
impl SR26_R {
    pub(crate) fn new(bits: bool) -> Self {
        SR26_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR26_A {
        match self.bits {
            false => SR26_A::NOACCESS,
            true => SR26_A::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NOACCESS`"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        **self == SR26_A::NOACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        **self == SR26_A::ACCESS
    }
}
impl core::ops::Deref for SR26_R {
    type Target = crate::FieldReader<bool, SR26_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SR26` writer - Subregion 26 in region n (write '1' to clear)"]
pub struct SR26_W<'a> {
    w: &'a mut W,
}
impl<'a> SR26_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SR26_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR26_A::NOACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut W {
        self.variant(SR26_A::ACCESS)
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
#[doc = "Subregion 27 in region n (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR27_A {
    #[doc = "0: No read access occurred in this subregion"]
    NOACCESS = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    ACCESS = 1,
}
impl From<SR27_A> for bool {
    #[inline(always)]
    fn from(variant: SR27_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR27` reader - Subregion 27 in region n (write '1' to clear)"]
pub struct SR27_R(crate::FieldReader<bool, SR27_A>);
impl SR27_R {
    pub(crate) fn new(bits: bool) -> Self {
        SR27_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR27_A {
        match self.bits {
            false => SR27_A::NOACCESS,
            true => SR27_A::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NOACCESS`"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        **self == SR27_A::NOACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        **self == SR27_A::ACCESS
    }
}
impl core::ops::Deref for SR27_R {
    type Target = crate::FieldReader<bool, SR27_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SR27` writer - Subregion 27 in region n (write '1' to clear)"]
pub struct SR27_W<'a> {
    w: &'a mut W,
}
impl<'a> SR27_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SR27_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR27_A::NOACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut W {
        self.variant(SR27_A::ACCESS)
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
#[doc = "Subregion 28 in region n (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR28_A {
    #[doc = "0: No read access occurred in this subregion"]
    NOACCESS = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    ACCESS = 1,
}
impl From<SR28_A> for bool {
    #[inline(always)]
    fn from(variant: SR28_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR28` reader - Subregion 28 in region n (write '1' to clear)"]
pub struct SR28_R(crate::FieldReader<bool, SR28_A>);
impl SR28_R {
    pub(crate) fn new(bits: bool) -> Self {
        SR28_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR28_A {
        match self.bits {
            false => SR28_A::NOACCESS,
            true => SR28_A::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NOACCESS`"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        **self == SR28_A::NOACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        **self == SR28_A::ACCESS
    }
}
impl core::ops::Deref for SR28_R {
    type Target = crate::FieldReader<bool, SR28_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SR28` writer - Subregion 28 in region n (write '1' to clear)"]
pub struct SR28_W<'a> {
    w: &'a mut W,
}
impl<'a> SR28_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SR28_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR28_A::NOACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut W {
        self.variant(SR28_A::ACCESS)
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
#[doc = "Subregion 29 in region n (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR29_A {
    #[doc = "0: No read access occurred in this subregion"]
    NOACCESS = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    ACCESS = 1,
}
impl From<SR29_A> for bool {
    #[inline(always)]
    fn from(variant: SR29_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR29` reader - Subregion 29 in region n (write '1' to clear)"]
pub struct SR29_R(crate::FieldReader<bool, SR29_A>);
impl SR29_R {
    pub(crate) fn new(bits: bool) -> Self {
        SR29_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR29_A {
        match self.bits {
            false => SR29_A::NOACCESS,
            true => SR29_A::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NOACCESS`"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        **self == SR29_A::NOACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        **self == SR29_A::ACCESS
    }
}
impl core::ops::Deref for SR29_R {
    type Target = crate::FieldReader<bool, SR29_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SR29` writer - Subregion 29 in region n (write '1' to clear)"]
pub struct SR29_W<'a> {
    w: &'a mut W,
}
impl<'a> SR29_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SR29_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR29_A::NOACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut W {
        self.variant(SR29_A::ACCESS)
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
#[doc = "Subregion 30 in region n (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR30_A {
    #[doc = "0: No read access occurred in this subregion"]
    NOACCESS = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    ACCESS = 1,
}
impl From<SR30_A> for bool {
    #[inline(always)]
    fn from(variant: SR30_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR30` reader - Subregion 30 in region n (write '1' to clear)"]
pub struct SR30_R(crate::FieldReader<bool, SR30_A>);
impl SR30_R {
    pub(crate) fn new(bits: bool) -> Self {
        SR30_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR30_A {
        match self.bits {
            false => SR30_A::NOACCESS,
            true => SR30_A::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NOACCESS`"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        **self == SR30_A::NOACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        **self == SR30_A::ACCESS
    }
}
impl core::ops::Deref for SR30_R {
    type Target = crate::FieldReader<bool, SR30_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SR30` writer - Subregion 30 in region n (write '1' to clear)"]
pub struct SR30_W<'a> {
    w: &'a mut W,
}
impl<'a> SR30_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SR30_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR30_A::NOACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut W {
        self.variant(SR30_A::ACCESS)
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
#[doc = "Subregion 31 in region n (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR31_A {
    #[doc = "0: No read access occurred in this subregion"]
    NOACCESS = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    ACCESS = 1,
}
impl From<SR31_A> for bool {
    #[inline(always)]
    fn from(variant: SR31_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR31` reader - Subregion 31 in region n (write '1' to clear)"]
pub struct SR31_R(crate::FieldReader<bool, SR31_A>);
impl SR31_R {
    pub(crate) fn new(bits: bool) -> Self {
        SR31_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR31_A {
        match self.bits {
            false => SR31_A::NOACCESS,
            true => SR31_A::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NOACCESS`"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        **self == SR31_A::NOACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        **self == SR31_A::ACCESS
    }
}
impl core::ops::Deref for SR31_R {
    type Target = crate::FieldReader<bool, SR31_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SR31` writer - Subregion 31 in region n (write '1' to clear)"]
pub struct SR31_W<'a> {
    w: &'a mut W,
}
impl<'a> SR31_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SR31_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR31_A::NOACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut W {
        self.variant(SR31_A::ACCESS)
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
    #[doc = "Bit 0 - Subregion 0 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr0(&self) -> SR0_R {
        SR0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Subregion 1 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr1(&self) -> SR1_R {
        SR1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Subregion 2 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr2(&self) -> SR2_R {
        SR2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Subregion 3 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr3(&self) -> SR3_R {
        SR3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Subregion 4 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr4(&self) -> SR4_R {
        SR4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Subregion 5 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr5(&self) -> SR5_R {
        SR5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Subregion 6 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr6(&self) -> SR6_R {
        SR6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Subregion 7 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr7(&self) -> SR7_R {
        SR7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Subregion 8 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr8(&self) -> SR8_R {
        SR8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Subregion 9 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr9(&self) -> SR9_R {
        SR9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Subregion 10 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr10(&self) -> SR10_R {
        SR10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Subregion 11 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr11(&self) -> SR11_R {
        SR11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Subregion 12 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr12(&self) -> SR12_R {
        SR12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Subregion 13 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr13(&self) -> SR13_R {
        SR13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Subregion 14 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr14(&self) -> SR14_R {
        SR14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Subregion 15 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr15(&self) -> SR15_R {
        SR15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Subregion 16 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr16(&self) -> SR16_R {
        SR16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Subregion 17 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr17(&self) -> SR17_R {
        SR17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Subregion 18 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr18(&self) -> SR18_R {
        SR18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Subregion 19 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr19(&self) -> SR19_R {
        SR19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Subregion 20 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr20(&self) -> SR20_R {
        SR20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Subregion 21 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr21(&self) -> SR21_R {
        SR21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Subregion 22 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr22(&self) -> SR22_R {
        SR22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Subregion 23 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr23(&self) -> SR23_R {
        SR23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Subregion 24 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr24(&self) -> SR24_R {
        SR24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Subregion 25 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr25(&self) -> SR25_R {
        SR25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Subregion 26 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr26(&self) -> SR26_R {
        SR26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Subregion 27 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr27(&self) -> SR27_R {
        SR27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Subregion 28 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr28(&self) -> SR28_R {
        SR28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Subregion 29 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr29(&self) -> SR29_R {
        SR29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Subregion 30 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr30(&self) -> SR30_R {
        SR30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Subregion 31 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr31(&self) -> SR31_R {
        SR31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Subregion 0 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr0(&mut self) -> SR0_W {
        SR0_W { w: self }
    }
    #[doc = "Bit 1 - Subregion 1 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr1(&mut self) -> SR1_W {
        SR1_W { w: self }
    }
    #[doc = "Bit 2 - Subregion 2 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr2(&mut self) -> SR2_W {
        SR2_W { w: self }
    }
    #[doc = "Bit 3 - Subregion 3 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr3(&mut self) -> SR3_W {
        SR3_W { w: self }
    }
    #[doc = "Bit 4 - Subregion 4 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr4(&mut self) -> SR4_W {
        SR4_W { w: self }
    }
    #[doc = "Bit 5 - Subregion 5 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr5(&mut self) -> SR5_W {
        SR5_W { w: self }
    }
    #[doc = "Bit 6 - Subregion 6 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr6(&mut self) -> SR6_W {
        SR6_W { w: self }
    }
    #[doc = "Bit 7 - Subregion 7 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr7(&mut self) -> SR7_W {
        SR7_W { w: self }
    }
    #[doc = "Bit 8 - Subregion 8 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr8(&mut self) -> SR8_W {
        SR8_W { w: self }
    }
    #[doc = "Bit 9 - Subregion 9 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr9(&mut self) -> SR9_W {
        SR9_W { w: self }
    }
    #[doc = "Bit 10 - Subregion 10 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr10(&mut self) -> SR10_W {
        SR10_W { w: self }
    }
    #[doc = "Bit 11 - Subregion 11 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr11(&mut self) -> SR11_W {
        SR11_W { w: self }
    }
    #[doc = "Bit 12 - Subregion 12 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr12(&mut self) -> SR12_W {
        SR12_W { w: self }
    }
    #[doc = "Bit 13 - Subregion 13 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr13(&mut self) -> SR13_W {
        SR13_W { w: self }
    }
    #[doc = "Bit 14 - Subregion 14 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr14(&mut self) -> SR14_W {
        SR14_W { w: self }
    }
    #[doc = "Bit 15 - Subregion 15 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr15(&mut self) -> SR15_W {
        SR15_W { w: self }
    }
    #[doc = "Bit 16 - Subregion 16 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr16(&mut self) -> SR16_W {
        SR16_W { w: self }
    }
    #[doc = "Bit 17 - Subregion 17 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr17(&mut self) -> SR17_W {
        SR17_W { w: self }
    }
    #[doc = "Bit 18 - Subregion 18 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr18(&mut self) -> SR18_W {
        SR18_W { w: self }
    }
    #[doc = "Bit 19 - Subregion 19 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr19(&mut self) -> SR19_W {
        SR19_W { w: self }
    }
    #[doc = "Bit 20 - Subregion 20 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr20(&mut self) -> SR20_W {
        SR20_W { w: self }
    }
    #[doc = "Bit 21 - Subregion 21 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr21(&mut self) -> SR21_W {
        SR21_W { w: self }
    }
    #[doc = "Bit 22 - Subregion 22 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr22(&mut self) -> SR22_W {
        SR22_W { w: self }
    }
    #[doc = "Bit 23 - Subregion 23 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr23(&mut self) -> SR23_W {
        SR23_W { w: self }
    }
    #[doc = "Bit 24 - Subregion 24 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr24(&mut self) -> SR24_W {
        SR24_W { w: self }
    }
    #[doc = "Bit 25 - Subregion 25 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr25(&mut self) -> SR25_W {
        SR25_W { w: self }
    }
    #[doc = "Bit 26 - Subregion 26 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr26(&mut self) -> SR26_W {
        SR26_W { w: self }
    }
    #[doc = "Bit 27 - Subregion 27 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr27(&mut self) -> SR27_W {
        SR27_W { w: self }
    }
    #[doc = "Bit 28 - Subregion 28 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr28(&mut self) -> SR28_W {
        SR28_W { w: self }
    }
    #[doc = "Bit 29 - Subregion 29 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr29(&mut self) -> SR29_W {
        SR29_W { w: self }
    }
    #[doc = "Bit 30 - Subregion 30 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr30(&mut self) -> SR30_W {
        SR30_W { w: self }
    }
    #[doc = "Bit 31 - Subregion 31 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr31(&mut self) -> SR31_W {
        SR31_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description cluster\\[n\\]: Source of event/interrupt in region n, read access detected while corresponding subregion was enabled for watching\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [substatra](index.html) module"]
pub struct SUBSTATRA_SPEC;
impl crate::RegisterSpec for SUBSTATRA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [substatra::R](R) reader structure"]
impl crate::Readable for SUBSTATRA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [substatra::W](W) writer structure"]
impl crate::Writable for SUBSTATRA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SUBSTATRA to value 0"]
impl crate::Resettable for SUBSTATRA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
