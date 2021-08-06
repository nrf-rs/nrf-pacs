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
#[doc = "Configure instruction fetch permissions from flash region n\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXECUTE_A {
    #[doc = "1: Allow instruction fetches from flash region n"]
    ENABLE = 1,
    #[doc = "0: Block instruction fetches from flash region n"]
    DISABLE = 0,
}
impl From<EXECUTE_A> for bool {
    #[inline(always)]
    fn from(variant: EXECUTE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXECUTE` reader - Configure instruction fetch permissions from flash region n"]
pub struct EXECUTE_R(crate::FieldReader<bool, EXECUTE_A>);
impl EXECUTE_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXECUTE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXECUTE_A {
        match self.bits {
            true => EXECUTE_A::ENABLE,
            false => EXECUTE_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == EXECUTE_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == EXECUTE_A::DISABLE
    }
}
impl core::ops::Deref for EXECUTE_R {
    type Target = crate::FieldReader<bool, EXECUTE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXECUTE` writer - Configure instruction fetch permissions from flash region n"]
pub struct EXECUTE_W<'a> {
    w: &'a mut W,
}
impl<'a> EXECUTE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXECUTE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Allow instruction fetches from flash region n"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EXECUTE_A::ENABLE)
    }
    #[doc = "Block instruction fetches from flash region n"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EXECUTE_A::DISABLE)
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
#[doc = "Configure write permission for flash region n\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRITE_A {
    #[doc = "1: Allow write operation to region n"]
    ENABLE = 1,
    #[doc = "0: Block write operation to region n"]
    DISABLE = 0,
}
impl From<WRITE_A> for bool {
    #[inline(always)]
    fn from(variant: WRITE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRITE` reader - Configure write permission for flash region n"]
pub struct WRITE_R(crate::FieldReader<bool, WRITE_A>);
impl WRITE_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRITE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WRITE_A {
        match self.bits {
            true => WRITE_A::ENABLE,
            false => WRITE_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == WRITE_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == WRITE_A::DISABLE
    }
}
impl core::ops::Deref for WRITE_R {
    type Target = crate::FieldReader<bool, WRITE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRITE` writer - Configure write permission for flash region n"]
pub struct WRITE_W<'a> {
    w: &'a mut W,
}
impl<'a> WRITE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WRITE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Allow write operation to region n"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WRITE_A::ENABLE)
    }
    #[doc = "Block write operation to region n"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WRITE_A::DISABLE)
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
#[doc = "Configure read permissions for flash region n\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READ_A {
    #[doc = "1: Allow read operation from flash region n"]
    ENABLE = 1,
    #[doc = "0: Block read operation from flash region n"]
    DISABLE = 0,
}
impl From<READ_A> for bool {
    #[inline(always)]
    fn from(variant: READ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READ` reader - Configure read permissions for flash region n"]
pub struct READ_R(crate::FieldReader<bool, READ_A>);
impl READ_R {
    pub(crate) fn new(bits: bool) -> Self {
        READ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> READ_A {
        match self.bits {
            true => READ_A::ENABLE,
            false => READ_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == READ_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == READ_A::DISABLE
    }
}
impl core::ops::Deref for READ_R {
    type Target = crate::FieldReader<bool, READ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `READ` writer - Configure read permissions for flash region n"]
pub struct READ_W<'a> {
    w: &'a mut W,
}
impl<'a> READ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: READ_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Allow read operation from flash region n"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(READ_A::ENABLE)
    }
    #[doc = "Block read operation from flash region n"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(READ_A::DISABLE)
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
#[doc = "Security attribute for flash region n\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SECATTR_A {
    #[doc = "0: Flash region n security attribute is non-secure"]
    NON_SECURE = 0,
    #[doc = "1: Flash region n security attribute is secure"]
    SECURE = 1,
}
impl From<SECATTR_A> for bool {
    #[inline(always)]
    fn from(variant: SECATTR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SECATTR` reader - Security attribute for flash region n"]
pub struct SECATTR_R(crate::FieldReader<bool, SECATTR_A>);
impl SECATTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        SECATTR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SECATTR_A {
        match self.bits {
            false => SECATTR_A::NON_SECURE,
            true => SECATTR_A::SECURE,
        }
    }
    #[doc = "Checks if the value of the field is `NON_SECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        **self == SECATTR_A::NON_SECURE
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        **self == SECATTR_A::SECURE
    }
}
impl core::ops::Deref for SECATTR_R {
    type Target = crate::FieldReader<bool, SECATTR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SECATTR` writer - Security attribute for flash region n"]
pub struct SECATTR_W<'a> {
    w: &'a mut W,
}
impl<'a> SECATTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SECATTR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Flash region n security attribute is non-secure"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(SECATTR_A::NON_SECURE)
    }
    #[doc = "Flash region n security attribute is secure"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(SECATTR_A::SECURE)
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
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_A {
    #[doc = "0: This register can be updated"]
    UNLOCKED = 0,
    #[doc = "1: The content of this register can't be changed until the next reset"]
    LOCKED = 1,
}
impl From<LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCK` reader - "]
pub struct LOCK_R(crate::FieldReader<bool, LOCK_A>);
impl LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCK_A {
        match self.bits {
            false => LOCK_A::UNLOCKED,
            true => LOCK_A::LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        **self == LOCK_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        **self == LOCK_A::LOCKED
    }
}
impl core::ops::Deref for LOCK_R {
    type Target = crate::FieldReader<bool, LOCK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK` writer - "]
pub struct LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "This register can be updated"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(LOCK_A::UNLOCKED)
    }
    #[doc = "The content of this register can't be changed until the next reset"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(LOCK_A::LOCKED)
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
impl R {
    #[doc = "Bit 0 - Configure instruction fetch permissions from flash region n"]
    #[inline(always)]
    pub fn execute(&self) -> EXECUTE_R {
        EXECUTE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Configure write permission for flash region n"]
    #[inline(always)]
    pub fn write(&self) -> WRITE_R {
        WRITE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Configure read permissions for flash region n"]
    #[inline(always)]
    pub fn read(&self) -> READ_R {
        READ_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Security attribute for flash region n"]
    #[inline(always)]
    pub fn secattr(&self) -> SECATTR_R {
        SECATTR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Configure instruction fetch permissions from flash region n"]
    #[inline(always)]
    pub fn execute(&mut self) -> EXECUTE_W {
        EXECUTE_W { w: self }
    }
    #[doc = "Bit 1 - Configure write permission for flash region n"]
    #[inline(always)]
    pub fn write(&mut self) -> WRITE_W {
        WRITE_W { w: self }
    }
    #[doc = "Bit 2 - Configure read permissions for flash region n"]
    #[inline(always)]
    pub fn read(&mut self) -> READ_W {
        READ_W { w: self }
    }
    #[doc = "Bit 4 - Security attribute for flash region n"]
    #[inline(always)]
    pub fn secattr(&mut self) -> SECATTR_W {
        SECATTR_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W {
        LOCK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description cluster: Access permissions for flash region n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perm](index.html) module"]
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
#[doc = "`reset()` method sets PERM to value 0x17"]
impl crate::Resettable for PERM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x17
    }
}
