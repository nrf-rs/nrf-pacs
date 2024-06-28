#[doc = "Register `PERM` reader"]
pub type R = crate::R<PermSpec>;
#[doc = "Register `PERM` writer"]
pub type W = crate::W<PermSpec>;
#[doc = "Write permission for key slot\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Write {
    #[doc = "0: Disable write to the key value registers"]
    Disabled = 0,
    #[doc = "1: Enable write to the key value registers"]
    Enabled = 1,
}
impl From<Write> for bool {
    #[inline(always)]
    fn from(variant: Write) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRITE` reader - Write permission for key slot"]
pub type WriteR = crate::BitReader<Write>;
impl WriteR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Write {
        match self.bits {
            false => Write::Disabled,
            true => Write::Enabled,
        }
    }
    #[doc = "Disable write to the key value registers"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Write::Disabled
    }
    #[doc = "Enable write to the key value registers"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Write::Enabled
    }
}
#[doc = "Field `WRITE` writer - Write permission for key slot"]
pub type WriteW<'a, REG> = crate::BitWriter<'a, REG, Write>;
impl<'a, REG> WriteW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable write to the key value registers"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Write::Disabled)
    }
    #[doc = "Enable write to the key value registers"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Write::Enabled)
    }
}
#[doc = "Read permission for key slot\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Read {
    #[doc = "0: Disable read from key value registers"]
    Disabled = 0,
    #[doc = "1: Enable read from key value registers"]
    Enabled = 1,
}
impl From<Read> for bool {
    #[inline(always)]
    fn from(variant: Read) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READ` reader - Read permission for key slot"]
pub type ReadR = crate::BitReader<Read>;
impl ReadR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Read {
        match self.bits {
            false => Read::Disabled,
            true => Read::Enabled,
        }
    }
    #[doc = "Disable read from key value registers"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Read::Disabled
    }
    #[doc = "Enable read from key value registers"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Read::Enabled
    }
}
#[doc = "Field `READ` writer - Read permission for key slot"]
pub type ReadW<'a, REG> = crate::BitWriter<'a, REG, Read>;
impl<'a, REG> ReadW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable read from key value registers"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Read::Disabled)
    }
    #[doc = "Enable read from key value registers"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Read::Enabled)
    }
}
#[doc = "Push permission for key slot\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Push {
    #[doc = "0: Disable pushing of key value registers over secure APB, but can be read if field READ is Enabled"]
    Disabled = 0,
    #[doc = "1: Enable pushing of key value registers over secure APB. Register KEYSLOT.CONFIGn.DEST must contain a valid destination address!"]
    Enabled = 1,
}
impl From<Push> for bool {
    #[inline(always)]
    fn from(variant: Push) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PUSH` reader - Push permission for key slot"]
pub type PushR = crate::BitReader<Push>;
impl PushR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Push {
        match self.bits {
            false => Push::Disabled,
            true => Push::Enabled,
        }
    }
    #[doc = "Disable pushing of key value registers over secure APB, but can be read if field READ is Enabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Push::Disabled
    }
    #[doc = "Enable pushing of key value registers over secure APB. Register KEYSLOT.CONFIGn.DEST must contain a valid destination address!"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Push::Enabled
    }
}
#[doc = "Field `PUSH` writer - Push permission for key slot"]
pub type PushW<'a, REG> = crate::BitWriter<'a, REG, Push>;
impl<'a, REG> PushW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable pushing of key value registers over secure APB, but can be read if field READ is Enabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Push::Disabled)
    }
    #[doc = "Enable pushing of key value registers over secure APB. Register KEYSLOT.CONFIGn.DEST must contain a valid destination address!"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Push::Enabled)
    }
}
#[doc = "Revocation state for the key slot\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    #[doc = "0: Key value registers can no longer be read or pushed"]
    Revoked = 0,
    #[doc = "1: Key value registers are readable (if enabled) and can be pushed (if enabled)"]
    Active = 1,
}
impl From<State> for bool {
    #[inline(always)]
    fn from(variant: State) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STATE` reader - Revocation state for the key slot"]
pub type StateR = crate::BitReader<State>;
impl StateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> State {
        match self.bits {
            false => State::Revoked,
            true => State::Active,
        }
    }
    #[doc = "Key value registers can no longer be read or pushed"]
    #[inline(always)]
    pub fn is_revoked(&self) -> bool {
        *self == State::Revoked
    }
    #[doc = "Key value registers are readable (if enabled) and can be pushed (if enabled)"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == State::Active
    }
}
#[doc = "Field `STATE` writer - Revocation state for the key slot"]
pub type StateW<'a, REG> = crate::BitWriter<'a, REG, State>;
impl<'a, REG> StateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Key value registers can no longer be read or pushed"]
    #[inline(always)]
    pub fn revoked(self) -> &'a mut crate::W<REG> {
        self.variant(State::Revoked)
    }
    #[doc = "Key value registers are readable (if enabled) and can be pushed (if enabled)"]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(State::Active)
    }
}
impl R {
    #[doc = "Bit 0 - Write permission for key slot"]
    #[inline(always)]
    pub fn write(&self) -> WriteR {
        WriteR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Read permission for key slot"]
    #[inline(always)]
    pub fn read(&self) -> ReadR {
        ReadR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Push permission for key slot"]
    #[inline(always)]
    pub fn push(&self) -> PushR {
        PushR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 16 - Revocation state for the key slot"]
    #[inline(always)]
    pub fn state(&self) -> StateR {
        StateR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write permission for key slot"]
    #[inline(always)]
    #[must_use]
    pub fn write(&mut self) -> WriteW<PermSpec> {
        WriteW::new(self, 0)
    }
    #[doc = "Bit 1 - Read permission for key slot"]
    #[inline(always)]
    #[must_use]
    pub fn read(&mut self) -> ReadW<PermSpec> {
        ReadW::new(self, 1)
    }
    #[doc = "Bit 2 - Push permission for key slot"]
    #[inline(always)]
    #[must_use]
    pub fn push(&mut self) -> PushW<PermSpec> {
        PushW::new(self, 2)
    }
    #[doc = "Bit 16 - Revocation state for the key slot"]
    #[inline(always)]
    #[must_use]
    pub fn state(&mut self) -> StateW<PermSpec> {
        StateW::new(self, 16)
    }
}
#[doc = "Description cluster: Define permissions for the key slot. Bits 0-15 and 16-31 can only be written when equal to 0xFFFF.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`perm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PermSpec;
impl crate::RegisterSpec for PermSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`perm::R`](R) reader structure"]
impl crate::Readable for PermSpec {}
#[doc = "`write(|w| ..)` method takes [`perm::W`](W) writer structure"]
impl crate::Writable for PermSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PERM to value 0xffff_ffff"]
impl crate::Resettable for PermSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
