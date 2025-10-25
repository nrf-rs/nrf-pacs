#[doc = "Register `INTEN` reader"]
pub type R = crate::R<IntenSpec>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<IntenSpec>;
#[doc = "Enable or disable interrupt for CTS event\n\nValue on reset: 0"]
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
#[doc = "Field `CTS` reader - Enable or disable interrupt for CTS event"]
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
#[doc = "Field `CTS` writer - Enable or disable interrupt for CTS event"]
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
#[doc = "Enable or disable interrupt for NCTS event\n\nValue on reset: 0"]
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
#[doc = "Field `NCTS` reader - Enable or disable interrupt for NCTS event"]
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
#[doc = "Field `NCTS` writer - Enable or disable interrupt for NCTS event"]
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
#[doc = "Enable or disable interrupt for RXDRDY event\n\nValue on reset: 0"]
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
#[doc = "Field `RXDRDY` reader - Enable or disable interrupt for RXDRDY event"]
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
#[doc = "Field `RXDRDY` writer - Enable or disable interrupt for RXDRDY event"]
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
#[doc = "Enable or disable interrupt for ENDRX event\n\nValue on reset: 0"]
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
#[doc = "Field `ENDRX` reader - Enable or disable interrupt for ENDRX event"]
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
#[doc = "Field `ENDRX` writer - Enable or disable interrupt for ENDRX event"]
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
#[doc = "Enable or disable interrupt for TXDRDY event\n\nValue on reset: 0"]
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
#[doc = "Field `TXDRDY` reader - Enable or disable interrupt for TXDRDY event"]
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
#[doc = "Field `TXDRDY` writer - Enable or disable interrupt for TXDRDY event"]
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
#[doc = "Enable or disable interrupt for ENDTX event\n\nValue on reset: 0"]
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
#[doc = "Field `ENDTX` reader - Enable or disable interrupt for ENDTX event"]
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
#[doc = "Field `ENDTX` writer - Enable or disable interrupt for ENDTX event"]
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
#[doc = "Enable or disable interrupt for ERROR event\n\nValue on reset: 0"]
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
#[doc = "Field `ERROR` reader - Enable or disable interrupt for ERROR event"]
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
#[doc = "Field `ERROR` writer - Enable or disable interrupt for ERROR event"]
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
#[doc = "Enable or disable interrupt for RXTO event\n\nValue on reset: 0"]
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
#[doc = "Field `RXTO` reader - Enable or disable interrupt for RXTO event"]
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
#[doc = "Field `RXTO` writer - Enable or disable interrupt for RXTO event"]
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
#[doc = "Enable or disable interrupt for RXSTARTED event\n\nValue on reset: 0"]
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
#[doc = "Field `RXSTARTED` reader - Enable or disable interrupt for RXSTARTED event"]
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
#[doc = "Field `RXSTARTED` writer - Enable or disable interrupt for RXSTARTED event"]
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
#[doc = "Enable or disable interrupt for TXSTARTED event\n\nValue on reset: 0"]
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
#[doc = "Field `TXSTARTED` reader - Enable or disable interrupt for TXSTARTED event"]
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
#[doc = "Field `TXSTARTED` writer - Enable or disable interrupt for TXSTARTED event"]
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
#[doc = "Enable or disable interrupt for TXSTOPPED event\n\nValue on reset: 0"]
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
#[doc = "Field `TXSTOPPED` reader - Enable or disable interrupt for TXSTOPPED event"]
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
#[doc = "Field `TXSTOPPED` writer - Enable or disable interrupt for TXSTOPPED event"]
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
    #[doc = "Bit 0 - Enable or disable interrupt for CTS event"]
    #[inline(always)]
    pub fn cts(&self) -> CtsR {
        CtsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable or disable interrupt for NCTS event"]
    #[inline(always)]
    pub fn ncts(&self) -> NctsR {
        NctsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable or disable interrupt for RXDRDY event"]
    #[inline(always)]
    pub fn rxdrdy(&self) -> RxdrdyR {
        RxdrdyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable or disable interrupt for ENDRX event"]
    #[inline(always)]
    pub fn endrx(&self) -> EndrxR {
        EndrxR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable or disable interrupt for TXDRDY event"]
    #[inline(always)]
    pub fn txdrdy(&self) -> TxdrdyR {
        TxdrdyR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable or disable interrupt for ENDTX event"]
    #[inline(always)]
    pub fn endtx(&self) -> EndtxR {
        EndtxR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable or disable interrupt for ERROR event"]
    #[inline(always)]
    pub fn error(&self) -> ErrorR {
        ErrorR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable or disable interrupt for RXTO event"]
    #[inline(always)]
    pub fn rxto(&self) -> RxtoR {
        RxtoR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable or disable interrupt for RXSTARTED event"]
    #[inline(always)]
    pub fn rxstarted(&self) -> RxstartedR {
        RxstartedR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable or disable interrupt for TXSTARTED event"]
    #[inline(always)]
    pub fn txstarted(&self) -> TxstartedR {
        TxstartedR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable or disable interrupt for TXSTOPPED event"]
    #[inline(always)]
    pub fn txstopped(&self) -> TxstoppedR {
        TxstoppedR::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable interrupt for CTS event"]
    #[inline(always)]
    pub fn cts(&mut self) -> CtsW<'_, IntenSpec> {
        CtsW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable or disable interrupt for NCTS event"]
    #[inline(always)]
    pub fn ncts(&mut self) -> NctsW<'_, IntenSpec> {
        NctsW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable or disable interrupt for RXDRDY event"]
    #[inline(always)]
    pub fn rxdrdy(&mut self) -> RxdrdyW<'_, IntenSpec> {
        RxdrdyW::new(self, 2)
    }
    #[doc = "Bit 4 - Enable or disable interrupt for ENDRX event"]
    #[inline(always)]
    pub fn endrx(&mut self) -> EndrxW<'_, IntenSpec> {
        EndrxW::new(self, 4)
    }
    #[doc = "Bit 7 - Enable or disable interrupt for TXDRDY event"]
    #[inline(always)]
    pub fn txdrdy(&mut self) -> TxdrdyW<'_, IntenSpec> {
        TxdrdyW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable or disable interrupt for ENDTX event"]
    #[inline(always)]
    pub fn endtx(&mut self) -> EndtxW<'_, IntenSpec> {
        EndtxW::new(self, 8)
    }
    #[doc = "Bit 9 - Enable or disable interrupt for ERROR event"]
    #[inline(always)]
    pub fn error(&mut self) -> ErrorW<'_, IntenSpec> {
        ErrorW::new(self, 9)
    }
    #[doc = "Bit 17 - Enable or disable interrupt for RXTO event"]
    #[inline(always)]
    pub fn rxto(&mut self) -> RxtoW<'_, IntenSpec> {
        RxtoW::new(self, 17)
    }
    #[doc = "Bit 19 - Enable or disable interrupt for RXSTARTED event"]
    #[inline(always)]
    pub fn rxstarted(&mut self) -> RxstartedW<'_, IntenSpec> {
        RxstartedW::new(self, 19)
    }
    #[doc = "Bit 20 - Enable or disable interrupt for TXSTARTED event"]
    #[inline(always)]
    pub fn txstarted(&mut self) -> TxstartedW<'_, IntenSpec> {
        TxstartedW::new(self, 20)
    }
    #[doc = "Bit 22 - Enable or disable interrupt for TXSTOPPED event"]
    #[inline(always)]
    pub fn txstopped(&mut self) -> TxstoppedW<'_, IntenSpec> {
        TxstoppedW::new(self, 22)
    }
}
#[doc = "Enable or disable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenSpec;
impl crate::RegisterSpec for IntenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for IntenSpec {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for IntenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for IntenSpec {}
