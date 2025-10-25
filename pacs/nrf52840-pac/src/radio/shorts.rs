#[doc = "Register `SHORTS` reader"]
pub type R = crate::R<ShortsSpec>;
#[doc = "Register `SHORTS` writer"]
pub type W = crate::W<ShortsSpec>;
#[doc = "Shortcut between event READY and task START\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReadyStart {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<ReadyStart> for bool {
    #[inline(always)]
    fn from(variant: ReadyStart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READY_START` reader - Shortcut between event READY and task START"]
pub type ReadyStartR = crate::BitReader<ReadyStart>;
impl ReadyStartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ReadyStart {
        match self.bits {
            false => ReadyStart::Disabled,
            true => ReadyStart::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ReadyStart::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ReadyStart::Enabled
    }
}
#[doc = "Field `READY_START` writer - Shortcut between event READY and task START"]
pub type ReadyStartW<'a, REG> = crate::BitWriter<'a, REG, ReadyStart>;
impl<'a, REG> ReadyStartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ReadyStart::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ReadyStart::Enabled)
    }
}
#[doc = "Shortcut between event END and task DISABLE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EndDisable {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<EndDisable> for bool {
    #[inline(always)]
    fn from(variant: EndDisable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `END_DISABLE` reader - Shortcut between event END and task DISABLE"]
pub type EndDisableR = crate::BitReader<EndDisable>;
impl EndDisableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EndDisable {
        match self.bits {
            false => EndDisable::Disabled,
            true => EndDisable::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EndDisable::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EndDisable::Enabled
    }
}
#[doc = "Field `END_DISABLE` writer - Shortcut between event END and task DISABLE"]
pub type EndDisableW<'a, REG> = crate::BitWriter<'a, REG, EndDisable>;
impl<'a, REG> EndDisableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EndDisable::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EndDisable::Enabled)
    }
}
#[doc = "Shortcut between event DISABLED and task TXEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DisabledTxen {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<DisabledTxen> for bool {
    #[inline(always)]
    fn from(variant: DisabledTxen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISABLED_TXEN` reader - Shortcut between event DISABLED and task TXEN"]
pub type DisabledTxenR = crate::BitReader<DisabledTxen>;
impl DisabledTxenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DisabledTxen {
        match self.bits {
            false => DisabledTxen::Disabled,
            true => DisabledTxen::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DisabledTxen::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DisabledTxen::Enabled
    }
}
#[doc = "Field `DISABLED_TXEN` writer - Shortcut between event DISABLED and task TXEN"]
pub type DisabledTxenW<'a, REG> = crate::BitWriter<'a, REG, DisabledTxen>;
impl<'a, REG> DisabledTxenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DisabledTxen::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DisabledTxen::Enabled)
    }
}
#[doc = "Shortcut between event DISABLED and task RXEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DisabledRxen {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<DisabledRxen> for bool {
    #[inline(always)]
    fn from(variant: DisabledRxen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISABLED_RXEN` reader - Shortcut between event DISABLED and task RXEN"]
pub type DisabledRxenR = crate::BitReader<DisabledRxen>;
impl DisabledRxenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DisabledRxen {
        match self.bits {
            false => DisabledRxen::Disabled,
            true => DisabledRxen::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DisabledRxen::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DisabledRxen::Enabled
    }
}
#[doc = "Field `DISABLED_RXEN` writer - Shortcut between event DISABLED and task RXEN"]
pub type DisabledRxenW<'a, REG> = crate::BitWriter<'a, REG, DisabledRxen>;
impl<'a, REG> DisabledRxenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DisabledRxen::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DisabledRxen::Enabled)
    }
}
#[doc = "Shortcut between event ADDRESS and task RSSISTART\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AddressRssistart {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<AddressRssistart> for bool {
    #[inline(always)]
    fn from(variant: AddressRssistart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDRESS_RSSISTART` reader - Shortcut between event ADDRESS and task RSSISTART"]
pub type AddressRssistartR = crate::BitReader<AddressRssistart>;
impl AddressRssistartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AddressRssistart {
        match self.bits {
            false => AddressRssistart::Disabled,
            true => AddressRssistart::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AddressRssistart::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AddressRssistart::Enabled
    }
}
#[doc = "Field `ADDRESS_RSSISTART` writer - Shortcut between event ADDRESS and task RSSISTART"]
pub type AddressRssistartW<'a, REG> = crate::BitWriter<'a, REG, AddressRssistart>;
impl<'a, REG> AddressRssistartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(AddressRssistart::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AddressRssistart::Enabled)
    }
}
#[doc = "Shortcut between event END and task START\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EndStart {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<EndStart> for bool {
    #[inline(always)]
    fn from(variant: EndStart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `END_START` reader - Shortcut between event END and task START"]
pub type EndStartR = crate::BitReader<EndStart>;
impl EndStartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EndStart {
        match self.bits {
            false => EndStart::Disabled,
            true => EndStart::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EndStart::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EndStart::Enabled
    }
}
#[doc = "Field `END_START` writer - Shortcut between event END and task START"]
pub type EndStartW<'a, REG> = crate::BitWriter<'a, REG, EndStart>;
impl<'a, REG> EndStartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EndStart::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EndStart::Enabled)
    }
}
#[doc = "Shortcut between event ADDRESS and task BCSTART\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AddressBcstart {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<AddressBcstart> for bool {
    #[inline(always)]
    fn from(variant: AddressBcstart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDRESS_BCSTART` reader - Shortcut between event ADDRESS and task BCSTART"]
pub type AddressBcstartR = crate::BitReader<AddressBcstart>;
impl AddressBcstartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AddressBcstart {
        match self.bits {
            false => AddressBcstart::Disabled,
            true => AddressBcstart::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AddressBcstart::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AddressBcstart::Enabled
    }
}
#[doc = "Field `ADDRESS_BCSTART` writer - Shortcut between event ADDRESS and task BCSTART"]
pub type AddressBcstartW<'a, REG> = crate::BitWriter<'a, REG, AddressBcstart>;
impl<'a, REG> AddressBcstartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(AddressBcstart::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AddressBcstart::Enabled)
    }
}
#[doc = "Shortcut between event DISABLED and task RSSISTOP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DisabledRssistop {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<DisabledRssistop> for bool {
    #[inline(always)]
    fn from(variant: DisabledRssistop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISABLED_RSSISTOP` reader - Shortcut between event DISABLED and task RSSISTOP"]
pub type DisabledRssistopR = crate::BitReader<DisabledRssistop>;
impl DisabledRssistopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DisabledRssistop {
        match self.bits {
            false => DisabledRssistop::Disabled,
            true => DisabledRssistop::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DisabledRssistop::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DisabledRssistop::Enabled
    }
}
#[doc = "Field `DISABLED_RSSISTOP` writer - Shortcut between event DISABLED and task RSSISTOP"]
pub type DisabledRssistopW<'a, REG> = crate::BitWriter<'a, REG, DisabledRssistop>;
impl<'a, REG> DisabledRssistopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DisabledRssistop::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DisabledRssistop::Enabled)
    }
}
#[doc = "Shortcut between event RXREADY and task CCASTART\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RxreadyCcastart {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<RxreadyCcastart> for bool {
    #[inline(always)]
    fn from(variant: RxreadyCcastart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXREADY_CCASTART` reader - Shortcut between event RXREADY and task CCASTART"]
pub type RxreadyCcastartR = crate::BitReader<RxreadyCcastart>;
impl RxreadyCcastartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RxreadyCcastart {
        match self.bits {
            false => RxreadyCcastart::Disabled,
            true => RxreadyCcastart::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RxreadyCcastart::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RxreadyCcastart::Enabled
    }
}
#[doc = "Field `RXREADY_CCASTART` writer - Shortcut between event RXREADY and task CCASTART"]
pub type RxreadyCcastartW<'a, REG> = crate::BitWriter<'a, REG, RxreadyCcastart>;
impl<'a, REG> RxreadyCcastartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RxreadyCcastart::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RxreadyCcastart::Enabled)
    }
}
#[doc = "Shortcut between event CCAIDLE and task TXEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CcaidleTxen {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<CcaidleTxen> for bool {
    #[inline(always)]
    fn from(variant: CcaidleTxen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCAIDLE_TXEN` reader - Shortcut between event CCAIDLE and task TXEN"]
pub type CcaidleTxenR = crate::BitReader<CcaidleTxen>;
impl CcaidleTxenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CcaidleTxen {
        match self.bits {
            false => CcaidleTxen::Disabled,
            true => CcaidleTxen::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CcaidleTxen::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CcaidleTxen::Enabled
    }
}
#[doc = "Field `CCAIDLE_TXEN` writer - Shortcut between event CCAIDLE and task TXEN"]
pub type CcaidleTxenW<'a, REG> = crate::BitWriter<'a, REG, CcaidleTxen>;
impl<'a, REG> CcaidleTxenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CcaidleTxen::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CcaidleTxen::Enabled)
    }
}
#[doc = "Shortcut between event CCABUSY and task DISABLE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CcabusyDisable {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<CcabusyDisable> for bool {
    #[inline(always)]
    fn from(variant: CcabusyDisable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCABUSY_DISABLE` reader - Shortcut between event CCABUSY and task DISABLE"]
pub type CcabusyDisableR = crate::BitReader<CcabusyDisable>;
impl CcabusyDisableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CcabusyDisable {
        match self.bits {
            false => CcabusyDisable::Disabled,
            true => CcabusyDisable::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CcabusyDisable::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CcabusyDisable::Enabled
    }
}
#[doc = "Field `CCABUSY_DISABLE` writer - Shortcut between event CCABUSY and task DISABLE"]
pub type CcabusyDisableW<'a, REG> = crate::BitWriter<'a, REG, CcabusyDisable>;
impl<'a, REG> CcabusyDisableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CcabusyDisable::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CcabusyDisable::Enabled)
    }
}
#[doc = "Shortcut between event FRAMESTART and task BCSTART\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FramestartBcstart {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<FramestartBcstart> for bool {
    #[inline(always)]
    fn from(variant: FramestartBcstart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRAMESTART_BCSTART` reader - Shortcut between event FRAMESTART and task BCSTART"]
pub type FramestartBcstartR = crate::BitReader<FramestartBcstart>;
impl FramestartBcstartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FramestartBcstart {
        match self.bits {
            false => FramestartBcstart::Disabled,
            true => FramestartBcstart::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FramestartBcstart::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FramestartBcstart::Enabled
    }
}
#[doc = "Field `FRAMESTART_BCSTART` writer - Shortcut between event FRAMESTART and task BCSTART"]
pub type FramestartBcstartW<'a, REG> = crate::BitWriter<'a, REG, FramestartBcstart>;
impl<'a, REG> FramestartBcstartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FramestartBcstart::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FramestartBcstart::Enabled)
    }
}
#[doc = "Shortcut between event READY and task EDSTART\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReadyEdstart {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<ReadyEdstart> for bool {
    #[inline(always)]
    fn from(variant: ReadyEdstart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READY_EDSTART` reader - Shortcut between event READY and task EDSTART"]
pub type ReadyEdstartR = crate::BitReader<ReadyEdstart>;
impl ReadyEdstartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ReadyEdstart {
        match self.bits {
            false => ReadyEdstart::Disabled,
            true => ReadyEdstart::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ReadyEdstart::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ReadyEdstart::Enabled
    }
}
#[doc = "Field `READY_EDSTART` writer - Shortcut between event READY and task EDSTART"]
pub type ReadyEdstartW<'a, REG> = crate::BitWriter<'a, REG, ReadyEdstart>;
impl<'a, REG> ReadyEdstartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ReadyEdstart::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ReadyEdstart::Enabled)
    }
}
#[doc = "Shortcut between event EDEND and task DISABLE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EdendDisable {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<EdendDisable> for bool {
    #[inline(always)]
    fn from(variant: EdendDisable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EDEND_DISABLE` reader - Shortcut between event EDEND and task DISABLE"]
pub type EdendDisableR = crate::BitReader<EdendDisable>;
impl EdendDisableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EdendDisable {
        match self.bits {
            false => EdendDisable::Disabled,
            true => EdendDisable::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EdendDisable::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EdendDisable::Enabled
    }
}
#[doc = "Field `EDEND_DISABLE` writer - Shortcut between event EDEND and task DISABLE"]
pub type EdendDisableW<'a, REG> = crate::BitWriter<'a, REG, EdendDisable>;
impl<'a, REG> EdendDisableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EdendDisable::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EdendDisable::Enabled)
    }
}
#[doc = "Shortcut between event CCAIDLE and task STOP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CcaidleStop {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<CcaidleStop> for bool {
    #[inline(always)]
    fn from(variant: CcaidleStop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCAIDLE_STOP` reader - Shortcut between event CCAIDLE and task STOP"]
pub type CcaidleStopR = crate::BitReader<CcaidleStop>;
impl CcaidleStopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CcaidleStop {
        match self.bits {
            false => CcaidleStop::Disabled,
            true => CcaidleStop::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CcaidleStop::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CcaidleStop::Enabled
    }
}
#[doc = "Field `CCAIDLE_STOP` writer - Shortcut between event CCAIDLE and task STOP"]
pub type CcaidleStopW<'a, REG> = crate::BitWriter<'a, REG, CcaidleStop>;
impl<'a, REG> CcaidleStopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CcaidleStop::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CcaidleStop::Enabled)
    }
}
#[doc = "Shortcut between event TXREADY and task START\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxreadyStart {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<TxreadyStart> for bool {
    #[inline(always)]
    fn from(variant: TxreadyStart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXREADY_START` reader - Shortcut between event TXREADY and task START"]
pub type TxreadyStartR = crate::BitReader<TxreadyStart>;
impl TxreadyStartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TxreadyStart {
        match self.bits {
            false => TxreadyStart::Disabled,
            true => TxreadyStart::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TxreadyStart::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TxreadyStart::Enabled
    }
}
#[doc = "Field `TXREADY_START` writer - Shortcut between event TXREADY and task START"]
pub type TxreadyStartW<'a, REG> = crate::BitWriter<'a, REG, TxreadyStart>;
impl<'a, REG> TxreadyStartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TxreadyStart::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TxreadyStart::Enabled)
    }
}
#[doc = "Shortcut between event RXREADY and task START\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RxreadyStart {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<RxreadyStart> for bool {
    #[inline(always)]
    fn from(variant: RxreadyStart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXREADY_START` reader - Shortcut between event RXREADY and task START"]
pub type RxreadyStartR = crate::BitReader<RxreadyStart>;
impl RxreadyStartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RxreadyStart {
        match self.bits {
            false => RxreadyStart::Disabled,
            true => RxreadyStart::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RxreadyStart::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RxreadyStart::Enabled
    }
}
#[doc = "Field `RXREADY_START` writer - Shortcut between event RXREADY and task START"]
pub type RxreadyStartW<'a, REG> = crate::BitWriter<'a, REG, RxreadyStart>;
impl<'a, REG> RxreadyStartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RxreadyStart::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RxreadyStart::Enabled)
    }
}
#[doc = "Shortcut between event PHYEND and task DISABLE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PhyendDisable {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<PhyendDisable> for bool {
    #[inline(always)]
    fn from(variant: PhyendDisable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PHYEND_DISABLE` reader - Shortcut between event PHYEND and task DISABLE"]
pub type PhyendDisableR = crate::BitReader<PhyendDisable>;
impl PhyendDisableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PhyendDisable {
        match self.bits {
            false => PhyendDisable::Disabled,
            true => PhyendDisable::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PhyendDisable::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PhyendDisable::Enabled
    }
}
#[doc = "Field `PHYEND_DISABLE` writer - Shortcut between event PHYEND and task DISABLE"]
pub type PhyendDisableW<'a, REG> = crate::BitWriter<'a, REG, PhyendDisable>;
impl<'a, REG> PhyendDisableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PhyendDisable::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PhyendDisable::Enabled)
    }
}
#[doc = "Shortcut between event PHYEND and task START\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PhyendStart {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<PhyendStart> for bool {
    #[inline(always)]
    fn from(variant: PhyendStart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PHYEND_START` reader - Shortcut between event PHYEND and task START"]
pub type PhyendStartR = crate::BitReader<PhyendStart>;
impl PhyendStartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PhyendStart {
        match self.bits {
            false => PhyendStart::Disabled,
            true => PhyendStart::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PhyendStart::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PhyendStart::Enabled
    }
}
#[doc = "Field `PHYEND_START` writer - Shortcut between event PHYEND and task START"]
pub type PhyendStartW<'a, REG> = crate::BitWriter<'a, REG, PhyendStart>;
impl<'a, REG> PhyendStartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PhyendStart::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PhyendStart::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Shortcut between event READY and task START"]
    #[inline(always)]
    pub fn ready_start(&self) -> ReadyStartR {
        ReadyStartR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Shortcut between event END and task DISABLE"]
    #[inline(always)]
    pub fn end_disable(&self) -> EndDisableR {
        EndDisableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Shortcut between event DISABLED and task TXEN"]
    #[inline(always)]
    pub fn disabled_txen(&self) -> DisabledTxenR {
        DisabledTxenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Shortcut between event DISABLED and task RXEN"]
    #[inline(always)]
    pub fn disabled_rxen(&self) -> DisabledRxenR {
        DisabledRxenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Shortcut between event ADDRESS and task RSSISTART"]
    #[inline(always)]
    pub fn address_rssistart(&self) -> AddressRssistartR {
        AddressRssistartR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Shortcut between event END and task START"]
    #[inline(always)]
    pub fn end_start(&self) -> EndStartR {
        EndStartR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Shortcut between event ADDRESS and task BCSTART"]
    #[inline(always)]
    pub fn address_bcstart(&self) -> AddressBcstartR {
        AddressBcstartR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Shortcut between event DISABLED and task RSSISTOP"]
    #[inline(always)]
    pub fn disabled_rssistop(&self) -> DisabledRssistopR {
        DisabledRssistopR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - Shortcut between event RXREADY and task CCASTART"]
    #[inline(always)]
    pub fn rxready_ccastart(&self) -> RxreadyCcastartR {
        RxreadyCcastartR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Shortcut between event CCAIDLE and task TXEN"]
    #[inline(always)]
    pub fn ccaidle_txen(&self) -> CcaidleTxenR {
        CcaidleTxenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Shortcut between event CCABUSY and task DISABLE"]
    #[inline(always)]
    pub fn ccabusy_disable(&self) -> CcabusyDisableR {
        CcabusyDisableR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Shortcut between event FRAMESTART and task BCSTART"]
    #[inline(always)]
    pub fn framestart_bcstart(&self) -> FramestartBcstartR {
        FramestartBcstartR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Shortcut between event READY and task EDSTART"]
    #[inline(always)]
    pub fn ready_edstart(&self) -> ReadyEdstartR {
        ReadyEdstartR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Shortcut between event EDEND and task DISABLE"]
    #[inline(always)]
    pub fn edend_disable(&self) -> EdendDisableR {
        EdendDisableR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Shortcut between event CCAIDLE and task STOP"]
    #[inline(always)]
    pub fn ccaidle_stop(&self) -> CcaidleStopR {
        CcaidleStopR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Shortcut between event TXREADY and task START"]
    #[inline(always)]
    pub fn txready_start(&self) -> TxreadyStartR {
        TxreadyStartR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Shortcut between event RXREADY and task START"]
    #[inline(always)]
    pub fn rxready_start(&self) -> RxreadyStartR {
        RxreadyStartR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Shortcut between event PHYEND and task DISABLE"]
    #[inline(always)]
    pub fn phyend_disable(&self) -> PhyendDisableR {
        PhyendDisableR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Shortcut between event PHYEND and task START"]
    #[inline(always)]
    pub fn phyend_start(&self) -> PhyendStartR {
        PhyendStartR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Shortcut between event READY and task START"]
    #[inline(always)]
    pub fn ready_start(&mut self) -> ReadyStartW<'_, ShortsSpec> {
        ReadyStartW::new(self, 0)
    }
    #[doc = "Bit 1 - Shortcut between event END and task DISABLE"]
    #[inline(always)]
    pub fn end_disable(&mut self) -> EndDisableW<'_, ShortsSpec> {
        EndDisableW::new(self, 1)
    }
    #[doc = "Bit 2 - Shortcut between event DISABLED and task TXEN"]
    #[inline(always)]
    pub fn disabled_txen(&mut self) -> DisabledTxenW<'_, ShortsSpec> {
        DisabledTxenW::new(self, 2)
    }
    #[doc = "Bit 3 - Shortcut between event DISABLED and task RXEN"]
    #[inline(always)]
    pub fn disabled_rxen(&mut self) -> DisabledRxenW<'_, ShortsSpec> {
        DisabledRxenW::new(self, 3)
    }
    #[doc = "Bit 4 - Shortcut between event ADDRESS and task RSSISTART"]
    #[inline(always)]
    pub fn address_rssistart(&mut self) -> AddressRssistartW<'_, ShortsSpec> {
        AddressRssistartW::new(self, 4)
    }
    #[doc = "Bit 5 - Shortcut between event END and task START"]
    #[inline(always)]
    pub fn end_start(&mut self) -> EndStartW<'_, ShortsSpec> {
        EndStartW::new(self, 5)
    }
    #[doc = "Bit 6 - Shortcut between event ADDRESS and task BCSTART"]
    #[inline(always)]
    pub fn address_bcstart(&mut self) -> AddressBcstartW<'_, ShortsSpec> {
        AddressBcstartW::new(self, 6)
    }
    #[doc = "Bit 8 - Shortcut between event DISABLED and task RSSISTOP"]
    #[inline(always)]
    pub fn disabled_rssistop(&mut self) -> DisabledRssistopW<'_, ShortsSpec> {
        DisabledRssistopW::new(self, 8)
    }
    #[doc = "Bit 11 - Shortcut between event RXREADY and task CCASTART"]
    #[inline(always)]
    pub fn rxready_ccastart(&mut self) -> RxreadyCcastartW<'_, ShortsSpec> {
        RxreadyCcastartW::new(self, 11)
    }
    #[doc = "Bit 12 - Shortcut between event CCAIDLE and task TXEN"]
    #[inline(always)]
    pub fn ccaidle_txen(&mut self) -> CcaidleTxenW<'_, ShortsSpec> {
        CcaidleTxenW::new(self, 12)
    }
    #[doc = "Bit 13 - Shortcut between event CCABUSY and task DISABLE"]
    #[inline(always)]
    pub fn ccabusy_disable(&mut self) -> CcabusyDisableW<'_, ShortsSpec> {
        CcabusyDisableW::new(self, 13)
    }
    #[doc = "Bit 14 - Shortcut between event FRAMESTART and task BCSTART"]
    #[inline(always)]
    pub fn framestart_bcstart(&mut self) -> FramestartBcstartW<'_, ShortsSpec> {
        FramestartBcstartW::new(self, 14)
    }
    #[doc = "Bit 15 - Shortcut between event READY and task EDSTART"]
    #[inline(always)]
    pub fn ready_edstart(&mut self) -> ReadyEdstartW<'_, ShortsSpec> {
        ReadyEdstartW::new(self, 15)
    }
    #[doc = "Bit 16 - Shortcut between event EDEND and task DISABLE"]
    #[inline(always)]
    pub fn edend_disable(&mut self) -> EdendDisableW<'_, ShortsSpec> {
        EdendDisableW::new(self, 16)
    }
    #[doc = "Bit 17 - Shortcut between event CCAIDLE and task STOP"]
    #[inline(always)]
    pub fn ccaidle_stop(&mut self) -> CcaidleStopW<'_, ShortsSpec> {
        CcaidleStopW::new(self, 17)
    }
    #[doc = "Bit 18 - Shortcut between event TXREADY and task START"]
    #[inline(always)]
    pub fn txready_start(&mut self) -> TxreadyStartW<'_, ShortsSpec> {
        TxreadyStartW::new(self, 18)
    }
    #[doc = "Bit 19 - Shortcut between event RXREADY and task START"]
    #[inline(always)]
    pub fn rxready_start(&mut self) -> RxreadyStartW<'_, ShortsSpec> {
        RxreadyStartW::new(self, 19)
    }
    #[doc = "Bit 20 - Shortcut between event PHYEND and task DISABLE"]
    #[inline(always)]
    pub fn phyend_disable(&mut self) -> PhyendDisableW<'_, ShortsSpec> {
        PhyendDisableW::new(self, 20)
    }
    #[doc = "Bit 21 - Shortcut between event PHYEND and task START"]
    #[inline(always)]
    pub fn phyend_start(&mut self) -> PhyendStartW<'_, ShortsSpec> {
        PhyendStartW::new(self, 21)
    }
}
#[doc = "Shortcuts between local events and tasks\n\nYou can [`read`](crate::Reg::read) this register and get [`shorts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shorts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShortsSpec;
impl crate::RegisterSpec for ShortsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shorts::R`](R) reader structure"]
impl crate::Readable for ShortsSpec {}
#[doc = "`write(|w| ..)` method takes [`shorts::W`](W) writer structure"]
impl crate::Writable for ShortsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SHORTS to value 0"]
impl crate::Resettable for ShortsSpec {}
