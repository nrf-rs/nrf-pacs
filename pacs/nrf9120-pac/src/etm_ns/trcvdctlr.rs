#[doc = "Register `TRCVDCTLR` reader"]
pub type R = crate::R<TrcvdctlrSpec>;
#[doc = "Register `TRCVDCTLR` writer"]
pub type W = crate::W<TrcvdctlrSpec>;
#[doc = "Event unit enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Event0 {
    #[doc = "0: The trace event is not selected for trace filtering."]
    Disabled = 0,
    #[doc = "1: The trace event is selected for trace filtering."]
    Enabled = 1,
}
impl From<Event0> for bool {
    #[inline(always)]
    fn from(variant: Event0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENT_0` reader - Event unit enable bit."]
pub type Event0R = crate::BitReader<Event0>;
impl Event0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Event0 {
        match self.bits {
            false => Event0::Disabled,
            true => Event0::Enabled,
        }
    }
    #[doc = "The trace event is not selected for trace filtering."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Event0::Disabled
    }
    #[doc = "The trace event is selected for trace filtering."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Event0::Enabled
    }
}
#[doc = "Field `EVENT_0` writer - Event unit enable bit."]
pub type Event0W<'a, REG> = crate::BitWriter<'a, REG, Event0>;
impl<'a, REG> Event0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The trace event is not selected for trace filtering."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Event0::Disabled)
    }
    #[doc = "The trace event is selected for trace filtering."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Event0::Enabled)
    }
}
#[doc = "Event unit enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Event1 {
    #[doc = "0: The trace event is not selected for trace filtering."]
    Disabled = 0,
    #[doc = "1: The trace event is selected for trace filtering."]
    Enabled = 1,
}
impl From<Event1> for bool {
    #[inline(always)]
    fn from(variant: Event1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENT_1` reader - Event unit enable bit."]
pub type Event1R = crate::BitReader<Event1>;
impl Event1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Event1 {
        match self.bits {
            false => Event1::Disabled,
            true => Event1::Enabled,
        }
    }
    #[doc = "The trace event is not selected for trace filtering."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Event1::Disabled
    }
    #[doc = "The trace event is selected for trace filtering."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Event1::Enabled
    }
}
#[doc = "Field `EVENT_1` writer - Event unit enable bit."]
pub type Event1W<'a, REG> = crate::BitWriter<'a, REG, Event1>;
impl<'a, REG> Event1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The trace event is not selected for trace filtering."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Event1::Disabled)
    }
    #[doc = "The trace event is selected for trace filtering."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Event1::Enabled)
    }
}
#[doc = "Event unit enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Event2 {
    #[doc = "0: The trace event is not selected for trace filtering."]
    Disabled = 0,
    #[doc = "1: The trace event is selected for trace filtering."]
    Enabled = 1,
}
impl From<Event2> for bool {
    #[inline(always)]
    fn from(variant: Event2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENT_2` reader - Event unit enable bit."]
pub type Event2R = crate::BitReader<Event2>;
impl Event2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Event2 {
        match self.bits {
            false => Event2::Disabled,
            true => Event2::Enabled,
        }
    }
    #[doc = "The trace event is not selected for trace filtering."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Event2::Disabled
    }
    #[doc = "The trace event is selected for trace filtering."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Event2::Enabled
    }
}
#[doc = "Field `EVENT_2` writer - Event unit enable bit."]
pub type Event2W<'a, REG> = crate::BitWriter<'a, REG, Event2>;
impl<'a, REG> Event2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The trace event is not selected for trace filtering."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Event2::Disabled)
    }
    #[doc = "The trace event is selected for trace filtering."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Event2::Enabled)
    }
}
#[doc = "Event unit enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Event3 {
    #[doc = "0: The trace event is not selected for trace filtering."]
    Disabled = 0,
    #[doc = "1: The trace event is selected for trace filtering."]
    Enabled = 1,
}
impl From<Event3> for bool {
    #[inline(always)]
    fn from(variant: Event3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENT_3` reader - Event unit enable bit."]
pub type Event3R = crate::BitReader<Event3>;
impl Event3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Event3 {
        match self.bits {
            false => Event3::Disabled,
            true => Event3::Enabled,
        }
    }
    #[doc = "The trace event is not selected for trace filtering."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Event3::Disabled
    }
    #[doc = "The trace event is selected for trace filtering."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Event3::Enabled
    }
}
#[doc = "Field `EVENT_3` writer - Event unit enable bit."]
pub type Event3W<'a, REG> = crate::BitWriter<'a, REG, Event3>;
impl<'a, REG> Event3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The trace event is not selected for trace filtering."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Event3::Disabled)
    }
    #[doc = "The trace event is selected for trace filtering."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Event3::Enabled)
    }
}
#[doc = "Event unit enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Event4 {
    #[doc = "0: The trace event is not selected for trace filtering."]
    Disabled = 0,
    #[doc = "1: The trace event is selected for trace filtering."]
    Enabled = 1,
}
impl From<Event4> for bool {
    #[inline(always)]
    fn from(variant: Event4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENT_4` reader - Event unit enable bit."]
pub type Event4R = crate::BitReader<Event4>;
impl Event4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Event4 {
        match self.bits {
            false => Event4::Disabled,
            true => Event4::Enabled,
        }
    }
    #[doc = "The trace event is not selected for trace filtering."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Event4::Disabled
    }
    #[doc = "The trace event is selected for trace filtering."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Event4::Enabled
    }
}
#[doc = "Field `EVENT_4` writer - Event unit enable bit."]
pub type Event4W<'a, REG> = crate::BitWriter<'a, REG, Event4>;
impl<'a, REG> Event4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The trace event is not selected for trace filtering."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Event4::Disabled)
    }
    #[doc = "The trace event is selected for trace filtering."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Event4::Enabled)
    }
}
#[doc = "Event unit enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Event5 {
    #[doc = "0: The trace event is not selected for trace filtering."]
    Disabled = 0,
    #[doc = "1: The trace event is selected for trace filtering."]
    Enabled = 1,
}
impl From<Event5> for bool {
    #[inline(always)]
    fn from(variant: Event5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENT_5` reader - Event unit enable bit."]
pub type Event5R = crate::BitReader<Event5>;
impl Event5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Event5 {
        match self.bits {
            false => Event5::Disabled,
            true => Event5::Enabled,
        }
    }
    #[doc = "The trace event is not selected for trace filtering."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Event5::Disabled
    }
    #[doc = "The trace event is selected for trace filtering."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Event5::Enabled
    }
}
#[doc = "Field `EVENT_5` writer - Event unit enable bit."]
pub type Event5W<'a, REG> = crate::BitWriter<'a, REG, Event5>;
impl<'a, REG> Event5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The trace event is not selected for trace filtering."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Event5::Disabled)
    }
    #[doc = "The trace event is selected for trace filtering."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Event5::Enabled)
    }
}
#[doc = "Event unit enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Event6 {
    #[doc = "0: The trace event is not selected for trace filtering."]
    Disabled = 0,
    #[doc = "1: The trace event is selected for trace filtering."]
    Enabled = 1,
}
impl From<Event6> for bool {
    #[inline(always)]
    fn from(variant: Event6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENT_6` reader - Event unit enable bit."]
pub type Event6R = crate::BitReader<Event6>;
impl Event6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Event6 {
        match self.bits {
            false => Event6::Disabled,
            true => Event6::Enabled,
        }
    }
    #[doc = "The trace event is not selected for trace filtering."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Event6::Disabled
    }
    #[doc = "The trace event is selected for trace filtering."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Event6::Enabled
    }
}
#[doc = "Field `EVENT_6` writer - Event unit enable bit."]
pub type Event6W<'a, REG> = crate::BitWriter<'a, REG, Event6>;
impl<'a, REG> Event6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The trace event is not selected for trace filtering."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Event6::Disabled)
    }
    #[doc = "The trace event is selected for trace filtering."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Event6::Enabled)
    }
}
#[doc = "Event unit enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Event7 {
    #[doc = "0: The trace event is not selected for trace filtering."]
    Disabled = 0,
    #[doc = "1: The trace event is selected for trace filtering."]
    Enabled = 1,
}
impl From<Event7> for bool {
    #[inline(always)]
    fn from(variant: Event7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENT_7` reader - Event unit enable bit."]
pub type Event7R = crate::BitReader<Event7>;
impl Event7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Event7 {
        match self.bits {
            false => Event7::Disabled,
            true => Event7::Enabled,
        }
    }
    #[doc = "The trace event is not selected for trace filtering."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Event7::Disabled
    }
    #[doc = "The trace event is selected for trace filtering."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Event7::Enabled
    }
}
#[doc = "Field `EVENT_7` writer - Event unit enable bit."]
pub type Event7W<'a, REG> = crate::BitWriter<'a, REG, Event7>;
impl<'a, REG> Event7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The trace event is not selected for trace filtering."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Event7::Disabled)
    }
    #[doc = "The trace event is selected for trace filtering."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Event7::Enabled)
    }
}
#[doc = "Controls whether a trace unit traces data for transfers that are relative to the Stack Pointer (SP).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sprel {
    #[doc = "0: The trace unit does not affect the tracing of SP-relative transfers."]
    Enabled = 0,
    #[doc = "2: The trace unit does not trace the address portion of SP-relative transfers. If data value tracing is enabled then the trace unit generates a P1 data address element."]
    DataOnly = 2,
    #[doc = "3: The trace unit does not trace the address or value portions of SP-relative transfers."]
    Disabled = 3,
}
impl From<Sprel> for u8 {
    #[inline(always)]
    fn from(variant: Sprel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sprel {
    type Ux = u8;
}
impl crate::IsEnum for Sprel {}
#[doc = "Field `SPREL` reader - Controls whether a trace unit traces data for transfers that are relative to the Stack Pointer (SP)."]
pub type SprelR = crate::FieldReader<Sprel>;
impl SprelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sprel> {
        match self.bits {
            0 => Some(Sprel::Enabled),
            2 => Some(Sprel::DataOnly),
            3 => Some(Sprel::Disabled),
            _ => None,
        }
    }
    #[doc = "The trace unit does not affect the tracing of SP-relative transfers."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Sprel::Enabled
    }
    #[doc = "The trace unit does not trace the address portion of SP-relative transfers. If data value tracing is enabled then the trace unit generates a P1 data address element."]
    #[inline(always)]
    pub fn is_data_only(&self) -> bool {
        *self == Sprel::DataOnly
    }
    #[doc = "The trace unit does not trace the address or value portions of SP-relative transfers."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Sprel::Disabled
    }
}
#[doc = "Field `SPREL` writer - Controls whether a trace unit traces data for transfers that are relative to the Stack Pointer (SP)."]
pub type SprelW<'a, REG> = crate::FieldWriter<'a, REG, 2, Sprel>;
impl<'a, REG> SprelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The trace unit does not affect the tracing of SP-relative transfers."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sprel::Enabled)
    }
    #[doc = "The trace unit does not trace the address portion of SP-relative transfers. If data value tracing is enabled then the trace unit generates a P1 data address element."]
    #[inline(always)]
    pub fn data_only(self) -> &'a mut crate::W<REG> {
        self.variant(Sprel::DataOnly)
    }
    #[doc = "The trace unit does not trace the address or value portions of SP-relative transfers."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sprel::Disabled)
    }
}
#[doc = "Controls whether a trace unit traces data for transfers that are relative to the Program Counter (PC).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pcrel {
    #[doc = "0: The trace unit does not affect the tracing of PC-relative transfers."]
    Enabled = 0,
    #[doc = "1: The trace unit does not trace the address or value portions of PC-relative transfers."]
    Disabled = 1,
}
impl From<Pcrel> for bool {
    #[inline(always)]
    fn from(variant: Pcrel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCREL` reader - Controls whether a trace unit traces data for transfers that are relative to the Program Counter (PC)."]
pub type PcrelR = crate::BitReader<Pcrel>;
impl PcrelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pcrel {
        match self.bits {
            false => Pcrel::Enabled,
            true => Pcrel::Disabled,
        }
    }
    #[doc = "The trace unit does not affect the tracing of PC-relative transfers."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pcrel::Enabled
    }
    #[doc = "The trace unit does not trace the address or value portions of PC-relative transfers."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pcrel::Disabled
    }
}
#[doc = "Field `PCREL` writer - Controls whether a trace unit traces data for transfers that are relative to the Program Counter (PC)."]
pub type PcrelW<'a, REG> = crate::BitWriter<'a, REG, Pcrel>;
impl<'a, REG> PcrelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The trace unit does not affect the tracing of PC-relative transfers."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pcrel::Enabled)
    }
    #[doc = "The trace unit does not trace the address or value portions of PC-relative transfers."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pcrel::Disabled)
    }
}
#[doc = "Controls which information a trace unit populates in bits\\[63:56\\] of the data address.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tbi {
    #[doc = "0: The trace unit assigns bits\\[63:56\\] to have the same value as bit\\[55\\] of the data address, that is, it sign-extends the value."]
    SignExtend = 0,
    #[doc = "1: The trace unit assigns bits\\[63:56\\] to have the same value as bits\\[63:56\\] of the data address."]
    Copy = 1,
}
impl From<Tbi> for bool {
    #[inline(always)]
    fn from(variant: Tbi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TBI` reader - Controls which information a trace unit populates in bits\\[63:56\\] of the data address."]
pub type TbiR = crate::BitReader<Tbi>;
impl TbiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tbi {
        match self.bits {
            false => Tbi::SignExtend,
            true => Tbi::Copy,
        }
    }
    #[doc = "The trace unit assigns bits\\[63:56\\] to have the same value as bit\\[55\\] of the data address, that is, it sign-extends the value."]
    #[inline(always)]
    pub fn is_sign_extend(&self) -> bool {
        *self == Tbi::SignExtend
    }
    #[doc = "The trace unit assigns bits\\[63:56\\] to have the same value as bits\\[63:56\\] of the data address."]
    #[inline(always)]
    pub fn is_copy(&self) -> bool {
        *self == Tbi::Copy
    }
}
#[doc = "Field `TBI` writer - Controls which information a trace unit populates in bits\\[63:56\\] of the data address."]
pub type TbiW<'a, REG> = crate::BitWriter<'a, REG, Tbi>;
impl<'a, REG> TbiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The trace unit assigns bits\\[63:56\\] to have the same value as bit\\[55\\] of the data address, that is, it sign-extends the value."]
    #[inline(always)]
    pub fn sign_extend(self) -> &'a mut crate::W<REG> {
        self.variant(Tbi::SignExtend)
    }
    #[doc = "The trace unit assigns bits\\[63:56\\] to have the same value as bits\\[63:56\\] of the data address."]
    #[inline(always)]
    pub fn copy(self) -> &'a mut crate::W<REG> {
        self.variant(Tbi::Copy)
    }
}
#[doc = "Controls the tracing of data transfers for exceptions and exception returns on Armv6-M, Armv7-M, and Armv8-M PEs.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trcexdata {
    #[doc = "0: Exception and exception return data transfers are not traced."]
    Disabled = 0,
    #[doc = "1: Exception and exception return data transfers are traced if the other aspects of ViewData indicate that the data transfers must be traced."]
    Enabled = 1,
}
impl From<Trcexdata> for bool {
    #[inline(always)]
    fn from(variant: Trcexdata) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRCEXDATA` reader - Controls the tracing of data transfers for exceptions and exception returns on Armv6-M, Armv7-M, and Armv8-M PEs."]
pub type TrcexdataR = crate::BitReader<Trcexdata>;
impl TrcexdataR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trcexdata {
        match self.bits {
            false => Trcexdata::Disabled,
            true => Trcexdata::Enabled,
        }
    }
    #[doc = "Exception and exception return data transfers are not traced."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Trcexdata::Disabled
    }
    #[doc = "Exception and exception return data transfers are traced if the other aspects of ViewData indicate that the data transfers must be traced."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Trcexdata::Enabled
    }
}
#[doc = "Field `TRCEXDATA` writer - Controls the tracing of data transfers for exceptions and exception returns on Armv6-M, Armv7-M, and Armv8-M PEs."]
pub type TrcexdataW<'a, REG> = crate::BitWriter<'a, REG, Trcexdata>;
impl<'a, REG> TrcexdataW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exception and exception return data transfers are not traced."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Trcexdata::Disabled)
    }
    #[doc = "Exception and exception return data transfers are traced if the other aspects of ViewData indicate that the data transfers must be traced."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Trcexdata::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Event unit enable bit."]
    #[inline(always)]
    pub fn event_0(&self) -> Event0R {
        Event0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Event unit enable bit."]
    #[inline(always)]
    pub fn event_1(&self) -> Event1R {
        Event1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Event unit enable bit."]
    #[inline(always)]
    pub fn event_2(&self) -> Event2R {
        Event2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Event unit enable bit."]
    #[inline(always)]
    pub fn event_3(&self) -> Event3R {
        Event3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Event unit enable bit."]
    #[inline(always)]
    pub fn event_4(&self) -> Event4R {
        Event4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Event unit enable bit."]
    #[inline(always)]
    pub fn event_5(&self) -> Event5R {
        Event5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Event unit enable bit."]
    #[inline(always)]
    pub fn event_6(&self) -> Event6R {
        Event6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Event unit enable bit."]
    #[inline(always)]
    pub fn event_7(&self) -> Event7R {
        Event7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Controls whether a trace unit traces data for transfers that are relative to the Stack Pointer (SP)."]
    #[inline(always)]
    pub fn sprel(&self) -> SprelR {
        SprelR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Controls whether a trace unit traces data for transfers that are relative to the Program Counter (PC)."]
    #[inline(always)]
    pub fn pcrel(&self) -> PcrelR {
        PcrelR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Controls which information a trace unit populates in bits\\[63:56\\] of the data address."]
    #[inline(always)]
    pub fn tbi(&self) -> TbiR {
        TbiR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Controls the tracing of data transfers for exceptions and exception returns on Armv6-M, Armv7-M, and Armv8-M PEs."]
    #[inline(always)]
    pub fn trcexdata(&self) -> TrcexdataR {
        TrcexdataR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Event unit enable bit."]
    #[inline(always)]
    pub fn event_0(&mut self) -> Event0W<'_, TrcvdctlrSpec> {
        Event0W::new(self, 0)
    }
    #[doc = "Bit 1 - Event unit enable bit."]
    #[inline(always)]
    pub fn event_1(&mut self) -> Event1W<'_, TrcvdctlrSpec> {
        Event1W::new(self, 1)
    }
    #[doc = "Bit 2 - Event unit enable bit."]
    #[inline(always)]
    pub fn event_2(&mut self) -> Event2W<'_, TrcvdctlrSpec> {
        Event2W::new(self, 2)
    }
    #[doc = "Bit 3 - Event unit enable bit."]
    #[inline(always)]
    pub fn event_3(&mut self) -> Event3W<'_, TrcvdctlrSpec> {
        Event3W::new(self, 3)
    }
    #[doc = "Bit 4 - Event unit enable bit."]
    #[inline(always)]
    pub fn event_4(&mut self) -> Event4W<'_, TrcvdctlrSpec> {
        Event4W::new(self, 4)
    }
    #[doc = "Bit 5 - Event unit enable bit."]
    #[inline(always)]
    pub fn event_5(&mut self) -> Event5W<'_, TrcvdctlrSpec> {
        Event5W::new(self, 5)
    }
    #[doc = "Bit 6 - Event unit enable bit."]
    #[inline(always)]
    pub fn event_6(&mut self) -> Event6W<'_, TrcvdctlrSpec> {
        Event6W::new(self, 6)
    }
    #[doc = "Bit 7 - Event unit enable bit."]
    #[inline(always)]
    pub fn event_7(&mut self) -> Event7W<'_, TrcvdctlrSpec> {
        Event7W::new(self, 7)
    }
    #[doc = "Bits 8:9 - Controls whether a trace unit traces data for transfers that are relative to the Stack Pointer (SP)."]
    #[inline(always)]
    pub fn sprel(&mut self) -> SprelW<'_, TrcvdctlrSpec> {
        SprelW::new(self, 8)
    }
    #[doc = "Bit 10 - Controls whether a trace unit traces data for transfers that are relative to the Program Counter (PC)."]
    #[inline(always)]
    pub fn pcrel(&mut self) -> PcrelW<'_, TrcvdctlrSpec> {
        PcrelW::new(self, 10)
    }
    #[doc = "Bit 11 - Controls which information a trace unit populates in bits\\[63:56\\] of the data address."]
    #[inline(always)]
    pub fn tbi(&mut self) -> TbiW<'_, TrcvdctlrSpec> {
        TbiW::new(self, 11)
    }
    #[doc = "Bit 12 - Controls the tracing of data transfers for exceptions and exception returns on Armv6-M, Armv7-M, and Armv8-M PEs."]
    #[inline(always)]
    pub fn trcexdata(&mut self) -> TrcexdataW<'_, TrcvdctlrSpec> {
        TrcexdataW::new(self, 12)
    }
}
#[doc = "Controls data trace filtering. Might ignore writes when the trace unit is enabled or not idle. This register must be programmed when data tracing is enabled, that is, when either TRCCONFIGR.DA == 1 or TRCCONFIGR.DV == 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`trcvdctlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcvdctlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrcvdctlrSpec;
impl crate::RegisterSpec for TrcvdctlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcvdctlr::R`](R) reader structure"]
impl crate::Readable for TrcvdctlrSpec {}
#[doc = "`write(|w| ..)` method takes [`trcvdctlr::W`](W) writer structure"]
impl crate::Writable for TrcvdctlrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRCVDCTLR to value 0"]
impl crate::Resettable for TrcvdctlrSpec {}
