#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<IntenclrSpec>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
#[doc = "Disable interrupt on READY event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ready {
    #[doc = "0: Interrupt disabled."]
    Disabled = 0,
    #[doc = "1: Interrupt enabled."]
    Enabled = 1,
}
impl From<Ready> for bool {
    #[inline(always)]
    fn from(variant: Ready) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READY` reader - Disable interrupt on READY event."]
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
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ready::Disabled
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ready::Enabled
    }
}
#[doc = "Disable interrupt on READY event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReadyWO {
    #[doc = "1: Disable interrupt on write."]
    Clear = 1,
}
impl From<ReadyWO> for bool {
    #[inline(always)]
    fn from(variant: ReadyWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READY` writer - Disable interrupt on READY event."]
pub type ReadyW<'a, REG> = crate::BitWriter<'a, REG, ReadyWO>;
impl<'a, REG> ReadyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable interrupt on write."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ReadyWO::Clear)
    }
}
#[doc = "Disable interrupt on ADDRESS event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Address {
    #[doc = "0: Interrupt disabled."]
    Disabled = 0,
    #[doc = "1: Interrupt enabled."]
    Enabled = 1,
}
impl From<Address> for bool {
    #[inline(always)]
    fn from(variant: Address) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDRESS` reader - Disable interrupt on ADDRESS event."]
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
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Address::Disabled
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Address::Enabled
    }
}
#[doc = "Disable interrupt on ADDRESS event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AddressWO {
    #[doc = "1: Disable interrupt on write."]
    Clear = 1,
}
impl From<AddressWO> for bool {
    #[inline(always)]
    fn from(variant: AddressWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDRESS` writer - Disable interrupt on ADDRESS event."]
pub type AddressW<'a, REG> = crate::BitWriter<'a, REG, AddressWO>;
impl<'a, REG> AddressW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable interrupt on write."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(AddressWO::Clear)
    }
}
#[doc = "Disable interrupt on PAYLOAD event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Payload {
    #[doc = "0: Interrupt disabled."]
    Disabled = 0,
    #[doc = "1: Interrupt enabled."]
    Enabled = 1,
}
impl From<Payload> for bool {
    #[inline(always)]
    fn from(variant: Payload) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAYLOAD` reader - Disable interrupt on PAYLOAD event."]
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
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Payload::Disabled
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Payload::Enabled
    }
}
#[doc = "Disable interrupt on PAYLOAD event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PayloadWO {
    #[doc = "1: Disable interrupt on write."]
    Clear = 1,
}
impl From<PayloadWO> for bool {
    #[inline(always)]
    fn from(variant: PayloadWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAYLOAD` writer - Disable interrupt on PAYLOAD event."]
pub type PayloadW<'a, REG> = crate::BitWriter<'a, REG, PayloadWO>;
impl<'a, REG> PayloadW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable interrupt on write."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(PayloadWO::Clear)
    }
}
#[doc = "Disable interrupt on END event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum End {
    #[doc = "0: Interrupt disabled."]
    Disabled = 0,
    #[doc = "1: Interrupt enabled."]
    Enabled = 1,
}
impl From<End> for bool {
    #[inline(always)]
    fn from(variant: End) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `END` reader - Disable interrupt on END event."]
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
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == End::Disabled
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == End::Enabled
    }
}
#[doc = "Disable interrupt on END event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EndWO {
    #[doc = "1: Disable interrupt on write."]
    Clear = 1,
}
impl From<EndWO> for bool {
    #[inline(always)]
    fn from(variant: EndWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `END` writer - Disable interrupt on END event."]
pub type EndW<'a, REG> = crate::BitWriter<'a, REG, EndWO>;
impl<'a, REG> EndW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable interrupt on write."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(EndWO::Clear)
    }
}
#[doc = "Disable interrupt on DISABLED event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Disabled {
    #[doc = "0: Interrupt disabled."]
    Disabled = 0,
    #[doc = "1: Interrupt enabled."]
    Enabled = 1,
}
impl From<Disabled> for bool {
    #[inline(always)]
    fn from(variant: Disabled) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISABLED` reader - Disable interrupt on DISABLED event."]
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
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Disabled::Disabled
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Disabled::Enabled
    }
}
#[doc = "Disable interrupt on DISABLED event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DisabledWO {
    #[doc = "1: Disable interrupt on write."]
    Clear = 1,
}
impl From<DisabledWO> for bool {
    #[inline(always)]
    fn from(variant: DisabledWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISABLED` writer - Disable interrupt on DISABLED event."]
pub type DisabledW<'a, REG> = crate::BitWriter<'a, REG, DisabledWO>;
impl<'a, REG> DisabledW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable interrupt on write."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(DisabledWO::Clear)
    }
}
#[doc = "Disable interrupt on DEVMATCH event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Devmatch {
    #[doc = "0: Interrupt disabled."]
    Disabled = 0,
    #[doc = "1: Interrupt enabled."]
    Enabled = 1,
}
impl From<Devmatch> for bool {
    #[inline(always)]
    fn from(variant: Devmatch) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEVMATCH` reader - Disable interrupt on DEVMATCH event."]
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
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Devmatch::Disabled
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Devmatch::Enabled
    }
}
#[doc = "Disable interrupt on DEVMATCH event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DevmatchWO {
    #[doc = "1: Disable interrupt on write."]
    Clear = 1,
}
impl From<DevmatchWO> for bool {
    #[inline(always)]
    fn from(variant: DevmatchWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEVMATCH` writer - Disable interrupt on DEVMATCH event."]
pub type DevmatchW<'a, REG> = crate::BitWriter<'a, REG, DevmatchWO>;
impl<'a, REG> DevmatchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable interrupt on write."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(DevmatchWO::Clear)
    }
}
#[doc = "Disable interrupt on DEVMISS event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Devmiss {
    #[doc = "0: Interrupt disabled."]
    Disabled = 0,
    #[doc = "1: Interrupt enabled."]
    Enabled = 1,
}
impl From<Devmiss> for bool {
    #[inline(always)]
    fn from(variant: Devmiss) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEVMISS` reader - Disable interrupt on DEVMISS event."]
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
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Devmiss::Disabled
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Devmiss::Enabled
    }
}
#[doc = "Disable interrupt on DEVMISS event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DevmissWO {
    #[doc = "1: Disable interrupt on write."]
    Clear = 1,
}
impl From<DevmissWO> for bool {
    #[inline(always)]
    fn from(variant: DevmissWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEVMISS` writer - Disable interrupt on DEVMISS event."]
pub type DevmissW<'a, REG> = crate::BitWriter<'a, REG, DevmissWO>;
impl<'a, REG> DevmissW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable interrupt on write."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(DevmissWO::Clear)
    }
}
#[doc = "Disable interrupt on RSSIEND event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rssiend {
    #[doc = "0: Interrupt disabled."]
    Disabled = 0,
    #[doc = "1: Interrupt enabled."]
    Enabled = 1,
}
impl From<Rssiend> for bool {
    #[inline(always)]
    fn from(variant: Rssiend) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSSIEND` reader - Disable interrupt on RSSIEND event."]
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
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rssiend::Disabled
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rssiend::Enabled
    }
}
#[doc = "Disable interrupt on RSSIEND event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RssiendWO {
    #[doc = "1: Disable interrupt on write."]
    Clear = 1,
}
impl From<RssiendWO> for bool {
    #[inline(always)]
    fn from(variant: RssiendWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSSIEND` writer - Disable interrupt on RSSIEND event."]
pub type RssiendW<'a, REG> = crate::BitWriter<'a, REG, RssiendWO>;
impl<'a, REG> RssiendW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable interrupt on write."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(RssiendWO::Clear)
    }
}
#[doc = "Disable interrupt on BCMATCH event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bcmatch {
    #[doc = "0: Interrupt disabled."]
    Disabled = 0,
    #[doc = "1: Interrupt enabled."]
    Enabled = 1,
}
impl From<Bcmatch> for bool {
    #[inline(always)]
    fn from(variant: Bcmatch) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BCMATCH` reader - Disable interrupt on BCMATCH event."]
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
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Bcmatch::Disabled
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Bcmatch::Enabled
    }
}
#[doc = "Disable interrupt on BCMATCH event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BcmatchWO {
    #[doc = "1: Disable interrupt on write."]
    Clear = 1,
}
impl From<BcmatchWO> for bool {
    #[inline(always)]
    fn from(variant: BcmatchWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BCMATCH` writer - Disable interrupt on BCMATCH event."]
pub type BcmatchW<'a, REG> = crate::BitWriter<'a, REG, BcmatchWO>;
impl<'a, REG> BcmatchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable interrupt on write."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(BcmatchWO::Clear)
    }
}
impl R {
    #[doc = "Bit 0 - Disable interrupt on READY event."]
    #[inline(always)]
    pub fn ready(&self) -> ReadyR {
        ReadyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Disable interrupt on ADDRESS event."]
    #[inline(always)]
    pub fn address(&self) -> AddressR {
        AddressR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Disable interrupt on PAYLOAD event."]
    #[inline(always)]
    pub fn payload(&self) -> PayloadR {
        PayloadR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Disable interrupt on END event."]
    #[inline(always)]
    pub fn end(&self) -> EndR {
        EndR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Disable interrupt on DISABLED event."]
    #[inline(always)]
    pub fn disabled(&self) -> DisabledR {
        DisabledR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Disable interrupt on DEVMATCH event."]
    #[inline(always)]
    pub fn devmatch(&self) -> DevmatchR {
        DevmatchR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Disable interrupt on DEVMISS event."]
    #[inline(always)]
    pub fn devmiss(&self) -> DevmissR {
        DevmissR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Disable interrupt on RSSIEND event."]
    #[inline(always)]
    pub fn rssiend(&self) -> RssiendR {
        RssiendR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - Disable interrupt on BCMATCH event."]
    #[inline(always)]
    pub fn bcmatch(&self) -> BcmatchR {
        BcmatchR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Disable interrupt on READY event."]
    #[inline(always)]
    pub fn ready(&mut self) -> ReadyW<'_, IntenclrSpec> {
        ReadyW::new(self, 0)
    }
    #[doc = "Bit 1 - Disable interrupt on ADDRESS event."]
    #[inline(always)]
    pub fn address(&mut self) -> AddressW<'_, IntenclrSpec> {
        AddressW::new(self, 1)
    }
    #[doc = "Bit 2 - Disable interrupt on PAYLOAD event."]
    #[inline(always)]
    pub fn payload(&mut self) -> PayloadW<'_, IntenclrSpec> {
        PayloadW::new(self, 2)
    }
    #[doc = "Bit 3 - Disable interrupt on END event."]
    #[inline(always)]
    pub fn end(&mut self) -> EndW<'_, IntenclrSpec> {
        EndW::new(self, 3)
    }
    #[doc = "Bit 4 - Disable interrupt on DISABLED event."]
    #[inline(always)]
    pub fn disabled(&mut self) -> DisabledW<'_, IntenclrSpec> {
        DisabledW::new(self, 4)
    }
    #[doc = "Bit 5 - Disable interrupt on DEVMATCH event."]
    #[inline(always)]
    pub fn devmatch(&mut self) -> DevmatchW<'_, IntenclrSpec> {
        DevmatchW::new(self, 5)
    }
    #[doc = "Bit 6 - Disable interrupt on DEVMISS event."]
    #[inline(always)]
    pub fn devmiss(&mut self) -> DevmissW<'_, IntenclrSpec> {
        DevmissW::new(self, 6)
    }
    #[doc = "Bit 7 - Disable interrupt on RSSIEND event."]
    #[inline(always)]
    pub fn rssiend(&mut self) -> RssiendW<'_, IntenclrSpec> {
        RssiendW::new(self, 7)
    }
    #[doc = "Bit 10 - Disable interrupt on BCMATCH event."]
    #[inline(always)]
    pub fn bcmatch(&mut self) -> BcmatchW<'_, IntenclrSpec> {
        BcmatchW::new(self, 10)
    }
}
#[doc = "Interrupt enable clear register.\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
