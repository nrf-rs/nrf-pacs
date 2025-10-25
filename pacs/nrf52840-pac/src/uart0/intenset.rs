#[doc = "Register `INTENSET` reader"]
pub type R = crate::R<IntensetSpec>;
#[doc = "Register `INTENSET` writer"]
pub type W = crate::W<IntensetSpec>;
#[doc = "Write '1' to enable interrupt for event CTS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cts {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Cts> for bool {
    #[inline(always)]
    fn from(variant: Cts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTS` reader - Write '1' to enable interrupt for event CTS"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Cts::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Cts::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event CTS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CtsWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<CtsWO> for bool {
    #[inline(always)]
    fn from(variant: CtsWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTS` writer - Write '1' to enable interrupt for event CTS"]
pub type CtsW<'a, REG> = crate::BitWriter<'a, REG, CtsWO>;
impl<'a, REG> CtsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(CtsWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event NCTS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ncts {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Ncts> for bool {
    #[inline(always)]
    fn from(variant: Ncts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NCTS` reader - Write '1' to enable interrupt for event NCTS"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ncts::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ncts::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event NCTS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NctsWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<NctsWO> for bool {
    #[inline(always)]
    fn from(variant: NctsWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NCTS` writer - Write '1' to enable interrupt for event NCTS"]
pub type NctsW<'a, REG> = crate::BitWriter<'a, REG, NctsWO>;
impl<'a, REG> NctsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(NctsWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event RXDRDY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxdrdy {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Rxdrdy> for bool {
    #[inline(always)]
    fn from(variant: Rxdrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXDRDY` reader - Write '1' to enable interrupt for event RXDRDY"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rxdrdy::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rxdrdy::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event RXDRDY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RxdrdyWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<RxdrdyWO> for bool {
    #[inline(always)]
    fn from(variant: RxdrdyWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXDRDY` writer - Write '1' to enable interrupt for event RXDRDY"]
pub type RxdrdyW<'a, REG> = crate::BitWriter<'a, REG, RxdrdyWO>;
impl<'a, REG> RxdrdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(RxdrdyWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event TXDRDY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txdrdy {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Txdrdy> for bool {
    #[inline(always)]
    fn from(variant: Txdrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXDRDY` reader - Write '1' to enable interrupt for event TXDRDY"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Txdrdy::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Txdrdy::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event TXDRDY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxdrdyWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<TxdrdyWO> for bool {
    #[inline(always)]
    fn from(variant: TxdrdyWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXDRDY` writer - Write '1' to enable interrupt for event TXDRDY"]
pub type TxdrdyW<'a, REG> = crate::BitWriter<'a, REG, TxdrdyWO>;
impl<'a, REG> TxdrdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(TxdrdyWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event ERROR\n\nValue on reset: 0"]
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
#[doc = "Field `ERROR` reader - Write '1' to enable interrupt for event ERROR"]
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
#[doc = "Write '1' to enable interrupt for event ERROR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrorWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<ErrorWO> for bool {
    #[inline(always)]
    fn from(variant: ErrorWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERROR` writer - Write '1' to enable interrupt for event ERROR"]
pub type ErrorW<'a, REG> = crate::BitWriter<'a, REG, ErrorWO>;
impl<'a, REG> ErrorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(ErrorWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event RXTO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxto {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Rxto> for bool {
    #[inline(always)]
    fn from(variant: Rxto) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXTO` reader - Write '1' to enable interrupt for event RXTO"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rxto::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rxto::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event RXTO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RxtoWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<RxtoWO> for bool {
    #[inline(always)]
    fn from(variant: RxtoWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXTO` writer - Write '1' to enable interrupt for event RXTO"]
pub type RxtoW<'a, REG> = crate::BitWriter<'a, REG, RxtoWO>;
impl<'a, REG> RxtoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(RxtoWO::Set)
    }
}
impl R {
    #[doc = "Bit 0 - Write '1' to enable interrupt for event CTS"]
    #[inline(always)]
    pub fn cts(&self) -> CtsR {
        CtsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write '1' to enable interrupt for event NCTS"]
    #[inline(always)]
    pub fn ncts(&self) -> NctsR {
        NctsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write '1' to enable interrupt for event RXDRDY"]
    #[inline(always)]
    pub fn rxdrdy(&self) -> RxdrdyR {
        RxdrdyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 7 - Write '1' to enable interrupt for event TXDRDY"]
    #[inline(always)]
    pub fn txdrdy(&self) -> TxdrdyR {
        TxdrdyR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Write '1' to enable interrupt for event ERROR"]
    #[inline(always)]
    pub fn error(&self) -> ErrorR {
        ErrorR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 17 - Write '1' to enable interrupt for event RXTO"]
    #[inline(always)]
    pub fn rxto(&self) -> RxtoR {
        RxtoR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to enable interrupt for event CTS"]
    #[inline(always)]
    pub fn cts(&mut self) -> CtsW<'_, IntensetSpec> {
        CtsW::new(self, 0)
    }
    #[doc = "Bit 1 - Write '1' to enable interrupt for event NCTS"]
    #[inline(always)]
    pub fn ncts(&mut self) -> NctsW<'_, IntensetSpec> {
        NctsW::new(self, 1)
    }
    #[doc = "Bit 2 - Write '1' to enable interrupt for event RXDRDY"]
    #[inline(always)]
    pub fn rxdrdy(&mut self) -> RxdrdyW<'_, IntensetSpec> {
        RxdrdyW::new(self, 2)
    }
    #[doc = "Bit 7 - Write '1' to enable interrupt for event TXDRDY"]
    #[inline(always)]
    pub fn txdrdy(&mut self) -> TxdrdyW<'_, IntensetSpec> {
        TxdrdyW::new(self, 7)
    }
    #[doc = "Bit 9 - Write '1' to enable interrupt for event ERROR"]
    #[inline(always)]
    pub fn error(&mut self) -> ErrorW<'_, IntensetSpec> {
        ErrorW::new(self, 9)
    }
    #[doc = "Bit 17 - Write '1' to enable interrupt for event RXTO"]
    #[inline(always)]
    pub fn rxto(&mut self) -> RxtoW<'_, IntensetSpec> {
        RxtoW::new(self, 17)
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
