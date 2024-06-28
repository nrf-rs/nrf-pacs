#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<IntenclrSpec>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
#[doc = "Write '1' to disable interrupt for event STOPPED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stopped {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Stopped> for bool {
    #[inline(always)]
    fn from(variant: Stopped) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPPED` reader - Write '1' to disable interrupt for event STOPPED"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Stopped::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Stopped::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event STOPPED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StoppedWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<StoppedWO> for bool {
    #[inline(always)]
    fn from(variant: StoppedWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPPED` writer - Write '1' to disable interrupt for event STOPPED"]
pub type StoppedW<'a, REG> = crate::BitWriter<'a, REG, StoppedWO>;
impl<'a, REG> StoppedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(StoppedWO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event ERROR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Error> for bool {
    #[inline(always)]
    fn from(variant: Error) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERROR` reader - Write '1' to disable interrupt for event ERROR"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Error::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Error::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event ERROR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrorWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<ErrorWO> for bool {
    #[inline(always)]
    fn from(variant: ErrorWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERROR` writer - Write '1' to disable interrupt for event ERROR"]
pub type ErrorW<'a, REG> = crate::BitWriter<'a, REG, ErrorWO>;
impl<'a, REG> ErrorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ErrorWO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event RXSTARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxstarted {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Rxstarted> for bool {
    #[inline(always)]
    fn from(variant: Rxstarted) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXSTARTED` reader - Write '1' to disable interrupt for event RXSTARTED"]
pub type RxstartedR = crate::BitReader<Rxstarted>;
impl RxstartedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxstarted {
        match self.bits {
            false => Rxstarted::Disabled,
            true => Rxstarted::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rxstarted::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rxstarted::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event RXSTARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RxstartedWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<RxstartedWO> for bool {
    #[inline(always)]
    fn from(variant: RxstartedWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXSTARTED` writer - Write '1' to disable interrupt for event RXSTARTED"]
pub type RxstartedW<'a, REG> = crate::BitWriter<'a, REG, RxstartedWO>;
impl<'a, REG> RxstartedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(RxstartedWO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event TXSTARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txstarted {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Txstarted> for bool {
    #[inline(always)]
    fn from(variant: Txstarted) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXSTARTED` reader - Write '1' to disable interrupt for event TXSTARTED"]
pub type TxstartedR = crate::BitReader<Txstarted>;
impl TxstartedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txstarted {
        match self.bits {
            false => Txstarted::Disabled,
            true => Txstarted::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Txstarted::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Txstarted::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event TXSTARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxstartedWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<TxstartedWO> for bool {
    #[inline(always)]
    fn from(variant: TxstartedWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXSTARTED` writer - Write '1' to disable interrupt for event TXSTARTED"]
pub type TxstartedW<'a, REG> = crate::BitWriter<'a, REG, TxstartedWO>;
impl<'a, REG> TxstartedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(TxstartedWO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event WRITE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Write {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Write> for bool {
    #[inline(always)]
    fn from(variant: Write) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRITE` reader - Write '1' to disable interrupt for event WRITE"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Write::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Write::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event WRITE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WriteWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<WriteWO> for bool {
    #[inline(always)]
    fn from(variant: WriteWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRITE` writer - Write '1' to disable interrupt for event WRITE"]
pub type WriteW<'a, REG> = crate::BitWriter<'a, REG, WriteWO>;
impl<'a, REG> WriteW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(WriteWO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event READ\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Read {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Read> for bool {
    #[inline(always)]
    fn from(variant: Read) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READ` reader - Write '1' to disable interrupt for event READ"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Read::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Read::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event READ\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReadWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<ReadWO> for bool {
    #[inline(always)]
    fn from(variant: ReadWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READ` writer - Write '1' to disable interrupt for event READ"]
pub type ReadW<'a, REG> = crate::BitWriter<'a, REG, ReadWO>;
impl<'a, REG> ReadW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ReadWO::Clear)
    }
}
impl R {
    #[doc = "Bit 1 - Write '1' to disable interrupt for event STOPPED"]
    #[inline(always)]
    pub fn stopped(&self) -> StoppedR {
        StoppedR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 9 - Write '1' to disable interrupt for event ERROR"]
    #[inline(always)]
    pub fn error(&self) -> ErrorR {
        ErrorR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 19 - Write '1' to disable interrupt for event RXSTARTED"]
    #[inline(always)]
    pub fn rxstarted(&self) -> RxstartedR {
        RxstartedR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Write '1' to disable interrupt for event TXSTARTED"]
    #[inline(always)]
    pub fn txstarted(&self) -> TxstartedR {
        TxstartedR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 25 - Write '1' to disable interrupt for event WRITE"]
    #[inline(always)]
    pub fn write(&self) -> WriteR {
        WriteR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Write '1' to disable interrupt for event READ"]
    #[inline(always)]
    pub fn read(&self) -> ReadR {
        ReadR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Write '1' to disable interrupt for event STOPPED"]
    #[inline(always)]
    #[must_use]
    pub fn stopped(&mut self) -> StoppedW<IntenclrSpec> {
        StoppedW::new(self, 1)
    }
    #[doc = "Bit 9 - Write '1' to disable interrupt for event ERROR"]
    #[inline(always)]
    #[must_use]
    pub fn error(&mut self) -> ErrorW<IntenclrSpec> {
        ErrorW::new(self, 9)
    }
    #[doc = "Bit 19 - Write '1' to disable interrupt for event RXSTARTED"]
    #[inline(always)]
    #[must_use]
    pub fn rxstarted(&mut self) -> RxstartedW<IntenclrSpec> {
        RxstartedW::new(self, 19)
    }
    #[doc = "Bit 20 - Write '1' to disable interrupt for event TXSTARTED"]
    #[inline(always)]
    #[must_use]
    pub fn txstarted(&mut self) -> TxstartedW<IntenclrSpec> {
        TxstartedW::new(self, 20)
    }
    #[doc = "Bit 25 - Write '1' to disable interrupt for event WRITE"]
    #[inline(always)]
    #[must_use]
    pub fn write(&mut self) -> WriteW<IntenclrSpec> {
        WriteW::new(self, 25)
    }
    #[doc = "Bit 26 - Write '1' to disable interrupt for event READ"]
    #[inline(always)]
    #[must_use]
    pub fn read(&mut self) -> ReadW<IntenclrSpec> {
        ReadW::new(self, 26)
    }
}
#[doc = "Disable interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intenclr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intenclr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenclrSpec;
impl crate::RegisterSpec for IntenclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenclr::R`](R) reader structure"]
impl crate::Readable for IntenclrSpec {}
#[doc = "`write(|w| ..)` method takes [`intenclr::W`](W) writer structure"]
impl crate::Writable for IntenclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for IntenclrSpec {
    const RESET_VALUE: u32 = 0;
}
