#[doc = "Register `INTEN` reader"]
pub type R = crate::R<IntenSpec>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<IntenSpec>;
#[doc = "Enable or disable interrupt for event STOPPED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stopped {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Stopped> for bool {
    #[inline(always)]
    fn from(variant: Stopped) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPPED` reader - Enable or disable interrupt for event STOPPED"]
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
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Stopped::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Stopped::Enabled
    }
}
#[doc = "Field `STOPPED` writer - Enable or disable interrupt for event STOPPED"]
pub type StoppedW<'a, REG> = crate::BitWriter<'a, REG, Stopped>;
impl<'a, REG> StoppedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Stopped::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Stopped::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event ERROR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Error> for bool {
    #[inline(always)]
    fn from(variant: Error) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERROR` reader - Enable or disable interrupt for event ERROR"]
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
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Error::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Error::Enabled
    }
}
#[doc = "Field `ERROR` writer - Enable or disable interrupt for event ERROR"]
pub type ErrorW<'a, REG> = crate::BitWriter<'a, REG, Error>;
impl<'a, REG> ErrorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Error::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Error::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event RXSTARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxstarted {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Rxstarted> for bool {
    #[inline(always)]
    fn from(variant: Rxstarted) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXSTARTED` reader - Enable or disable interrupt for event RXSTARTED"]
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
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rxstarted::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rxstarted::Enabled
    }
}
#[doc = "Field `RXSTARTED` writer - Enable or disable interrupt for event RXSTARTED"]
pub type RxstartedW<'a, REG> = crate::BitWriter<'a, REG, Rxstarted>;
impl<'a, REG> RxstartedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rxstarted::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rxstarted::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event TXSTARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txstarted {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Txstarted> for bool {
    #[inline(always)]
    fn from(variant: Txstarted) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXSTARTED` reader - Enable or disable interrupt for event TXSTARTED"]
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
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Txstarted::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Txstarted::Enabled
    }
}
#[doc = "Field `TXSTARTED` writer - Enable or disable interrupt for event TXSTARTED"]
pub type TxstartedW<'a, REG> = crate::BitWriter<'a, REG, Txstarted>;
impl<'a, REG> TxstartedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Txstarted::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Txstarted::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event WRITE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Write {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Write> for bool {
    #[inline(always)]
    fn from(variant: Write) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRITE` reader - Enable or disable interrupt for event WRITE"]
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
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Write::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Write::Enabled
    }
}
#[doc = "Field `WRITE` writer - Enable or disable interrupt for event WRITE"]
pub type WriteW<'a, REG> = crate::BitWriter<'a, REG, Write>;
impl<'a, REG> WriteW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Write::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Write::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event READ\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Read {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Read> for bool {
    #[inline(always)]
    fn from(variant: Read) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READ` reader - Enable or disable interrupt for event READ"]
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
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Read::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Read::Enabled
    }
}
#[doc = "Field `READ` writer - Enable or disable interrupt for event READ"]
pub type ReadW<'a, REG> = crate::BitWriter<'a, REG, Read>;
impl<'a, REG> ReadW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Read::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Read::Enabled)
    }
}
impl R {
    #[doc = "Bit 1 - Enable or disable interrupt for event STOPPED"]
    #[inline(always)]
    pub fn stopped(&self) -> StoppedR {
        StoppedR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable or disable interrupt for event ERROR"]
    #[inline(always)]
    pub fn error(&self) -> ErrorR {
        ErrorR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable or disable interrupt for event RXSTARTED"]
    #[inline(always)]
    pub fn rxstarted(&self) -> RxstartedR {
        RxstartedR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable or disable interrupt for event TXSTARTED"]
    #[inline(always)]
    pub fn txstarted(&self) -> TxstartedR {
        TxstartedR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable or disable interrupt for event WRITE"]
    #[inline(always)]
    pub fn write(&self) -> WriteR {
        WriteR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable or disable interrupt for event READ"]
    #[inline(always)]
    pub fn read(&self) -> ReadR {
        ReadR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Enable or disable interrupt for event STOPPED"]
    #[inline(always)]
    #[must_use]
    pub fn stopped(&mut self) -> StoppedW<IntenSpec> {
        StoppedW::new(self, 1)
    }
    #[doc = "Bit 9 - Enable or disable interrupt for event ERROR"]
    #[inline(always)]
    #[must_use]
    pub fn error(&mut self) -> ErrorW<IntenSpec> {
        ErrorW::new(self, 9)
    }
    #[doc = "Bit 19 - Enable or disable interrupt for event RXSTARTED"]
    #[inline(always)]
    #[must_use]
    pub fn rxstarted(&mut self) -> RxstartedW<IntenSpec> {
        RxstartedW::new(self, 19)
    }
    #[doc = "Bit 20 - Enable or disable interrupt for event TXSTARTED"]
    #[inline(always)]
    #[must_use]
    pub fn txstarted(&mut self) -> TxstartedW<IntenSpec> {
        TxstartedW::new(self, 20)
    }
    #[doc = "Bit 25 - Enable or disable interrupt for event WRITE"]
    #[inline(always)]
    #[must_use]
    pub fn write(&mut self) -> WriteW<IntenSpec> {
        WriteW::new(self, 25)
    }
    #[doc = "Bit 26 - Enable or disable interrupt for event READ"]
    #[inline(always)]
    #[must_use]
    pub fn read(&mut self) -> ReadW<IntenSpec> {
        ReadW::new(self, 26)
    }
}
#[doc = "Enable or disable interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenSpec;
impl crate::RegisterSpec for IntenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for IntenSpec {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for IntenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for IntenSpec {
    const RESET_VALUE: u32 = 0;
}
