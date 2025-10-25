#[doc = "Register `TRCVICTLR` reader"]
pub type R = crate::R<TrcvictlrSpec>;
#[doc = "Register `TRCVICTLR` writer"]
pub type W = crate::W<TrcvictlrSpec>;
#[doc = "Select which resource number should be filtered.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EventSel {
    #[doc = "0: This event is not filtered."]
    Disabled = 0,
    #[doc = "1: This event is filtered."]
    Enabled = 1,
}
impl From<EventSel> for u8 {
    #[inline(always)]
    fn from(variant: EventSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EventSel {
    type Ux = u8;
}
impl crate::IsEnum for EventSel {}
#[doc = "Field `EVENT_SEL` reader - Select which resource number should be filtered."]
pub type EventSelR = crate::FieldReader<EventSel>;
impl EventSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EventSel> {
        match self.bits {
            0 => Some(EventSel::Disabled),
            1 => Some(EventSel::Enabled),
            _ => None,
        }
    }
    #[doc = "This event is not filtered."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EventSel::Disabled
    }
    #[doc = "This event is filtered."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EventSel::Enabled
    }
}
#[doc = "Field `EVENT_SEL` writer - Select which resource number should be filtered."]
pub type EventSelW<'a, REG> = crate::FieldWriter<'a, REG, 5, EventSel>;
impl<'a, REG> EventSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "This event is not filtered."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EventSel::Disabled)
    }
    #[doc = "This event is filtered."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EventSel::Enabled)
    }
}
#[doc = "When TRCIDR4.NUMACPAIRS &gt; 0 or TRCIDR4.NUMPC &gt; 0, this bit returns the status of the start/stop logic.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ssstatus {
    #[doc = "0: The start/stop logic is in the stopped state."]
    Stopped = 0,
    #[doc = "1: The start/stop logic is in the started state."]
    Started = 1,
}
impl From<Ssstatus> for bool {
    #[inline(always)]
    fn from(variant: Ssstatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSSTATUS` reader - When TRCIDR4.NUMACPAIRS &gt; 0 or TRCIDR4.NUMPC &gt; 0, this bit returns the status of the start/stop logic."]
pub type SsstatusR = crate::BitReader<Ssstatus>;
impl SsstatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ssstatus {
        match self.bits {
            false => Ssstatus::Stopped,
            true => Ssstatus::Started,
        }
    }
    #[doc = "The start/stop logic is in the stopped state."]
    #[inline(always)]
    pub fn is_stopped(&self) -> bool {
        *self == Ssstatus::Stopped
    }
    #[doc = "The start/stop logic is in the started state."]
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == Ssstatus::Started
    }
}
#[doc = "Field `SSSTATUS` writer - When TRCIDR4.NUMACPAIRS &gt; 0 or TRCIDR4.NUMPC &gt; 0, this bit returns the status of the start/stop logic."]
pub type SsstatusW<'a, REG> = crate::BitWriter<'a, REG, Ssstatus>;
impl<'a, REG> SsstatusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The start/stop logic is in the stopped state."]
    #[inline(always)]
    pub fn stopped(self) -> &'a mut crate::W<REG> {
        self.variant(Ssstatus::Stopped)
    }
    #[doc = "The start/stop logic is in the started state."]
    #[inline(always)]
    pub fn started(self) -> &'a mut crate::W<REG> {
        self.variant(Ssstatus::Started)
    }
}
#[doc = "Controls whether a trace unit must trace a Reset exception.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trcreset {
    #[doc = "0: The trace unit does not trace a Reset exception unless it traces the exception or instruction immediately prior to the Reset exception."]
    Disabled = 0,
    #[doc = "1: The trace unit always traces a Reset exception."]
    Enabled = 1,
}
impl From<Trcreset> for bool {
    #[inline(always)]
    fn from(variant: Trcreset) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRCRESET` reader - Controls whether a trace unit must trace a Reset exception."]
pub type TrcresetR = crate::BitReader<Trcreset>;
impl TrcresetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trcreset {
        match self.bits {
            false => Trcreset::Disabled,
            true => Trcreset::Enabled,
        }
    }
    #[doc = "The trace unit does not trace a Reset exception unless it traces the exception or instruction immediately prior to the Reset exception."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Trcreset::Disabled
    }
    #[doc = "The trace unit always traces a Reset exception."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Trcreset::Enabled
    }
}
#[doc = "Field `TRCRESET` writer - Controls whether a trace unit must trace a Reset exception."]
pub type TrcresetW<'a, REG> = crate::BitWriter<'a, REG, Trcreset>;
impl<'a, REG> TrcresetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The trace unit does not trace a Reset exception unless it traces the exception or instruction immediately prior to the Reset exception."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Trcreset::Disabled)
    }
    #[doc = "The trace unit always traces a Reset exception."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Trcreset::Enabled)
    }
}
#[doc = "When TRCIDR3.TRCERR==1, this bit controls whether a trace unit must trace a System error exception.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trcerr {
    #[doc = "0: The trace unit does not trace a System error exception unless it traces the exception or instruction immediately prior to the System error exception."]
    Disabled = 0,
    #[doc = "1: The trace unit always traces a System error exception, regardless of the value of ViewInst."]
    Enabled = 1,
}
impl From<Trcerr> for bool {
    #[inline(always)]
    fn from(variant: Trcerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRCERR` reader - When TRCIDR3.TRCERR==1, this bit controls whether a trace unit must trace a System error exception."]
pub type TrcerrR = crate::BitReader<Trcerr>;
impl TrcerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trcerr {
        match self.bits {
            false => Trcerr::Disabled,
            true => Trcerr::Enabled,
        }
    }
    #[doc = "The trace unit does not trace a System error exception unless it traces the exception or instruction immediately prior to the System error exception."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Trcerr::Disabled
    }
    #[doc = "The trace unit always traces a System error exception, regardless of the value of ViewInst."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Trcerr::Enabled
    }
}
#[doc = "Field `TRCERR` writer - When TRCIDR3.TRCERR==1, this bit controls whether a trace unit must trace a System error exception."]
pub type TrcerrW<'a, REG> = crate::BitWriter<'a, REG, Trcerr>;
impl<'a, REG> TrcerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The trace unit does not trace a System error exception unless it traces the exception or instruction immediately prior to the System error exception."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Trcerr::Disabled)
    }
    #[doc = "The trace unit always traces a System error exception, regardless of the value of ViewInst."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Trcerr::Enabled)
    }
}
#[doc = "In Secure state, each bit controls whether instruction tracing is enabled for the corresponding Exception level 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Exlevel0S {
    #[doc = "1: The trace unit does not generate instruction trace, in Secure state, for Exception level 0."]
    Disabled = 1,
    #[doc = "0: The trace unit generates instruction trace, in Secure state, for Exception level 0."]
    Enabled = 0,
}
impl From<Exlevel0S> for bool {
    #[inline(always)]
    fn from(variant: Exlevel0S) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXLEVEL0_S` reader - In Secure state, each bit controls whether instruction tracing is enabled for the corresponding Exception level 0."]
pub type Exlevel0SR = crate::BitReader<Exlevel0S>;
impl Exlevel0SR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Exlevel0S {
        match self.bits {
            true => Exlevel0S::Disabled,
            false => Exlevel0S::Enabled,
        }
    }
    #[doc = "The trace unit does not generate instruction trace, in Secure state, for Exception level 0."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Exlevel0S::Disabled
    }
    #[doc = "The trace unit generates instruction trace, in Secure state, for Exception level 0."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Exlevel0S::Enabled
    }
}
#[doc = "Field `EXLEVEL0_S` writer - In Secure state, each bit controls whether instruction tracing is enabled for the corresponding Exception level 0."]
pub type Exlevel0SW<'a, REG> = crate::BitWriter<'a, REG, Exlevel0S>;
impl<'a, REG> Exlevel0SW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The trace unit does not generate instruction trace, in Secure state, for Exception level 0."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Exlevel0S::Disabled)
    }
    #[doc = "The trace unit generates instruction trace, in Secure state, for Exception level 0."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Exlevel0S::Enabled)
    }
}
#[doc = "In Secure state, each bit controls whether instruction tracing is enabled for the corresponding Exception level 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Exlevel1S {
    #[doc = "1: The trace unit does not generate instruction trace, in Secure state, for Exception level 1."]
    Disabled = 1,
    #[doc = "0: The trace unit generates instruction trace, in Secure state, for Exception level 1."]
    Enabled = 0,
}
impl From<Exlevel1S> for bool {
    #[inline(always)]
    fn from(variant: Exlevel1S) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXLEVEL1_S` reader - In Secure state, each bit controls whether instruction tracing is enabled for the corresponding Exception level 1."]
pub type Exlevel1SR = crate::BitReader<Exlevel1S>;
impl Exlevel1SR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Exlevel1S {
        match self.bits {
            true => Exlevel1S::Disabled,
            false => Exlevel1S::Enabled,
        }
    }
    #[doc = "The trace unit does not generate instruction trace, in Secure state, for Exception level 1."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Exlevel1S::Disabled
    }
    #[doc = "The trace unit generates instruction trace, in Secure state, for Exception level 1."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Exlevel1S::Enabled
    }
}
#[doc = "Field `EXLEVEL1_S` writer - In Secure state, each bit controls whether instruction tracing is enabled for the corresponding Exception level 1."]
pub type Exlevel1SW<'a, REG> = crate::BitWriter<'a, REG, Exlevel1S>;
impl<'a, REG> Exlevel1SW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The trace unit does not generate instruction trace, in Secure state, for Exception level 1."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Exlevel1S::Disabled)
    }
    #[doc = "The trace unit generates instruction trace, in Secure state, for Exception level 1."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Exlevel1S::Enabled)
    }
}
#[doc = "In Secure state, each bit controls whether instruction tracing is enabled for the corresponding Exception level 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Exlevel2S {
    #[doc = "1: The trace unit does not generate instruction trace, in Secure state, for Exception level 2."]
    Disabled = 1,
    #[doc = "0: The trace unit generates instruction trace, in Secure state, for Exception level 2."]
    Enabled = 0,
}
impl From<Exlevel2S> for bool {
    #[inline(always)]
    fn from(variant: Exlevel2S) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXLEVEL2_S` reader - In Secure state, each bit controls whether instruction tracing is enabled for the corresponding Exception level 2."]
pub type Exlevel2SR = crate::BitReader<Exlevel2S>;
impl Exlevel2SR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Exlevel2S {
        match self.bits {
            true => Exlevel2S::Disabled,
            false => Exlevel2S::Enabled,
        }
    }
    #[doc = "The trace unit does not generate instruction trace, in Secure state, for Exception level 2."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Exlevel2S::Disabled
    }
    #[doc = "The trace unit generates instruction trace, in Secure state, for Exception level 2."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Exlevel2S::Enabled
    }
}
#[doc = "Field `EXLEVEL2_S` writer - In Secure state, each bit controls whether instruction tracing is enabled for the corresponding Exception level 2."]
pub type Exlevel2SW<'a, REG> = crate::BitWriter<'a, REG, Exlevel2S>;
impl<'a, REG> Exlevel2SW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The trace unit does not generate instruction trace, in Secure state, for Exception level 2."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Exlevel2S::Disabled)
    }
    #[doc = "The trace unit generates instruction trace, in Secure state, for Exception level 2."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Exlevel2S::Enabled)
    }
}
#[doc = "In Secure state, each bit controls whether instruction tracing is enabled for the corresponding Exception level 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Exlevel3S {
    #[doc = "1: The trace unit does not generate instruction trace, in Secure state, for Exception level 3."]
    Disabled = 1,
    #[doc = "0: The trace unit generates instruction trace, in Secure state, for Exception level 3."]
    Enabled = 0,
}
impl From<Exlevel3S> for bool {
    #[inline(always)]
    fn from(variant: Exlevel3S) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXLEVEL3_S` reader - In Secure state, each bit controls whether instruction tracing is enabled for the corresponding Exception level 3."]
pub type Exlevel3SR = crate::BitReader<Exlevel3S>;
impl Exlevel3SR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Exlevel3S {
        match self.bits {
            true => Exlevel3S::Disabled,
            false => Exlevel3S::Enabled,
        }
    }
    #[doc = "The trace unit does not generate instruction trace, in Secure state, for Exception level 3."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Exlevel3S::Disabled
    }
    #[doc = "The trace unit generates instruction trace, in Secure state, for Exception level 3."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Exlevel3S::Enabled
    }
}
#[doc = "Field `EXLEVEL3_S` writer - In Secure state, each bit controls whether instruction tracing is enabled for the corresponding Exception level 3."]
pub type Exlevel3SW<'a, REG> = crate::BitWriter<'a, REG, Exlevel3S>;
impl<'a, REG> Exlevel3SW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The trace unit does not generate instruction trace, in Secure state, for Exception level 3."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Exlevel3S::Disabled)
    }
    #[doc = "The trace unit generates instruction trace, in Secure state, for Exception level 3."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Exlevel3S::Enabled)
    }
}
#[doc = "In Non-secure state, each bit controls whether instruction tracing is enabled for the corresponding Exception level 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Exlevel0Ns {
    #[doc = "1: The trace unit does not generate instruction trace, in Non-secure state, for Exception level 0."]
    Disabled = 1,
    #[doc = "0: The trace unit generates instruction trace, in Non-secure state, for Exception level 0."]
    Enabled = 0,
}
impl From<Exlevel0Ns> for bool {
    #[inline(always)]
    fn from(variant: Exlevel0Ns) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXLEVEL0_NS` reader - In Non-secure state, each bit controls whether instruction tracing is enabled for the corresponding Exception level 0."]
pub type Exlevel0NsR = crate::BitReader<Exlevel0Ns>;
impl Exlevel0NsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Exlevel0Ns {
        match self.bits {
            true => Exlevel0Ns::Disabled,
            false => Exlevel0Ns::Enabled,
        }
    }
    #[doc = "The trace unit does not generate instruction trace, in Non-secure state, for Exception level 0."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Exlevel0Ns::Disabled
    }
    #[doc = "The trace unit generates instruction trace, in Non-secure state, for Exception level 0."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Exlevel0Ns::Enabled
    }
}
#[doc = "Field `EXLEVEL0_NS` writer - In Non-secure state, each bit controls whether instruction tracing is enabled for the corresponding Exception level 0."]
pub type Exlevel0NsW<'a, REG> = crate::BitWriter<'a, REG, Exlevel0Ns>;
impl<'a, REG> Exlevel0NsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The trace unit does not generate instruction trace, in Non-secure state, for Exception level 0."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Exlevel0Ns::Disabled)
    }
    #[doc = "The trace unit generates instruction trace, in Non-secure state, for Exception level 0."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Exlevel0Ns::Enabled)
    }
}
#[doc = "In Non-secure state, each bit controls whether instruction tracing is enabled for the corresponding Exception level 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Exlevel1Ns {
    #[doc = "1: The trace unit does not generate instruction trace, in Non-secure state, for Exception level 1."]
    Disabled = 1,
    #[doc = "0: The trace unit generates instruction trace, in Non-secure state, for Exception level 1."]
    Enabled = 0,
}
impl From<Exlevel1Ns> for bool {
    #[inline(always)]
    fn from(variant: Exlevel1Ns) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXLEVEL1_NS` reader - In Non-secure state, each bit controls whether instruction tracing is enabled for the corresponding Exception level 1."]
pub type Exlevel1NsR = crate::BitReader<Exlevel1Ns>;
impl Exlevel1NsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Exlevel1Ns {
        match self.bits {
            true => Exlevel1Ns::Disabled,
            false => Exlevel1Ns::Enabled,
        }
    }
    #[doc = "The trace unit does not generate instruction trace, in Non-secure state, for Exception level 1."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Exlevel1Ns::Disabled
    }
    #[doc = "The trace unit generates instruction trace, in Non-secure state, for Exception level 1."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Exlevel1Ns::Enabled
    }
}
#[doc = "Field `EXLEVEL1_NS` writer - In Non-secure state, each bit controls whether instruction tracing is enabled for the corresponding Exception level 1."]
pub type Exlevel1NsW<'a, REG> = crate::BitWriter<'a, REG, Exlevel1Ns>;
impl<'a, REG> Exlevel1NsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The trace unit does not generate instruction trace, in Non-secure state, for Exception level 1."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Exlevel1Ns::Disabled)
    }
    #[doc = "The trace unit generates instruction trace, in Non-secure state, for Exception level 1."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Exlevel1Ns::Enabled)
    }
}
#[doc = "In Non-secure state, each bit controls whether instruction tracing is enabled for the corresponding Exception level 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Exlevel2Ns {
    #[doc = "1: The trace unit does not generate instruction trace, in Non-secure state, for Exception level 2."]
    Disabled = 1,
    #[doc = "0: The trace unit generates instruction trace, in Non-secure state, for Exception level 2."]
    Enabled = 0,
}
impl From<Exlevel2Ns> for bool {
    #[inline(always)]
    fn from(variant: Exlevel2Ns) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXLEVEL2_NS` reader - In Non-secure state, each bit controls whether instruction tracing is enabled for the corresponding Exception level 2."]
pub type Exlevel2NsR = crate::BitReader<Exlevel2Ns>;
impl Exlevel2NsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Exlevel2Ns {
        match self.bits {
            true => Exlevel2Ns::Disabled,
            false => Exlevel2Ns::Enabled,
        }
    }
    #[doc = "The trace unit does not generate instruction trace, in Non-secure state, for Exception level 2."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Exlevel2Ns::Disabled
    }
    #[doc = "The trace unit generates instruction trace, in Non-secure state, for Exception level 2."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Exlevel2Ns::Enabled
    }
}
#[doc = "Field `EXLEVEL2_NS` writer - In Non-secure state, each bit controls whether instruction tracing is enabled for the corresponding Exception level 2."]
pub type Exlevel2NsW<'a, REG> = crate::BitWriter<'a, REG, Exlevel2Ns>;
impl<'a, REG> Exlevel2NsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The trace unit does not generate instruction trace, in Non-secure state, for Exception level 2."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Exlevel2Ns::Disabled)
    }
    #[doc = "The trace unit generates instruction trace, in Non-secure state, for Exception level 2."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Exlevel2Ns::Enabled)
    }
}
#[doc = "In Non-secure state, each bit controls whether instruction tracing is enabled for the corresponding Exception level 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Exlevel3Ns {
    #[doc = "1: The trace unit does not generate instruction trace, in Non-secure state, for Exception level 3."]
    Disabled = 1,
    #[doc = "0: The trace unit generates instruction trace, in Non-secure state, for Exception level 3."]
    Enabled = 0,
}
impl From<Exlevel3Ns> for bool {
    #[inline(always)]
    fn from(variant: Exlevel3Ns) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXLEVEL3_NS` reader - In Non-secure state, each bit controls whether instruction tracing is enabled for the corresponding Exception level 3."]
pub type Exlevel3NsR = crate::BitReader<Exlevel3Ns>;
impl Exlevel3NsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Exlevel3Ns {
        match self.bits {
            true => Exlevel3Ns::Disabled,
            false => Exlevel3Ns::Enabled,
        }
    }
    #[doc = "The trace unit does not generate instruction trace, in Non-secure state, for Exception level 3."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Exlevel3Ns::Disabled
    }
    #[doc = "The trace unit generates instruction trace, in Non-secure state, for Exception level 3."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Exlevel3Ns::Enabled
    }
}
#[doc = "Field `EXLEVEL3_NS` writer - In Non-secure state, each bit controls whether instruction tracing is enabled for the corresponding Exception level 3."]
pub type Exlevel3NsW<'a, REG> = crate::BitWriter<'a, REG, Exlevel3Ns>;
impl<'a, REG> Exlevel3NsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The trace unit does not generate instruction trace, in Non-secure state, for Exception level 3."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Exlevel3Ns::Disabled)
    }
    #[doc = "The trace unit generates instruction trace, in Non-secure state, for Exception level 3."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Exlevel3Ns::Enabled)
    }
}
impl R {
    #[doc = "Bits 0:4 - Select which resource number should be filtered."]
    #[inline(always)]
    pub fn event_sel(&self) -> EventSelR {
        EventSelR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 9 - When TRCIDR4.NUMACPAIRS &gt; 0 or TRCIDR4.NUMPC &gt; 0, this bit returns the status of the start/stop logic."]
    #[inline(always)]
    pub fn ssstatus(&self) -> SsstatusR {
        SsstatusR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Controls whether a trace unit must trace a Reset exception."]
    #[inline(always)]
    pub fn trcreset(&self) -> TrcresetR {
        TrcresetR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - When TRCIDR3.TRCERR==1, this bit controls whether a trace unit must trace a System error exception."]
    #[inline(always)]
    pub fn trcerr(&self) -> TrcerrR {
        TrcerrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - In Secure state, each bit controls whether instruction tracing is enabled for the corresponding Exception level 0."]
    #[inline(always)]
    pub fn exlevel0_s(&self) -> Exlevel0SR {
        Exlevel0SR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - In Secure state, each bit controls whether instruction tracing is enabled for the corresponding Exception level 1."]
    #[inline(always)]
    pub fn exlevel1_s(&self) -> Exlevel1SR {
        Exlevel1SR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - In Secure state, each bit controls whether instruction tracing is enabled for the corresponding Exception level 2."]
    #[inline(always)]
    pub fn exlevel2_s(&self) -> Exlevel2SR {
        Exlevel2SR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - In Secure state, each bit controls whether instruction tracing is enabled for the corresponding Exception level 3."]
    #[inline(always)]
    pub fn exlevel3_s(&self) -> Exlevel3SR {
        Exlevel3SR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - In Non-secure state, each bit controls whether instruction tracing is enabled for the corresponding Exception level 0."]
    #[inline(always)]
    pub fn exlevel0_ns(&self) -> Exlevel0NsR {
        Exlevel0NsR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - In Non-secure state, each bit controls whether instruction tracing is enabled for the corresponding Exception level 1."]
    #[inline(always)]
    pub fn exlevel1_ns(&self) -> Exlevel1NsR {
        Exlevel1NsR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - In Non-secure state, each bit controls whether instruction tracing is enabled for the corresponding Exception level 2."]
    #[inline(always)]
    pub fn exlevel2_ns(&self) -> Exlevel2NsR {
        Exlevel2NsR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - In Non-secure state, each bit controls whether instruction tracing is enabled for the corresponding Exception level 3."]
    #[inline(always)]
    pub fn exlevel3_ns(&self) -> Exlevel3NsR {
        Exlevel3NsR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Select which resource number should be filtered."]
    #[inline(always)]
    pub fn event_sel(&mut self) -> EventSelW<'_, TrcvictlrSpec> {
        EventSelW::new(self, 0)
    }
    #[doc = "Bit 9 - When TRCIDR4.NUMACPAIRS &gt; 0 or TRCIDR4.NUMPC &gt; 0, this bit returns the status of the start/stop logic."]
    #[inline(always)]
    pub fn ssstatus(&mut self) -> SsstatusW<'_, TrcvictlrSpec> {
        SsstatusW::new(self, 9)
    }
    #[doc = "Bit 10 - Controls whether a trace unit must trace a Reset exception."]
    #[inline(always)]
    pub fn trcreset(&mut self) -> TrcresetW<'_, TrcvictlrSpec> {
        TrcresetW::new(self, 10)
    }
    #[doc = "Bit 11 - When TRCIDR3.TRCERR==1, this bit controls whether a trace unit must trace a System error exception."]
    #[inline(always)]
    pub fn trcerr(&mut self) -> TrcerrW<'_, TrcvictlrSpec> {
        TrcerrW::new(self, 11)
    }
    #[doc = "Bit 16 - In Secure state, each bit controls whether instruction tracing is enabled for the corresponding Exception level 0."]
    #[inline(always)]
    pub fn exlevel0_s(&mut self) -> Exlevel0SW<'_, TrcvictlrSpec> {
        Exlevel0SW::new(self, 16)
    }
    #[doc = "Bit 17 - In Secure state, each bit controls whether instruction tracing is enabled for the corresponding Exception level 1."]
    #[inline(always)]
    pub fn exlevel1_s(&mut self) -> Exlevel1SW<'_, TrcvictlrSpec> {
        Exlevel1SW::new(self, 17)
    }
    #[doc = "Bit 18 - In Secure state, each bit controls whether instruction tracing is enabled for the corresponding Exception level 2."]
    #[inline(always)]
    pub fn exlevel2_s(&mut self) -> Exlevel2SW<'_, TrcvictlrSpec> {
        Exlevel2SW::new(self, 18)
    }
    #[doc = "Bit 19 - In Secure state, each bit controls whether instruction tracing is enabled for the corresponding Exception level 3."]
    #[inline(always)]
    pub fn exlevel3_s(&mut self) -> Exlevel3SW<'_, TrcvictlrSpec> {
        Exlevel3SW::new(self, 19)
    }
    #[doc = "Bit 20 - In Non-secure state, each bit controls whether instruction tracing is enabled for the corresponding Exception level 0."]
    #[inline(always)]
    pub fn exlevel0_ns(&mut self) -> Exlevel0NsW<'_, TrcvictlrSpec> {
        Exlevel0NsW::new(self, 20)
    }
    #[doc = "Bit 21 - In Non-secure state, each bit controls whether instruction tracing is enabled for the corresponding Exception level 1."]
    #[inline(always)]
    pub fn exlevel1_ns(&mut self) -> Exlevel1NsW<'_, TrcvictlrSpec> {
        Exlevel1NsW::new(self, 21)
    }
    #[doc = "Bit 22 - In Non-secure state, each bit controls whether instruction tracing is enabled for the corresponding Exception level 2."]
    #[inline(always)]
    pub fn exlevel2_ns(&mut self) -> Exlevel2NsW<'_, TrcvictlrSpec> {
        Exlevel2NsW::new(self, 22)
    }
    #[doc = "Bit 23 - In Non-secure state, each bit controls whether instruction tracing is enabled for the corresponding Exception level 3."]
    #[inline(always)]
    pub fn exlevel3_ns(&mut self) -> Exlevel3NsW<'_, TrcvictlrSpec> {
        Exlevel3NsW::new(self, 23)
    }
}
#[doc = "Controls instruction trace filtering. Might ignore writes when the trace unit is enabled or not idle. Only returns stable data when TRCSTATR.PMSTABLE == 1. Must be programmed, particularly to set the value of the SSSTATUS bit, which sets the state of the start/stop logic.\n\nYou can [`read`](crate::Reg::read) this register and get [`trcvictlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcvictlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrcvictlrSpec;
impl crate::RegisterSpec for TrcvictlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcvictlr::R`](R) reader structure"]
impl crate::Readable for TrcvictlrSpec {}
#[doc = "`write(|w| ..)` method takes [`trcvictlr::W`](W) writer structure"]
impl crate::Writable for TrcvictlrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRCVICTLR to value 0"]
impl crate::Resettable for TrcvictlrSpec {}
