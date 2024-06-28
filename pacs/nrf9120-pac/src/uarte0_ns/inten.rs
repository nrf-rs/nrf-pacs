#[doc = "Register `INTEN` reader"]
pub type R = crate::R<IntenSpec>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<IntenSpec>;
#[doc = "Enable or disable interrupt for event CTS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cts {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Cts> for bool {
    #[inline(always)]
    fn from(variant: Cts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTS` reader - Enable or disable interrupt for event CTS"]
pub type CtsR = crate::BitReader<Cts>;
impl CtsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cts {
        match self.bits {
            false => Cts::Disabled,
            true => Cts::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Cts::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Cts::Enabled
    }
}
#[doc = "Field `CTS` writer - Enable or disable interrupt for event CTS"]
pub type CtsW<'a, REG> = crate::BitWriter<'a, REG, Cts>;
impl<'a, REG> CtsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cts::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cts::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event NCTS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ncts {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Ncts> for bool {
    #[inline(always)]
    fn from(variant: Ncts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NCTS` reader - Enable or disable interrupt for event NCTS"]
pub type NctsR = crate::BitReader<Ncts>;
impl NctsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ncts {
        match self.bits {
            false => Ncts::Disabled,
            true => Ncts::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ncts::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ncts::Enabled
    }
}
#[doc = "Field `NCTS` writer - Enable or disable interrupt for event NCTS"]
pub type NctsW<'a, REG> = crate::BitWriter<'a, REG, Ncts>;
impl<'a, REG> NctsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ncts::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ncts::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event RXDRDY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxdrdy {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Rxdrdy> for bool {
    #[inline(always)]
    fn from(variant: Rxdrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXDRDY` reader - Enable or disable interrupt for event RXDRDY"]
pub type RxdrdyR = crate::BitReader<Rxdrdy>;
impl RxdrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxdrdy {
        match self.bits {
            false => Rxdrdy::Disabled,
            true => Rxdrdy::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rxdrdy::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rxdrdy::Enabled
    }
}
#[doc = "Field `RXDRDY` writer - Enable or disable interrupt for event RXDRDY"]
pub type RxdrdyW<'a, REG> = crate::BitWriter<'a, REG, Rxdrdy>;
impl<'a, REG> RxdrdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rxdrdy::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rxdrdy::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event ENDRX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endrx {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Endrx> for bool {
    #[inline(always)]
    fn from(variant: Endrx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDRX` reader - Enable or disable interrupt for event ENDRX"]
pub type EndrxR = crate::BitReader<Endrx>;
impl EndrxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Endrx {
        match self.bits {
            false => Endrx::Disabled,
            true => Endrx::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Endrx::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Endrx::Enabled
    }
}
#[doc = "Field `ENDRX` writer - Enable or disable interrupt for event ENDRX"]
pub type EndrxW<'a, REG> = crate::BitWriter<'a, REG, Endrx>;
impl<'a, REG> EndrxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Endrx::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Endrx::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event TXDRDY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txdrdy {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Txdrdy> for bool {
    #[inline(always)]
    fn from(variant: Txdrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXDRDY` reader - Enable or disable interrupt for event TXDRDY"]
pub type TxdrdyR = crate::BitReader<Txdrdy>;
impl TxdrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txdrdy {
        match self.bits {
            false => Txdrdy::Disabled,
            true => Txdrdy::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Txdrdy::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Txdrdy::Enabled
    }
}
#[doc = "Field `TXDRDY` writer - Enable or disable interrupt for event TXDRDY"]
pub type TxdrdyW<'a, REG> = crate::BitWriter<'a, REG, Txdrdy>;
impl<'a, REG> TxdrdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Txdrdy::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Txdrdy::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event ENDTX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endtx {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Endtx> for bool {
    #[inline(always)]
    fn from(variant: Endtx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDTX` reader - Enable or disable interrupt for event ENDTX"]
pub type EndtxR = crate::BitReader<Endtx>;
impl EndtxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Endtx {
        match self.bits {
            false => Endtx::Disabled,
            true => Endtx::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Endtx::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Endtx::Enabled
    }
}
#[doc = "Field `ENDTX` writer - Enable or disable interrupt for event ENDTX"]
pub type EndtxW<'a, REG> = crate::BitWriter<'a, REG, Endtx>;
impl<'a, REG> EndtxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Endtx::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Endtx::Enabled)
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
#[doc = "Enable or disable interrupt for event RXTO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxto {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Rxto> for bool {
    #[inline(always)]
    fn from(variant: Rxto) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXTO` reader - Enable or disable interrupt for event RXTO"]
pub type RxtoR = crate::BitReader<Rxto>;
impl RxtoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxto {
        match self.bits {
            false => Rxto::Disabled,
            true => Rxto::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rxto::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rxto::Enabled
    }
}
#[doc = "Field `RXTO` writer - Enable or disable interrupt for event RXTO"]
pub type RxtoW<'a, REG> = crate::BitWriter<'a, REG, Rxto>;
impl<'a, REG> RxtoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rxto::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rxto::Enabled)
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
#[doc = "Enable or disable interrupt for event TXSTOPPED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txstopped {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Txstopped> for bool {
    #[inline(always)]
    fn from(variant: Txstopped) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXSTOPPED` reader - Enable or disable interrupt for event TXSTOPPED"]
pub type TxstoppedR = crate::BitReader<Txstopped>;
impl TxstoppedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txstopped {
        match self.bits {
            false => Txstopped::Disabled,
            true => Txstopped::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Txstopped::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Txstopped::Enabled
    }
}
#[doc = "Field `TXSTOPPED` writer - Enable or disable interrupt for event TXSTOPPED"]
pub type TxstoppedW<'a, REG> = crate::BitWriter<'a, REG, Txstopped>;
impl<'a, REG> TxstoppedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Txstopped::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Txstopped::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Enable or disable interrupt for event CTS"]
    #[inline(always)]
    pub fn cts(&self) -> CtsR {
        CtsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable or disable interrupt for event NCTS"]
    #[inline(always)]
    pub fn ncts(&self) -> NctsR {
        NctsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable or disable interrupt for event RXDRDY"]
    #[inline(always)]
    pub fn rxdrdy(&self) -> RxdrdyR {
        RxdrdyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable or disable interrupt for event ENDRX"]
    #[inline(always)]
    pub fn endrx(&self) -> EndrxR {
        EndrxR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable or disable interrupt for event TXDRDY"]
    #[inline(always)]
    pub fn txdrdy(&self) -> TxdrdyR {
        TxdrdyR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable or disable interrupt for event ENDTX"]
    #[inline(always)]
    pub fn endtx(&self) -> EndtxR {
        EndtxR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable or disable interrupt for event ERROR"]
    #[inline(always)]
    pub fn error(&self) -> ErrorR {
        ErrorR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable or disable interrupt for event RXTO"]
    #[inline(always)]
    pub fn rxto(&self) -> RxtoR {
        RxtoR::new(((self.bits >> 17) & 1) != 0)
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
    #[doc = "Bit 22 - Enable or disable interrupt for event TXSTOPPED"]
    #[inline(always)]
    pub fn txstopped(&self) -> TxstoppedR {
        TxstoppedR::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable interrupt for event CTS"]
    #[inline(always)]
    #[must_use]
    pub fn cts(&mut self) -> CtsW<IntenSpec> {
        CtsW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable or disable interrupt for event NCTS"]
    #[inline(always)]
    #[must_use]
    pub fn ncts(&mut self) -> NctsW<IntenSpec> {
        NctsW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable or disable interrupt for event RXDRDY"]
    #[inline(always)]
    #[must_use]
    pub fn rxdrdy(&mut self) -> RxdrdyW<IntenSpec> {
        RxdrdyW::new(self, 2)
    }
    #[doc = "Bit 4 - Enable or disable interrupt for event ENDRX"]
    #[inline(always)]
    #[must_use]
    pub fn endrx(&mut self) -> EndrxW<IntenSpec> {
        EndrxW::new(self, 4)
    }
    #[doc = "Bit 7 - Enable or disable interrupt for event TXDRDY"]
    #[inline(always)]
    #[must_use]
    pub fn txdrdy(&mut self) -> TxdrdyW<IntenSpec> {
        TxdrdyW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable or disable interrupt for event ENDTX"]
    #[inline(always)]
    #[must_use]
    pub fn endtx(&mut self) -> EndtxW<IntenSpec> {
        EndtxW::new(self, 8)
    }
    #[doc = "Bit 9 - Enable or disable interrupt for event ERROR"]
    #[inline(always)]
    #[must_use]
    pub fn error(&mut self) -> ErrorW<IntenSpec> {
        ErrorW::new(self, 9)
    }
    #[doc = "Bit 17 - Enable or disable interrupt for event RXTO"]
    #[inline(always)]
    #[must_use]
    pub fn rxto(&mut self) -> RxtoW<IntenSpec> {
        RxtoW::new(self, 17)
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
    #[doc = "Bit 22 - Enable or disable interrupt for event TXSTOPPED"]
    #[inline(always)]
    #[must_use]
    pub fn txstopped(&mut self) -> TxstoppedW<IntenSpec> {
        TxstoppedW::new(self, 22)
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
