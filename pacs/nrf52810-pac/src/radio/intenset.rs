#[doc = "Register `INTENSET` reader"]
pub type R = crate::R<IntensetSpec>;
#[doc = "Register `INTENSET` writer"]
pub type W = crate::W<IntensetSpec>;
#[doc = "Write '1' to enable interrupt for event READY\n\nValue on reset: 0"]
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
#[doc = "Field `READY` reader - Write '1' to enable interrupt for event READY"]
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
#[doc = "Write '1' to enable interrupt for event READY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReadyWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<ReadyWO> for bool {
    #[inline(always)]
    fn from(variant: ReadyWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READY` writer - Write '1' to enable interrupt for event READY"]
pub type ReadyW<'a, REG> = crate::BitWriter<'a, REG, ReadyWO>;
impl<'a, REG> ReadyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(ReadyWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event ADDRESS\n\nValue on reset: 0"]
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
#[doc = "Field `ADDRESS` reader - Write '1' to enable interrupt for event ADDRESS"]
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
#[doc = "Write '1' to enable interrupt for event ADDRESS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AddressWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<AddressWO> for bool {
    #[inline(always)]
    fn from(variant: AddressWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDRESS` writer - Write '1' to enable interrupt for event ADDRESS"]
pub type AddressW<'a, REG> = crate::BitWriter<'a, REG, AddressWO>;
impl<'a, REG> AddressW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(AddressWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event PAYLOAD\n\nValue on reset: 0"]
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
#[doc = "Field `PAYLOAD` reader - Write '1' to enable interrupt for event PAYLOAD"]
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
#[doc = "Write '1' to enable interrupt for event PAYLOAD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PayloadWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<PayloadWO> for bool {
    #[inline(always)]
    fn from(variant: PayloadWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAYLOAD` writer - Write '1' to enable interrupt for event PAYLOAD"]
pub type PayloadW<'a, REG> = crate::BitWriter<'a, REG, PayloadWO>;
impl<'a, REG> PayloadW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(PayloadWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event END\n\nValue on reset: 0"]
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
#[doc = "Field `END` reader - Write '1' to enable interrupt for event END"]
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
#[doc = "Write '1' to enable interrupt for event END\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EndWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<EndWO> for bool {
    #[inline(always)]
    fn from(variant: EndWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `END` writer - Write '1' to enable interrupt for event END"]
pub type EndW<'a, REG> = crate::BitWriter<'a, REG, EndWO>;
impl<'a, REG> EndW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(EndWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event DISABLED\n\nValue on reset: 0"]
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
#[doc = "Field `DISABLED` reader - Write '1' to enable interrupt for event DISABLED"]
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
#[doc = "Write '1' to enable interrupt for event DISABLED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DisabledWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<DisabledWO> for bool {
    #[inline(always)]
    fn from(variant: DisabledWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISABLED` writer - Write '1' to enable interrupt for event DISABLED"]
pub type DisabledW<'a, REG> = crate::BitWriter<'a, REG, DisabledWO>;
impl<'a, REG> DisabledW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(DisabledWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event DEVMATCH\n\nValue on reset: 0"]
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
#[doc = "Field `DEVMATCH` reader - Write '1' to enable interrupt for event DEVMATCH"]
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
#[doc = "Write '1' to enable interrupt for event DEVMATCH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DevmatchWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<DevmatchWO> for bool {
    #[inline(always)]
    fn from(variant: DevmatchWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEVMATCH` writer - Write '1' to enable interrupt for event DEVMATCH"]
pub type DevmatchW<'a, REG> = crate::BitWriter<'a, REG, DevmatchWO>;
impl<'a, REG> DevmatchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(DevmatchWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event DEVMISS\n\nValue on reset: 0"]
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
#[doc = "Field `DEVMISS` reader - Write '1' to enable interrupt for event DEVMISS"]
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
#[doc = "Write '1' to enable interrupt for event DEVMISS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DevmissWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<DevmissWO> for bool {
    #[inline(always)]
    fn from(variant: DevmissWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEVMISS` writer - Write '1' to enable interrupt for event DEVMISS"]
pub type DevmissW<'a, REG> = crate::BitWriter<'a, REG, DevmissWO>;
impl<'a, REG> DevmissW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(DevmissWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event RSSIEND\n\nValue on reset: 0"]
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
#[doc = "Field `RSSIEND` reader - Write '1' to enable interrupt for event RSSIEND"]
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
#[doc = "Write '1' to enable interrupt for event RSSIEND\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RssiendWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<RssiendWO> for bool {
    #[inline(always)]
    fn from(variant: RssiendWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSSIEND` writer - Write '1' to enable interrupt for event RSSIEND"]
pub type RssiendW<'a, REG> = crate::BitWriter<'a, REG, RssiendWO>;
impl<'a, REG> RssiendW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(RssiendWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event BCMATCH\n\nValue on reset: 0"]
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
#[doc = "Field `BCMATCH` reader - Write '1' to enable interrupt for event BCMATCH"]
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
#[doc = "Write '1' to enable interrupt for event BCMATCH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BcmatchWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<BcmatchWO> for bool {
    #[inline(always)]
    fn from(variant: BcmatchWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BCMATCH` writer - Write '1' to enable interrupt for event BCMATCH"]
pub type BcmatchW<'a, REG> = crate::BitWriter<'a, REG, BcmatchWO>;
impl<'a, REG> BcmatchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(BcmatchWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event CRCOK\n\nValue on reset: 0"]
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
#[doc = "Field `CRCOK` reader - Write '1' to enable interrupt for event CRCOK"]
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
#[doc = "Write '1' to enable interrupt for event CRCOK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CrcokWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<CrcokWO> for bool {
    #[inline(always)]
    fn from(variant: CrcokWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCOK` writer - Write '1' to enable interrupt for event CRCOK"]
pub type CrcokW<'a, REG> = crate::BitWriter<'a, REG, CrcokWO>;
impl<'a, REG> CrcokW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(CrcokWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event CRCERROR\n\nValue on reset: 0"]
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
#[doc = "Field `CRCERROR` reader - Write '1' to enable interrupt for event CRCERROR"]
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
#[doc = "Write '1' to enable interrupt for event CRCERROR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CrcerrorWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<CrcerrorWO> for bool {
    #[inline(always)]
    fn from(variant: CrcerrorWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCERROR` writer - Write '1' to enable interrupt for event CRCERROR"]
pub type CrcerrorW<'a, REG> = crate::BitWriter<'a, REG, CrcerrorWO>;
impl<'a, REG> CrcerrorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(CrcerrorWO::Set)
    }
}
impl R {
    #[doc = "Bit 0 - Write '1' to enable interrupt for event READY"]
    #[inline(always)]
    pub fn ready(&self) -> ReadyR {
        ReadyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write '1' to enable interrupt for event ADDRESS"]
    #[inline(always)]
    pub fn address(&self) -> AddressR {
        AddressR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write '1' to enable interrupt for event PAYLOAD"]
    #[inline(always)]
    pub fn payload(&self) -> PayloadR {
        PayloadR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write '1' to enable interrupt for event END"]
    #[inline(always)]
    pub fn end(&self) -> EndR {
        EndR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write '1' to enable interrupt for event DISABLED"]
    #[inline(always)]
    pub fn disabled(&self) -> DisabledR {
        DisabledR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write '1' to enable interrupt for event DEVMATCH"]
    #[inline(always)]
    pub fn devmatch(&self) -> DevmatchR {
        DevmatchR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write '1' to enable interrupt for event DEVMISS"]
    #[inline(always)]
    pub fn devmiss(&self) -> DevmissR {
        DevmissR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write '1' to enable interrupt for event RSSIEND"]
    #[inline(always)]
    pub fn rssiend(&self) -> RssiendR {
        RssiendR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - Write '1' to enable interrupt for event BCMATCH"]
    #[inline(always)]
    pub fn bcmatch(&self) -> BcmatchR {
        BcmatchR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Write '1' to enable interrupt for event CRCOK"]
    #[inline(always)]
    pub fn crcok(&self) -> CrcokR {
        CrcokR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Write '1' to enable interrupt for event CRCERROR"]
    #[inline(always)]
    pub fn crcerror(&self) -> CrcerrorR {
        CrcerrorR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to enable interrupt for event READY"]
    #[inline(always)]
    pub fn ready(&mut self) -> ReadyW<'_, IntensetSpec> {
        ReadyW::new(self, 0)
    }
    #[doc = "Bit 1 - Write '1' to enable interrupt for event ADDRESS"]
    #[inline(always)]
    pub fn address(&mut self) -> AddressW<'_, IntensetSpec> {
        AddressW::new(self, 1)
    }
    #[doc = "Bit 2 - Write '1' to enable interrupt for event PAYLOAD"]
    #[inline(always)]
    pub fn payload(&mut self) -> PayloadW<'_, IntensetSpec> {
        PayloadW::new(self, 2)
    }
    #[doc = "Bit 3 - Write '1' to enable interrupt for event END"]
    #[inline(always)]
    pub fn end(&mut self) -> EndW<'_, IntensetSpec> {
        EndW::new(self, 3)
    }
    #[doc = "Bit 4 - Write '1' to enable interrupt for event DISABLED"]
    #[inline(always)]
    pub fn disabled(&mut self) -> DisabledW<'_, IntensetSpec> {
        DisabledW::new(self, 4)
    }
    #[doc = "Bit 5 - Write '1' to enable interrupt for event DEVMATCH"]
    #[inline(always)]
    pub fn devmatch(&mut self) -> DevmatchW<'_, IntensetSpec> {
        DevmatchW::new(self, 5)
    }
    #[doc = "Bit 6 - Write '1' to enable interrupt for event DEVMISS"]
    #[inline(always)]
    pub fn devmiss(&mut self) -> DevmissW<'_, IntensetSpec> {
        DevmissW::new(self, 6)
    }
    #[doc = "Bit 7 - Write '1' to enable interrupt for event RSSIEND"]
    #[inline(always)]
    pub fn rssiend(&mut self) -> RssiendW<'_, IntensetSpec> {
        RssiendW::new(self, 7)
    }
    #[doc = "Bit 10 - Write '1' to enable interrupt for event BCMATCH"]
    #[inline(always)]
    pub fn bcmatch(&mut self) -> BcmatchW<'_, IntensetSpec> {
        BcmatchW::new(self, 10)
    }
    #[doc = "Bit 12 - Write '1' to enable interrupt for event CRCOK"]
    #[inline(always)]
    pub fn crcok(&mut self) -> CrcokW<'_, IntensetSpec> {
        CrcokW::new(self, 12)
    }
    #[doc = "Bit 13 - Write '1' to enable interrupt for event CRCERROR"]
    #[inline(always)]
    pub fn crcerror(&mut self) -> CrcerrorW<'_, IntensetSpec> {
        CrcerrorW::new(self, 13)
    }
}
#[doc = "Enable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntensetSpec;
impl crate::RegisterSpec for IntensetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenset::R`](R) reader structure"]
impl crate::Readable for IntensetSpec {}
#[doc = "`write(|w| ..)` method takes [`intenset::W`](W) writer structure"]
impl crate::Writable for IntensetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for IntensetSpec {}
