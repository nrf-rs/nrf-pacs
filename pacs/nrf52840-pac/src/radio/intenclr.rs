#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<IntenclrSpec>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
#[doc = "Write '1' to disable interrupt for event READY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ready {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Ready> for bool {
    #[inline(always)]
    fn from(variant: Ready) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READY` reader - Write '1' to disable interrupt for event READY"]
pub type ReadyR = crate::BitReader<Ready>;
impl ReadyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ready {
        match self.bits {
            false => Ready::Disabled,
            true => Ready::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ready::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ready::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event READY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReadyWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<ReadyWO> for bool {
    #[inline(always)]
    fn from(variant: ReadyWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READY` writer - Write '1' to disable interrupt for event READY"]
pub type ReadyW<'a, REG> = crate::BitWriter<'a, REG, ReadyWO>;
impl<'a, REG> ReadyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ReadyWO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event ADDRESS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Address {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Address> for bool {
    #[inline(always)]
    fn from(variant: Address) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDRESS` reader - Write '1' to disable interrupt for event ADDRESS"]
pub type AddressR = crate::BitReader<Address>;
impl AddressR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Address {
        match self.bits {
            false => Address::Disabled,
            true => Address::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Address::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Address::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event ADDRESS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AddressWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<AddressWO> for bool {
    #[inline(always)]
    fn from(variant: AddressWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDRESS` writer - Write '1' to disable interrupt for event ADDRESS"]
pub type AddressW<'a, REG> = crate::BitWriter<'a, REG, AddressWO>;
impl<'a, REG> AddressW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(AddressWO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event PAYLOAD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Payload {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Payload> for bool {
    #[inline(always)]
    fn from(variant: Payload) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAYLOAD` reader - Write '1' to disable interrupt for event PAYLOAD"]
pub type PayloadR = crate::BitReader<Payload>;
impl PayloadR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Payload {
        match self.bits {
            false => Payload::Disabled,
            true => Payload::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Payload::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Payload::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event PAYLOAD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PayloadWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<PayloadWO> for bool {
    #[inline(always)]
    fn from(variant: PayloadWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAYLOAD` writer - Write '1' to disable interrupt for event PAYLOAD"]
pub type PayloadW<'a, REG> = crate::BitWriter<'a, REG, PayloadWO>;
impl<'a, REG> PayloadW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(PayloadWO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event END\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum End {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<End> for bool {
    #[inline(always)]
    fn from(variant: End) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `END` reader - Write '1' to disable interrupt for event END"]
