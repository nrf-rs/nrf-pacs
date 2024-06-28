#[doc = "Register `INTPEND` reader"]
pub type R = crate::R<IntpendSpec>;
#[doc = "Read pending status of interrupt for event KEYSLOT_PUSHED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeyslotPushed {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<KeyslotPushed> for bool {
    #[inline(always)]
    fn from(variant: KeyslotPushed) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KEYSLOT_PUSHED` reader - Read pending status of interrupt for event KEYSLOT_PUSHED"]
pub type KeyslotPushedR = crate::BitReader<KeyslotPushed>;
impl KeyslotPushedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> KeyslotPushed {
        match self.bits {
            false => KeyslotPushed::NotPending,
            true => KeyslotPushed::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == KeyslotPushed::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == KeyslotPushed::Pending
    }
}
#[doc = "Read pending status of interrupt for event KEYSLOT_REVOKED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeyslotRevoked {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<KeyslotRevoked> for bool {
    #[inline(always)]
    fn from(variant: KeyslotRevoked) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KEYSLOT_REVOKED` reader - Read pending status of interrupt for event KEYSLOT_REVOKED"]
pub type KeyslotRevokedR = crate::BitReader<KeyslotRevoked>;
impl KeyslotRevokedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> KeyslotRevoked {
        match self.bits {
            false => KeyslotRevoked::NotPending,
            true => KeyslotRevoked::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == KeyslotRevoked::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == KeyslotRevoked::Pending
    }
}
#[doc = "Read pending status of interrupt for event KEYSLOT_ERROR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeyslotError {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<KeyslotError> for bool {
    #[inline(always)]
    fn from(variant: KeyslotError) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KEYSLOT_ERROR` reader - Read pending status of interrupt for event KEYSLOT_ERROR"]
pub type KeyslotErrorR = crate::BitReader<KeyslotError>;
impl KeyslotErrorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> KeyslotError {
        match self.bits {
            false => KeyslotError::NotPending,
            true => KeyslotError::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == KeyslotError::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == KeyslotError::Pending
    }
}
impl R {
    #[doc = "Bit 0 - Read pending status of interrupt for event KEYSLOT_PUSHED"]
    #[inline(always)]
    pub fn keyslot_pushed(&self) -> KeyslotPushedR {
        KeyslotPushedR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Read pending status of interrupt for event KEYSLOT_REVOKED"]
    #[inline(always)]
    pub fn keyslot_revoked(&self) -> KeyslotRevokedR {
        KeyslotRevokedR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Read pending status of interrupt for event KEYSLOT_ERROR"]
    #[inline(always)]
    pub fn keyslot_error(&self) -> KeyslotErrorR {
        KeyslotErrorR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Pending interrupts\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intpend::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntpendSpec;
impl crate::RegisterSpec for IntpendSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intpend::R`](R) reader structure"]
impl crate::Readable for IntpendSpec {}
#[doc = "`reset()` method sets INTPEND to value 0"]
impl crate::Resettable for IntpendSpec {
    const RESET_VALUE: u32 = 0;
}
