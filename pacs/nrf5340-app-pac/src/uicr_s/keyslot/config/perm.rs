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
#[doc = "Field `WRITE` reader - Write permission for key slot"]
pub type WRITE_R = crate::BitReader<WRITE_A>;
#[doc = "Write permission for key slot\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRITE_A {
    #[doc = "0: Disable write to the key value registers"]
    DISABLED = 0,
    #[doc = "1: Enable write to the key value registers"]
    ENABLED = 1,
}
impl From<WRITE_A> for bool {
    #[inline(always)]
    fn from(variant: WRITE_A) -> Self {
        variant as u8 != 0
    }
}
impl WRITE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WRITE_A {
        match self.bits {
            false => WRITE_A::DISABLED,
            true => WRITE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WRITE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WRITE_A::ENABLED
    }
}
#[doc = "Field `WRITE` writer - Write permission for key slot"]
pub type WRITE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERM_SPEC, WRITE_A, O>;
impl<'a, const O: u8> WRITE_W<'a, O> {
    #[doc = "Disable write to the key value registers"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WRITE_A::DISABLED)
    }
    #[doc = "Enable write to the key value registers"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WRITE_A::ENABLED)
    }
}
#[doc = "Field `READ` reader - Read permission for key slot"]
pub type READ_R = crate::BitReader<READ_A>;
#[doc = "Read permission for key slot\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READ_A {
    #[doc = "0: Disable read from key value registers"]
    DISABLED = 0,
    #[doc = "1: Enable read from key value registers"]
    ENABLED = 1,
}
impl From<READ_A> for bool {
    #[inline(always)]
    fn from(variant: READ_A) -> Self {
        variant as u8 != 0
    }
}
impl READ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> READ_A {
        match self.bits {
            false => READ_A::DISABLED,
            true => READ_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == READ_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == READ_A::ENABLED
    }
}
#[doc = "Field `READ` writer - Read permission for key slot"]
pub type READ_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERM_SPEC, READ_A, O>;
impl<'a, const O: u8> READ_W<'a, O> {
    #[doc = "Disable read from key value registers"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(READ_A::DISABLED)
    }
    #[doc = "Enable read from key value registers"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(READ_A::ENABLED)
    }
}
#[doc = "Field `PUSH` reader - Push permission for key slot"]
pub type PUSH_R = crate::BitReader<PUSH_A>;
#[doc = "Push permission for key slot\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PUSH_A {
    #[doc = "0: Disable pushing of key value registers over secure APB, but can be read if field READ is Enabled"]
    DISABLED = 0,
    #[doc = "1: Enable pushing of key value registers over secure APB. Register KEYSLOT.CONFIGn.DEST must contain a valid destination address!"]
    ENABLED = 1,
}
impl From<PUSH_A> for bool {
    #[inline(always)]
    fn from(variant: PUSH_A) -> Self {
        variant as u8 != 0
    }
}
impl PUSH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PUSH_A {
        match self.bits {
            false => PUSH_A::DISABLED,
            true => PUSH_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PUSH_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PUSH_A::ENABLED
    }
}
#[doc = "Field `PUSH` writer - Push permission for key slot"]
pub type PUSH_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERM_SPEC, PUSH_A, O>;
impl<'a, const O: u8> PUSH_W<'a, O> {
    #[doc = "Disable pushing of key value registers over secure APB, but can be read if field READ is Enabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PUSH_A::DISABLED)
    }
    #[doc = "Enable pushing of key value registers over secure APB. Register KEYSLOT.CONFIGn.DEST must contain a valid destination address!"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PUSH_A::ENABLED)
    }
}
#[doc = "Field `STATE` reader - Revocation state for the key slot"]
pub type STATE_R = crate::BitReader<STATE_A>;
#[doc = "Revocation state for the key slot\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STATE_A {
    #[doc = "0: Key value registers can no longer be read or pushed"]
    REVOKED = 0,
    #[doc = "1: Key value registers are readable (if enabled) and can be pushed (if enabled)"]
    ACTIVE = 1,
}
impl From<STATE_A> for bool {
    #[inline(always)]
    fn from(variant: STATE_A) -> Self {
        variant as u8 != 0
    }
}
impl STATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STATE_A {
        match self.bits {
            false => STATE_A::REVOKED,
            true => STATE_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `REVOKED`"]
    #[inline(always)]
    pub fn is_revoked(&self) -> bool {
        *self == STATE_A::REVOKED
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == STATE_A::ACTIVE
    }
}
#[doc = "Field `STATE` writer - Revocation state for the key slot"]
pub type STATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERM_SPEC, STATE_A, O>;
impl<'a, const O: u8> STATE_W<'a, O> {
    #[doc = "Key value registers can no longer be read or pushed"]
    #[inline(always)]
    pub fn revoked(self) -> &'a mut W {
        self.variant(STATE_A::REVOKED)
    }
    #[doc = "Key value registers are readable (if enabled) and can be pushed (if enabled)"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(STATE_A::ACTIVE)
    }
}
impl R {
    #[doc = "Bit 0 - Write permission for key slot"]
    #[inline(always)]
    pub fn write(&self) -> WRITE_R {
        WRITE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Read permission for key slot"]
    #[inline(always)]
    pub fn read(&self) -> READ_R {
        READ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Push permission for key slot"]
    #[inline(always)]
    pub fn push(&self) -> PUSH_R {
        PUSH_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 16 - Revocation state for the key slot"]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write permission for key slot"]
    #[inline(always)]
    pub fn write(&mut self) -> WRITE_W<0> {
        WRITE_W::new(self)
    }
    #[doc = "Bit 1 - Read permission for key slot"]
    #[inline(always)]
    pub fn read(&mut self) -> READ_W<1> {
        READ_W::new(self)
    }
    #[doc = "Bit 2 - Push permission for key slot"]
    #[inline(always)]
    pub fn push(&mut self) -> PUSH_W<2> {
        PUSH_W::new(self)
    }
    #[doc = "Bit 16 - Revocation state for the key slot"]
    #[inline(always)]
    pub fn state(&mut self) -> STATE_W<16> {
        STATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description cluster: Define permissions for the key slot. Bits 0-15 and 16-31 can only be written when equal to 0xFFFF.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perm](index.html) module"]
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
