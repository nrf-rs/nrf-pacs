#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<IntenclrSpec>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
#[doc = "Write '1' to disable interrupt for event KEYSLOT_PUSHED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeyslotPushed {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<KeyslotPushed> for bool {
    #[inline(always)]
    fn from(variant: KeyslotPushed) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KEYSLOT_PUSHED` reader - Write '1' to disable interrupt for event KEYSLOT_PUSHED"]
pub type KeyslotPushedR = crate::BitReader<KeyslotPushed>;
impl KeyslotPushedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> KeyslotPushed {
        match self.bits {
            false => KeyslotPushed::Disabled,
            true => KeyslotPushed::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == KeyslotPushed::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == KeyslotPushed::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event KEYSLOT_PUSHED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeyslotPushedWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<KeyslotPushedWO> for bool {
    #[inline(always)]
    fn from(variant: KeyslotPushedWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KEYSLOT_PUSHED` writer - Write '1' to disable interrupt for event KEYSLOT_PUSHED"]
pub type KeyslotPushedW<'a, REG> = crate::BitWriter<'a, REG, KeyslotPushedWO>;
impl<'a, REG> KeyslotPushedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(KeyslotPushedWO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event KEYSLOT_REVOKED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeyslotRevoked {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<KeyslotRevoked> for bool {
    #[inline(always)]
    fn from(variant: KeyslotRevoked) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KEYSLOT_REVOKED` reader - Write '1' to disable interrupt for event KEYSLOT_REVOKED"]
pub type KeyslotRevokedR = crate::BitReader<KeyslotRevoked>;
impl KeyslotRevokedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> KeyslotRevoked {
        match self.bits {
            false => KeyslotRevoked::Disabled,
            true => KeyslotRevoked::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == KeyslotRevoked::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == KeyslotRevoked::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event KEYSLOT_REVOKED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeyslotRevokedWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<KeyslotRevokedWO> for bool {
    #[inline(always)]
    fn from(variant: KeyslotRevokedWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KEYSLOT_REVOKED` writer - Write '1' to disable interrupt for event KEYSLOT_REVOKED"]
pub type KeyslotRevokedW<'a, REG> = crate::BitWriter<'a, REG, KeyslotRevokedWO>;
impl<'a, REG> KeyslotRevokedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(KeyslotRevokedWO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event KEYSLOT_ERROR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeyslotError {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<KeyslotError> for bool {
    #[inline(always)]
    fn from(variant: KeyslotError) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KEYSLOT_ERROR` reader - Write '1' to disable interrupt for event KEYSLOT_ERROR"]
pub type KeyslotErrorR = crate::BitReader<KeyslotError>;
impl KeyslotErrorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> KeyslotError {
        match self.bits {
            false => KeyslotError::Disabled,
            true => KeyslotError::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == KeyslotError::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == KeyslotError::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event KEYSLOT_ERROR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeyslotErrorWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<KeyslotErrorWO> for bool {
    #[inline(always)]
    fn from(variant: KeyslotErrorWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KEYSLOT_ERROR` writer - Write '1' to disable interrupt for event KEYSLOT_ERROR"]
pub type KeyslotErrorW<'a, REG> = crate::BitWriter<'a, REG, KeyslotErrorWO>;
impl<'a, REG> KeyslotErrorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(KeyslotErrorWO::Clear)
    }
}
impl R {
    #[doc = "Bit 0 - Write '1' to disable interrupt for event KEYSLOT_PUSHED"]
    #[inline(always)]
    pub fn keyslot_pushed(&self) -> KeyslotPushedR {
        KeyslotPushedR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write '1' to disable interrupt for event KEYSLOT_REVOKED"]
    #[inline(always)]
    pub fn keyslot_revoked(&self) -> KeyslotRevokedR {
        KeyslotRevokedR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write '1' to disable interrupt for event KEYSLOT_ERROR"]
    #[inline(always)]
    pub fn keyslot_error(&self) -> KeyslotErrorR {
        KeyslotErrorR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to disable interrupt for event KEYSLOT_PUSHED"]
    #[inline(always)]
    #[must_use]
    pub fn keyslot_pushed(&mut self) -> KeyslotPushedW<IntenclrSpec> {
        KeyslotPushedW::new(self, 0)
    }
    #[doc = "Bit 1 - Write '1' to disable interrupt for event KEYSLOT_REVOKED"]
    #[inline(always)]
    #[must_use]
    pub fn keyslot_revoked(&mut self) -> KeyslotRevokedW<IntenclrSpec> {
        KeyslotRevokedW::new(self, 1)
    }
    #[doc = "Bit 2 - Write '1' to disable interrupt for event KEYSLOT_ERROR"]
    #[inline(always)]
    #[must_use]
    pub fn keyslot_error(&mut self) -> KeyslotErrorW<IntenclrSpec> {
        KeyslotErrorW::new(self, 2)
    }
}
#[doc = "Disable interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intenclr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intenclr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenclrSpec;
impl crate::RegisterSpec for IntenclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenclr::R`](R) reader structure"]
impl crate::Readable for IntenclrSpec {}
#[doc = "`write(|w| ..)` method takes [`intenclr::W`](W) writer structure"]
impl crate::Writable for IntenclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for IntenclrSpec {
    const RESET_VALUE: u32 = 0;
}
