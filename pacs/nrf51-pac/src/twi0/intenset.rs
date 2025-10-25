#[doc = "Register `INTENSET` reader"]
pub type R = crate::R<IntensetSpec>;
#[doc = "Register `INTENSET` writer"]
pub type W = crate::W<IntensetSpec>;
#[doc = "Enable interrupt on STOPPED event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stopped {
    #[doc = "0: Interrupt disabled."]
    Disabled = 0,
    #[doc = "1: Interrupt enabled."]
    Enabled = 1,
}
impl From<Stopped> for bool {
    #[inline(always)]
    fn from(variant: Stopped) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPPED` reader - Enable interrupt on STOPPED event."]
pub type StoppedR = crate::BitReader<Stopped>;
impl StoppedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stopped {
        match self.bits {
            false => Stopped::Disabled,
            true => Stopped::Enabled,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Stopped::Disabled
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Stopped::Enabled
    }
}
#[doc = "Enable interrupt on STOPPED event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StoppedWO {
    #[doc = "1: Enable interrupt on write."]
    Set = 1,
}
impl From<StoppedWO> for bool {
    #[inline(always)]
    fn from(variant: StoppedWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPPED` writer - Enable interrupt on STOPPED event."]
pub type StoppedW<'a, REG> = crate::BitWriter<'a, REG, StoppedWO>;
impl<'a, REG> StoppedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt on write."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(StoppedWO::Set)
    }
}
#[doc = "Enable interrupt on READY event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxdready {
    #[doc = "0: Interrupt disabled."]
    Disabled = 0,
    #[doc = "1: Interrupt enabled."]
    Enabled = 1,
}
impl From<Rxdready> for bool {
    #[inline(always)]
    fn from(variant: Rxdready) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXDREADY` reader - Enable interrupt on READY event."]
pub type RxdreadyR = crate::BitReader<Rxdready>;
impl RxdreadyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxdready {
        match self.bits {
            false => Rxdready::Disabled,
            true => Rxdready::Enabled,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rxdready::Disabled
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rxdready::Enabled
    }
}
#[doc = "Enable interrupt on READY event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RxdreadyWO {
    #[doc = "1: Enable interrupt on write."]
    Set = 1,
}
impl From<RxdreadyWO> for bool {
    #[inline(always)]
    fn from(variant: RxdreadyWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXDREADY` writer - Enable interrupt on READY event."]
pub type RxdreadyW<'a, REG> = crate::BitWriter<'a, REG, RxdreadyWO>;
impl<'a, REG> RxdreadyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt on write."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(RxdreadyWO::Set)
    }
}
#[doc = "Enable interrupt on TXDSENT event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txdsent {
    #[doc = "0: Interrupt disabled."]
    Disabled = 0,
    #[doc = "1: Interrupt enabled."]
    Enabled = 1,
}
impl From<Txdsent> for bool {
    #[inline(always)]
    fn from(variant: Txdsent) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXDSENT` reader - Enable interrupt on TXDSENT event."]
pub type TxdsentR = crate::BitReader<Txdsent>;
impl TxdsentR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txdsent {
        match self.bits {
            false => Txdsent::Disabled,
            true => Txdsent::Enabled,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Txdsent::Disabled
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Txdsent::Enabled
    }
}
#[doc = "Enable interrupt on TXDSENT event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxdsentWO {
    #[doc = "1: Enable interrupt on write."]
    Set = 1,
}
impl From<TxdsentWO> for bool {
    #[inline(always)]
    fn from(variant: TxdsentWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXDSENT` writer - Enable interrupt on TXDSENT event."]
pub type TxdsentW<'a, REG> = crate::BitWriter<'a, REG, TxdsentWO>;
impl<'a, REG> TxdsentW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt on write."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(TxdsentWO::Set)
    }
}
#[doc = "Enable interrupt on ERROR event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    #[doc = "0: Interrupt disabled."]
    Disabled = 0,
    #[doc = "1: Interrupt enabled."]
    Enabled = 1,
}
impl From<Error> for bool {
    #[inline(always)]
    fn from(variant: Error) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERROR` reader - Enable interrupt on ERROR event."]
pub type ErrorR = crate::BitReader<Error>;
impl ErrorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Error {
        match self.bits {
            false => Error::Disabled,
            true => Error::Enabled,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Error::Disabled
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Error::Enabled
    }
}
#[doc = "Enable interrupt on ERROR event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrorWO {
    #[doc = "1: Enable interrupt on write."]
    Set = 1,
}
impl From<ErrorWO> for bool {
    #[inline(always)]
    fn from(variant: ErrorWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERROR` writer - Enable interrupt on ERROR event."]
pub type ErrorW<'a, REG> = crate::BitWriter<'a, REG, ErrorWO>;
impl<'a, REG> ErrorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt on write."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(ErrorWO::Set)
    }
}
#[doc = "Enable interrupt on BB event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bb {
    #[doc = "0: Interrupt disabled."]
    Disabled = 0,
    #[doc = "1: Interrupt enabled."]
    Enabled = 1,
}
impl From<Bb> for bool {
    #[inline(always)]
    fn from(variant: Bb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BB` reader - Enable interrupt on BB event."]
pub type BbR = crate::BitReader<Bb>;
impl BbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bb {
        match self.bits {
            false => Bb::Disabled,
            true => Bb::Enabled,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Bb::Disabled
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Bb::Enabled
    }
}
#[doc = "Enable interrupt on BB event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BbWO {
    #[doc = "1: Enable interrupt on write."]
    Set = 1,
}
impl From<BbWO> for bool {
    #[inline(always)]
    fn from(variant: BbWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BB` writer - Enable interrupt on BB event."]
pub type BbW<'a, REG> = crate::BitWriter<'a, REG, BbWO>;
impl<'a, REG> BbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt on write."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(BbWO::Set)
    }
}
#[doc = "Enable interrupt on SUSPENDED event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Suspended {
    #[doc = "0: Interrupt disabled."]
    Disabled = 0,
    #[doc = "1: Interrupt enabled."]
    Enabled = 1,
}
impl From<Suspended> for bool {
    #[inline(always)]
    fn from(variant: Suspended) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUSPENDED` reader - Enable interrupt on SUSPENDED event."]
pub type SuspendedR = crate::BitReader<Suspended>;
impl SuspendedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Suspended {
        match self.bits {
            false => Suspended::Disabled,
            true => Suspended::Enabled,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Suspended::Disabled
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Suspended::Enabled
    }
}
#[doc = "Enable interrupt on SUSPENDED event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SuspendedWO {
    #[doc = "1: Enable interrupt on write."]
    Set = 1,
}
impl From<SuspendedWO> for bool {
    #[inline(always)]
    fn from(variant: SuspendedWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUSPENDED` writer - Enable interrupt on SUSPENDED event."]
pub type SuspendedW<'a, REG> = crate::BitWriter<'a, REG, SuspendedWO>;
impl<'a, REG> SuspendedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt on write."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(SuspendedWO::Set)
    }
}
impl R {
    #[doc = "Bit 1 - Enable interrupt on STOPPED event."]
    #[inline(always)]
    pub fn stopped(&self) -> StoppedR {
        StoppedR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable interrupt on READY event."]
    #[inline(always)]
    pub fn rxdready(&self) -> RxdreadyR {
        RxdreadyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable interrupt on TXDSENT event."]
    #[inline(always)]
    pub fn txdsent(&self) -> TxdsentR {
        TxdsentR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable interrupt on ERROR event."]
    #[inline(always)]
    pub fn error(&self) -> ErrorR {
        ErrorR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable interrupt on BB event."]
    #[inline(always)]
    pub fn bb(&self) -> BbR {
        BbR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable interrupt on SUSPENDED event."]
    #[inline(always)]
    pub fn suspended(&self) -> SuspendedR {
        SuspendedR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Enable interrupt on STOPPED event."]
    #[inline(always)]
    pub fn stopped(&mut self) -> StoppedW<'_, IntensetSpec> {
        StoppedW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable interrupt on READY event."]
    #[inline(always)]
    pub fn rxdready(&mut self) -> RxdreadyW<'_, IntensetSpec> {
        RxdreadyW::new(self, 2)
    }
    #[doc = "Bit 7 - Enable interrupt on TXDSENT event."]
    #[inline(always)]
    pub fn txdsent(&mut self) -> TxdsentW<'_, IntensetSpec> {
        TxdsentW::new(self, 7)
    }
    #[doc = "Bit 9 - Enable interrupt on ERROR event."]
    #[inline(always)]
    pub fn error(&mut self) -> ErrorW<'_, IntensetSpec> {
        ErrorW::new(self, 9)
    }
    #[doc = "Bit 14 - Enable interrupt on BB event."]
    #[inline(always)]
    pub fn bb(&mut self) -> BbW<'_, IntensetSpec> {
        BbW::new(self, 14)
    }
    #[doc = "Bit 18 - Enable interrupt on SUSPENDED event."]
    #[inline(always)]
    pub fn suspended(&mut self) -> SuspendedW<'_, IntensetSpec> {
        SuspendedW::new(self, 18)
    }
}
#[doc = "Interrupt enable set register.\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