pub type EndR = crate::BitReader<End>;
impl EndR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> End {
        match self.bits {
            false => End::Disabled,
            true => End::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == End::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == End::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event END\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EndWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<EndWO> for bool {
    #[inline(always)]
    fn from(variant: EndWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `END` writer - Write '1' to disable interrupt for event END"]
pub type EndW<'a, REG> = crate::BitWriter<'a, REG, EndWO>;
impl<'a, REG> EndW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(EndWO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event DISABLED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Disabled {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Disabled> for bool {
    #[inline(always)]
    fn from(variant: Disabled) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISABLED` reader - Write '1' to disable interrupt for event DISABLED"]
pub type DisabledR = crate::BitReader<Disabled>;
impl DisabledR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Disabled {
        match self.bits {
            false => Disabled::Disabled,
            true => Disabled::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Disabled::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Disabled::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event DISABLED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DisabledWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<DisabledWO> for bool {
    #[inline(always)]
    fn from(variant: DisabledWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISABLED` writer - Write '1' to disable interrupt for event DISABLED"]
pub type DisabledW<'a, REG> = crate::BitWriter<'a, REG, DisabledWO>;
impl<'a, REG> DisabledW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(DisabledWO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event DEVMATCH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Devmatch {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Devmatch> for bool {
    #[inline(always)]
    fn from(variant: Devmatch) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEVMATCH` reader - Write '1' to disable interrupt for event DEVMATCH"]
pub type DevmatchR = crate::BitReader<Devmatch>;
impl DevmatchR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Devmatch {
        match self.bits {
            false => Devmatch::Disabled,
            true => Devmatch::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Devmatch::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Devmatch::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event DEVMATCH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DevmatchWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<DevmatchWO> for bool {
    #[inline(always)]
    fn from(variant: DevmatchWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEVMATCH` writer - Write '1' to disable interrupt for event DEVMATCH"]
pub type DevmatchW<'a, REG> = crate::BitWriter<'a, REG, DevmatchWO>;
impl<'a, REG> DevmatchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(DevmatchWO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event DEVMISS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Devmiss {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Devmiss> for bool {
    #[inline(always)]
    fn from(variant: Devmiss) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEVMISS` reader - Write '1' to disable interrupt for event DEVMISS"]
pub type DevmissR = crate::BitReader<Devmiss>;
impl DevmissR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Devmiss {
        match self.bits {
            false => Devmiss::Disabled,
            true => Devmiss::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Devmiss::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Devmiss::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event DEVMISS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DevmissWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<DevmissWO> for bool {
    #[inline(always)]
    fn from(variant: DevmissWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEVMISS` writer - Write '1' to disable interrupt for event DEVMISS"]
pub type DevmissW<'a, REG> = crate::BitWriter<'a, REG, DevmissWO>;
impl<'a, REG> DevmissW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(DevmissWO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event RSSIEND\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rssiend {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Rssiend> for bool {
    #[inline(always)]
    fn from(variant: Rssiend) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSSIEND` reader - Write '1' to disable interrupt for event RSSIEND"]
pub type RssiendR = crate::BitReader<Rssiend>;
impl RssiendR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rssiend {
        match self.bits {
            false => Rssiend::Disabled,
            true => Rssiend::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rssiend::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rssiend::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event RSSIEND\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RssiendWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<RssiendWO> for bool {
    #[inline(always)]
    fn from(variant: RssiendWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSSIEND` writer - Write '1' to disable interrupt for event RSSIEND"]
pub type RssiendW<'a, REG> = crate::BitWriter<'a, REG, RssiendWO>;
impl<'a, REG> RssiendW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(RssiendWO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event BCMATCH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bcmatch {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Bcmatch> for bool {
    #[inline(always)]
    fn from(variant: Bcmatch) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BCMATCH` reader - Write '1' to disable interrupt for event BCMATCH"]
pub type BcmatchR = crate::BitReader<Bcmatch>;
impl BcmatchR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bcmatch {
        match self.bits {
            false => Bcmatch::Disabled,
            true => Bcmatch::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Bcmatch::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Bcmatch::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event BCMATCH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BcmatchWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<BcmatchWO> for bool {
    #[inline(always)]
    fn from(variant: BcmatchWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BCMATCH` writer - Write '1' to disable interrupt for event BCMATCH"]
pub type BcmatchW<'a, REG> = crate::BitWriter<'a, REG, BcmatchWO>;
impl<'a, REG> BcmatchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(BcmatchWO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event CRCOK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crcok {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Crcok> for bool {
    #[inline(always)]
    fn from(variant: Crcok) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCOK` reader - Write '1' to disable interrupt for event CRCOK"]
pub type CrcokR = crate::BitReader<Crcok>;
impl CrcokR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Crcok {
        match self.bits {
            false => Crcok::Disabled,
            true => Crcok::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Crcok::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Crcok::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event CRCOK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CrcokWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<CrcokWO> for bool {
    #[inline(always)]
    fn from(variant: CrcokWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCOK` writer - Write '1' to disable interrupt for event CRCOK"]
pub type CrcokW<'a, REG> = crate::BitWriter<'a, REG, CrcokWO>;
impl<'a, REG> CrcokW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CrcokWO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event CRCERROR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crcerror {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Crcerror> for bool {
    #[inline(always)]
    fn from(variant: Crcerror) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCERROR` reader - Write '1' to disable interrupt for event CRCERROR"]
pub type CrcerrorR = crate::BitReader<Crcerror>;
impl CrcerrorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Crcerror {
        match self.bits {
            false => Crcerror::Disabled,
            true => Crcerror::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Crcerror::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Crcerror::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event CRCERROR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CrcerrorWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<CrcerrorWO> for bool {
    #[inline(always)]
    fn from(variant: CrcerrorWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCERROR` writer - Write '1' to disable interrupt for event CRCERROR"]
pub type CrcerrorW<'a, REG> = crate::BitWriter<'a, REG, CrcerrorWO>;
impl<'a, REG> CrcerrorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CrcerrorWO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event FRAMESTART\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Framestart {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Framestart> for bool {
    #[inline(always)]
    fn from(variant: Framestart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRAMESTART` reader - Write '1' to disable interrupt for event FRAMESTART"]
pub type FramestartR = crate::BitReader<Framestart>;
impl FramestartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Framestart {
        match self.bits {
            false => Framestart::Disabled,
            true => Framestart::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Framestart::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Framestart::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event FRAMESTART\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FramestartWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<FramestartWO> for bool {
    #[inline(always)]
    fn from(variant: FramestartWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRAMESTART` writer - Write '1' to disable interrupt for event FRAMESTART"]
pub type FramestartW<'a, REG> = crate::BitWriter<'a, REG, FramestartWO>;
impl<'a, REG> FramestartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(FramestartWO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event EDEND\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Edend {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Edend> for bool {
    #[inline(always)]
    fn from(variant: Edend) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EDEND` reader - Write '1' to disable interrupt for event EDEND"]
pub type EdendR = crate::BitReader<Edend>;
impl EdendR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Edend {
        match self.bits {
            false => Edend::Disabled,
            true => Edend::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Edend::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Edend::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event EDEND\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EdendWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<EdendWO> for bool {
    #[inline(always)]
    fn from(variant: EdendWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EDEND` writer - Write '1' to disable interrupt for event EDEND"]
pub type EdendW<'a, REG> = crate::BitWriter<'a, REG, EdendWO>;
impl<'a, REG> EdendW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(EdendWO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event EDSTOPPED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Edstopped {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Edstopped> for bool {
    #[inline(always)]
    fn from(variant: Edstopped) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EDSTOPPED` reader - Write '1' to disable interrupt for event EDSTOPPED"]
pub type EdstoppedR = crate::BitReader<Edstopped>;
impl EdstoppedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Edstopped {
        match self.bits {
            false => Edstopped::Disabled,
            true => Edstopped::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Edstopped::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Edstopped::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event EDSTOPPED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EdstoppedWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<EdstoppedWO> for bool {
    #[inline(always)]
    fn from(variant: EdstoppedWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EDSTOPPED` writer - Write '1' to disable interrupt for event EDSTOPPED"]
pub type EdstoppedW<'a, REG> = crate::BitWriter<'a, REG, EdstoppedWO>;
impl<'a, REG> EdstoppedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(EdstoppedWO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event CCAIDLE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccaidle {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Ccaidle> for bool {
    #[inline(always)]
    fn from(variant: Ccaidle) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCAIDLE` reader - Write '1' to disable interrupt for event CCAIDLE"]
pub type CcaidleR = crate::BitReader<Ccaidle>;
impl CcaidleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccaidle {
        match self.bits {
            false => Ccaidle::Disabled,
            true => Ccaidle::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ccaidle::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ccaidle::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event CCAIDLE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CcaidleWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<CcaidleWO> for bool {
    #[inline(always)]
    fn from(variant: CcaidleWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCAIDLE` writer - Write '1' to disable interrupt for event CCAIDLE"]
pub type CcaidleW<'a, REG> = crate::BitWriter<'a, REG, CcaidleWO>;
impl<'a, REG> CcaidleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CcaidleWO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event CCABUSY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccabusy {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Ccabusy> for bool {
    #[inline(always)]
    fn from(variant: Ccabusy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCABUSY` reader - Write '1' to disable interrupt for event CCABUSY"]
pub type CcabusyR = crate::BitReader<Ccabusy>;
impl CcabusyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccabusy {
        match self.bits {
            false => Ccabusy::Disabled,
            true => Ccabusy::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ccabusy::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ccabusy::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event CCABUSY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CcabusyWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<CcabusyWO> for bool {
    #[inline(always)]
    fn from(variant: CcabusyWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCABUSY` writer - Write '1' to disable interrupt for event CCABUSY"]
pub type CcabusyW<'a, REG> = crate::BitWriter<'a, REG, CcabusyWO>;
impl<'a, REG> CcabusyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CcabusyWO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event CCASTOPPED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccastopped {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Ccastopped> for bool {
    #[inline(always)]
    fn from(variant: Ccastopped) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCASTOPPED` reader - Write '1' to disable interrupt for event CCASTOPPED"]
pub type CcastoppedR = crate::BitReader<Ccastopped>;
impl CcastoppedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccastopped {
        match self.bits {
            false => Ccastopped::Disabled,
            true => Ccastopped::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ccastopped::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ccastopped::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event CCASTOPPED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CcastoppedWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<CcastoppedWO> for bool {
    #[inline(always)]
    fn from(variant: CcastoppedWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCASTOPPED` writer - Write '1' to disable interrupt for event CCASTOPPED"]
pub type CcastoppedW<'a, REG> = crate::BitWriter<'a, REG, CcastoppedWO>;
impl<'a, REG> CcastoppedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CcastoppedWO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event RATEBOOST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rateboost {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Rateboost> for bool {
    #[inline(always)]
    fn from(variant: Rateboost) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RATEBOOST` reader - Write '1' to disable interrupt for event RATEBOOST"]
pub type RateboostR = crate::BitReader<Rateboost>;
impl RateboostR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rateboost {
        match self.bits {
            false => Rateboost::Disabled,
            true => Rateboost::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rateboost::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rateboost::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event RATEBOOST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RateboostWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<RateboostWO> for bool {
    #[inline(always)]
    fn from(variant: RateboostWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RATEBOOST` writer - Write '1' to disable interrupt for event RATEBOOST"]
pub type RateboostW<'a, REG> = crate::BitWriter<'a, REG, RateboostWO>;
impl<'a, REG> RateboostW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(RateboostWO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event TXREADY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txready {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Txready> for bool {
    #[inline(always)]
    fn from(variant: Txready) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXREADY` reader - Write '1' to disable interrupt for event TXREADY"]
pub type TxreadyR = crate::BitReader<Txready>;
impl TxreadyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txready {
        match self.bits {
            false => Txready::Disabled,
            true => Txready::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Txready::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Txready::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event TXREADY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxreadyWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<TxreadyWO> for bool {
    #[inline(always)]
    fn from(variant: TxreadyWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXREADY` writer - Write '1' to disable interrupt for event TXREADY"]
pub type TxreadyW<'a, REG> = crate::BitWriter<'a, REG, TxreadyWO>;
impl<'a, REG> TxreadyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(TxreadyWO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event RXREADY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxready {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Rxready> for bool {
    #[inline(always)]
    fn from(variant: Rxready) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXREADY` reader - Write '1' to disable interrupt for event RXREADY"]
pub type RxreadyR = crate::BitReader<Rxready>;
impl RxreadyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxready {
        match self.bits {
            false => Rxready::Disabled,
            true => Rxready::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rxready::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rxready::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event RXREADY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RxreadyWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<RxreadyWO> for bool {
    #[inline(always)]
    fn from(variant: RxreadyWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXREADY` writer - Write '1' to disable interrupt for event RXREADY"]
pub type RxreadyW<'a, REG> = crate::BitWriter<'a, REG, RxreadyWO>;
impl<'a, REG> RxreadyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(RxreadyWO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event MHRMATCH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mhrmatch {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Mhrmatch> for bool {
    #[inline(always)]
    fn from(variant: Mhrmatch) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MHRMATCH` reader - Write '1' to disable interrupt for event MHRMATCH"]
pub type MhrmatchR = crate::BitReader<Mhrmatch>;
impl MhrmatchR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mhrmatch {
        match self.bits {
            false => Mhrmatch::Disabled,
            true => Mhrmatch::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Mhrmatch::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Mhrmatch::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event MHRMATCH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MhrmatchWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<MhrmatchWO> for bool {
    #[inline(always)]
    fn from(variant: MhrmatchWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MHRMATCH` writer - Write '1' to disable interrupt for event MHRMATCH"]
pub type MhrmatchW<'a, REG> = crate::BitWriter<'a, REG, MhrmatchWO>;
impl<'a, REG> MhrmatchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(MhrmatchWO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event SYNC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sync {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Sync> for bool {
    #[inline(always)]
    fn from(variant: Sync) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNC` reader - Write '1' to disable interrupt for event SYNC"]
pub type SyncR = crate::BitReader<Sync>;
impl SyncR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sync {
        match self.bits {
            false => Sync::Disabled,
            true => Sync::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Sync::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Sync::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event SYNC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SyncWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<SyncWO> for bool {
    #[inline(always)]
    fn from(variant: SyncWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNC` writer - Write '1' to disable interrupt for event SYNC"]
pub type SyncW<'a, REG> = crate::BitWriter<'a, REG, SyncWO>;
impl<'a, REG> SyncW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(SyncWO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event PHYEND\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phyend {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Phyend> for bool {
    #[inline(always)]
    fn from(variant: Phyend) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PHYEND` reader - Write '1' to disable interrupt for event PHYEND"]
pub type PhyendR = crate::BitReader<Phyend>;
impl PhyendR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Phyend {
        match self.bits {
            false => Phyend::Disabled,
            true => Phyend::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Phyend::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Phyend::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event PHYEND\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PhyendWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<PhyendWO> for bool {
    #[inline(always)]
    fn from(variant: PhyendWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PHYEND` writer - Write '1' to disable interrupt for event PHYEND"]
pub type PhyendW<'a, REG> = crate::BitWriter<'a, REG, PhyendWO>;
impl<'a, REG> PhyendW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(PhyendWO::Clear)
    }
}
impl R {
    #[doc = "Bit 0 - Write '1' to disable interrupt for event READY"]
    #[inline(always)]
    pub fn ready(&self) -> ReadyR {
        ReadyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write '1' to disable interrupt for event ADDRESS"]
    #[inline(always)]
    pub fn address(&self) -> AddressR {
        AddressR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write '1' to disable interrupt for event PAYLOAD"]
    #[inline(always)]
    pub fn payload(&self) -> PayloadR {
        PayloadR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write '1' to disable interrupt for event END"]
    #[inline(always)]
    pub fn end(&self) -> EndR {
        EndR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write '1' to disable interrupt for event DISABLED"]
    #[inline(always)]
    pub fn disabled(&self) -> DisabledR {
        DisabledR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write '1' to disable interrupt for event DEVMATCH"]
    #[inline(always)]
    pub fn devmatch(&self) -> DevmatchR {
        DevmatchR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write '1' to disable interrupt for event DEVMISS"]
    #[inline(always)]
    pub fn devmiss(&self) -> DevmissR {
        DevmissR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write '1' to disable interrupt for event RSSIEND"]
    #[inline(always)]
    pub fn rssiend(&self) -> RssiendR {
        RssiendR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - Write '1' to disable interrupt for event BCMATCH"]
    #[inline(always)]
    pub fn bcmatch(&self) -> BcmatchR {
        BcmatchR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Write '1' to disable interrupt for event CRCOK"]
    #[inline(always)]
    pub fn crcok(&self) -> CrcokR {
        CrcokR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Write '1' to disable interrupt for event CRCERROR"]
    #[inline(always)]
    pub fn crcerror(&self) -> CrcerrorR {
        CrcerrorR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Write '1' to disable interrupt for event FRAMESTART"]
    #[inline(always)]
    pub fn framestart(&self) -> FramestartR {
        FramestartR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Write '1' to disable interrupt for event EDEND"]
    #[inline(always)]
    pub fn edend(&self) -> EdendR {
        EdendR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Write '1' to disable interrupt for event EDSTOPPED"]
    #[inline(always)]
    pub fn edstopped(&self) -> EdstoppedR {
        EdstoppedR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Write '1' to disable interrupt for event CCAIDLE"]
    #[inline(always)]
    pub fn ccaidle(&self) -> CcaidleR {
        CcaidleR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Write '1' to disable interrupt for event CCABUSY"]
    #[inline(always)]
    pub fn ccabusy(&self) -> CcabusyR {
        CcabusyR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Write '1' to disable interrupt for event CCASTOPPED"]
    #[inline(always)]
    pub fn ccastopped(&self) -> CcastoppedR {
        CcastoppedR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Write '1' to disable interrupt for event RATEBOOST"]
    #[inline(always)]
    pub fn rateboost(&self) -> RateboostR {
        RateboostR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Write '1' to disable interrupt for event TXREADY"]
    #[inline(always)]
    pub fn txready(&self) -> TxreadyR {
        TxreadyR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Write '1' to disable interrupt for event RXREADY"]
    #[inline(always)]
    pub fn rxready(&self) -> RxreadyR {
        RxreadyR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Write '1' to disable interrupt for event MHRMATCH"]
    #[inline(always)]
    pub fn mhrmatch(&self) -> MhrmatchR {
        MhrmatchR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 26 - Write '1' to disable interrupt for event SYNC"]
    #[inline(always)]
    pub fn sync(&self) -> SyncR {
        SyncR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Write '1' to disable interrupt for event PHYEND"]
    #[inline(always)]
    pub fn phyend(&self) -> PhyendR {
        PhyendR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to disable interrupt for event READY"]
    #[inline(always)]
    pub fn ready(&mut self) -> ReadyW<'_, IntenclrSpec> {
        ReadyW::new(self, 0)
    }
    #[doc = "Bit 1 - Write '1' to disable interrupt for event ADDRESS"]
    #[inline(always)]
    pub fn address(&mut self) -> AddressW<'_, IntenclrSpec> {
        AddressW::new(self, 1)
    }
    #[doc = "Bit 2 - Write '1' to disable interrupt for event PAYLOAD"]
    #[inline(always)]
    pub fn payload(&mut self) -> PayloadW<'_, IntenclrSpec> {
        PayloadW::new(self, 2)
    }
    #[doc = "Bit 3 - Write '1' to disable interrupt for event END"]
    #[inline(always)]
    pub fn end(&mut self) -> EndW<'_, IntenclrSpec> {
        EndW::new(self, 3)
    }
    #[doc = "Bit 4 - Write '1' to disable interrupt for event DISABLED"]
    #[inline(always)]
    pub fn disabled(&mut self) -> DisabledW<'_, IntenclrSpec> {
        DisabledW::new(self, 4)
    }
    #[doc = "Bit 5 - Write '1' to disable interrupt for event DEVMATCH"]
    #[inline(always)]
    pub fn devmatch(&mut self) -> DevmatchW<'_, IntenclrSpec> {
        DevmatchW::new(self, 5)
    }
    #[doc = "Bit 6 - Write '1' to disable interrupt for event DEVMISS"]
    #[inline(always)]
    pub fn devmiss(&mut self) -> DevmissW<'_, IntenclrSpec> {
        DevmissW::new(self, 6)
    }
    #[doc = "Bit 7 - Write '1' to disable interrupt for event RSSIEND"]
    #[inline(always)]
    pub fn rssiend(&mut self) -> RssiendW<'_, IntenclrSpec> {
        RssiendW::new(self, 7)
    }
    #[doc = "Bit 10 - Write '1' to disable interrupt for event BCMATCH"]
    #[inline(always)]
    pub fn bcmatch(&mut self) -> BcmatchW<'_, IntenclrSpec> {
        BcmatchW::new(self, 10)
    }
    #[doc = "Bit 12 - Write '1' to disable interrupt for event CRCOK"]
    #[inline(always)]
    pub fn crcok(&mut self) -> CrcokW<'_, IntenclrSpec> {
        CrcokW::new(self, 12)
    }
    #[doc = "Bit 13 - Write '1' to disable interrupt for event CRCERROR"]
    #[inline(always)]
    pub fn crcerror(&mut self) -> CrcerrorW<'_, IntenclrSpec> {
        CrcerrorW::new(self, 13)
    }
    #[doc = "Bit 14 - Write '1' to disable interrupt for event FRAMESTART"]
    #[inline(always)]
    pub fn framestart(&mut self) -> FramestartW<'_, IntenclrSpec> {
        FramestartW::new(self, 14)
    }
    #[doc = "Bit 15 - Write '1' to disable interrupt for event EDEND"]
    #[inline(always)]
    pub fn edend(&mut self) -> EdendW<'_, IntenclrSpec> {
        EdendW::new(self, 15)
    }
    #[doc = "Bit 16 - Write '1' to disable interrupt for event EDSTOPPED"]
    #[inline(always)]
    pub fn edstopped(&mut self) -> EdstoppedW<'_, IntenclrSpec> {
        EdstoppedW::new(self, 16)
    }
    #[doc = "Bit 17 - Write '1' to disable interrupt for event CCAIDLE"]
    #[inline(always)]
    pub fn ccaidle(&mut self) -> CcaidleW<'_, IntenclrSpec> {
        CcaidleW::new(self, 17)
    }
    #[doc = "Bit 18 - Write '1' to disable interrupt for event CCABUSY"]
    #[inline(always)]
    pub fn ccabusy(&mut self) -> CcabusyW<'_, IntenclrSpec> {
        CcabusyW::new(self, 18)
    }
    #[doc = "Bit 19 - Write '1' to disable interrupt for event CCASTOPPED"]
    #[inline(always)]
    pub fn ccastopped(&mut self) -> CcastoppedW<'_, IntenclrSpec> {
        CcastoppedW::new(self, 19)
    }
    #[doc = "Bit 20 - Write '1' to disable interrupt for event RATEBOOST"]
    #[inline(always)]
    pub fn rateboost(&mut self) -> RateboostW<'_, IntenclrSpec> {
        RateboostW::new(self, 20)
    }
    #[doc = "Bit 21 - Write '1' to disable interrupt for event TXREADY"]
    #[inline(always)]
    pub fn txready(&mut self) -> TxreadyW<'_, IntenclrSpec> {
        TxreadyW::new(self, 21)
    }
    #[doc = "Bit 22 - Write '1' to disable interrupt for event RXREADY"]
    #[inline(always)]
    pub fn rxready(&mut self) -> RxreadyW<'_, IntenclrSpec> {
        RxreadyW::new(self, 22)
    }
    #[doc = "Bit 23 - Write '1' to disable interrupt for event MHRMATCH"]
    #[inline(always)]
    pub fn mhrmatch(&mut self) -> MhrmatchW<'_, IntenclrSpec> {
        MhrmatchW::new(self, 23)
    }
    #[doc = "Bit 26 - Write '1' to disable interrupt for event SYNC"]
    #[inline(always)]
    pub fn sync(&mut self) -> SyncW<'_, IntenclrSpec> {
        SyncW::new(self, 26)
    }
    #[doc = "Bit 27 - Write '1' to disable interrupt for event PHYEND"]
    #[inline(always)]
    pub fn phyend(&mut self) -> PhyendW<'_, IntenclrSpec> {
        PhyendW::new(self, 27)
    }
}
#[doc = "Disable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenclrSpec;
impl crate::RegisterSpec for IntenclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenclr::R`](R) reader structure"]
impl crate::Readable for IntenclrSpec {}
#[doc = "`write(|w| ..)` method takes [`intenclr::W`](W) writer structure"]
impl crate::Writable for IntenclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for IntenclrSpec {}
