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
