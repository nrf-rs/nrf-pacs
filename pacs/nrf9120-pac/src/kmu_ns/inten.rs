#[doc = "Register `INTEN` reader"]
pub type R = crate::R<IntenSpec>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<IntenSpec>;
#[doc = "Enable or disable interrupt for event KEYSLOT_PUSHED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeyslotPushed {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<KeyslotPushed> for bool {
    #[inline(always)]
    fn from(variant: KeyslotPushed) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KEYSLOT_PUSHED` reader - Enable or disable interrupt for event KEYSLOT_PUSHED"]
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
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == KeyslotPushed::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == KeyslotPushed::Enabled
    }
}
#[doc = "Field `KEYSLOT_PUSHED` writer - Enable or disable interrupt for event KEYSLOT_PUSHED"]
pub type KeyslotPushedW<'a, REG> = crate::BitWriter<'a, REG, KeyslotPushed>;
impl<'a, REG> KeyslotPushedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(KeyslotPushed::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(KeyslotPushed::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event KEYSLOT_REVOKED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeyslotRevoked {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<KeyslotRevoked> for bool {
    #[inline(always)]
    fn from(variant: KeyslotRevoked) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KEYSLOT_REVOKED` reader - Enable or disable interrupt for event KEYSLOT_REVOKED"]
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
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == KeyslotRevoked::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == KeyslotRevoked::Enabled
    }
}
#[doc = "Field `KEYSLOT_REVOKED` writer - Enable or disable interrupt for event KEYSLOT_REVOKED"]
pub type KeyslotRevokedW<'a, REG> = crate::BitWriter<'a, REG, KeyslotRevoked>;
impl<'a, REG> KeyslotRevokedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(KeyslotRevoked::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(KeyslotRevoked::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event KEYSLOT_ERROR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeyslotError {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<KeyslotError> for bool {
    #[inline(always)]
    fn from(variant: KeyslotError) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KEYSLOT_ERROR` reader - Enable or disable interrupt for event KEYSLOT_ERROR"]
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
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == KeyslotError::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == KeyslotError::Enabled
    }
}
#[doc = "Field `KEYSLOT_ERROR` writer - Enable or disable interrupt for event KEYSLOT_ERROR"]
pub type KeyslotErrorW<'a, REG> = crate::BitWriter<'a, REG, KeyslotError>;
impl<'a, REG> KeyslotErrorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(KeyslotError::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(KeyslotError::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Enable or disable interrupt for event KEYSLOT_PUSHED"]
    #[inline(always)]
    pub fn keyslot_pushed(&self) -> KeyslotPushedR {
        KeyslotPushedR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable or disable interrupt for event KEYSLOT_REVOKED"]
    #[inline(always)]
    pub fn keyslot_revoked(&self) -> KeyslotRevokedR {
        KeyslotRevokedR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable or disable interrupt for event KEYSLOT_ERROR"]
    #[inline(always)]
    pub fn keyslot_error(&self) -> KeyslotErrorR {
        KeyslotErrorR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable interrupt for event KEYSLOT_PUSHED"]
    #[inline(always)]
    #[must_use]
    pub fn keyslot_pushed(&mut self) -> KeyslotPushedW<IntenSpec> {
        KeyslotPushedW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable or disable interrupt for event KEYSLOT_REVOKED"]
    #[inline(always)]
    #[must_use]
    pub fn keyslot_revoked(&mut self) -> KeyslotRevokedW<IntenSpec> {
        KeyslotRevokedW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable or disable interrupt for event KEYSLOT_ERROR"]
    #[inline(always)]
    #[must_use]
    pub fn keyslot_error(&mut self) -> KeyslotErrorW<IntenSpec> {
        KeyslotErrorW::new(self, 2)
    }
}
#[doc = "Enable or disable interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenSpec;
impl crate::RegisterSpec for IntenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for IntenSpec {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for IntenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for IntenSpec {
    const RESET_VALUE: u32 = 0;
}
