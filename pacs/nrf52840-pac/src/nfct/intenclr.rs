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
#[doc = "Write '1' to disable interrupt for event FIELDDETECTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fielddetected {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Fielddetected> for bool {
    #[inline(always)]
    fn from(variant: Fielddetected) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIELDDETECTED` reader - Write '1' to disable interrupt for event FIELDDETECTED"]
pub type FielddetectedR = crate::BitReader<Fielddetected>;
impl FielddetectedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fielddetected {
        match self.bits {
            false => Fielddetected::Disabled,
            true => Fielddetected::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Fielddetected::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Fielddetected::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event FIELDDETECTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FielddetectedWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<FielddetectedWO> for bool {
    #[inline(always)]
    fn from(variant: FielddetectedWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIELDDETECTED` writer - Write '1' to disable interrupt for event FIELDDETECTED"]
pub type FielddetectedW<'a, REG> = crate::BitWriter<'a, REG, FielddetectedWO>;
impl<'a, REG> FielddetectedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(FielddetectedWO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event FIELDLOST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fieldlost {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Fieldlost> for bool {
    #[inline(always)]
    fn from(variant: Fieldlost) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIELDLOST` reader - Write '1' to disable interrupt for event FIELDLOST"]
pub type FieldlostR = crate::BitReader<Fieldlost>;
impl FieldlostR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fieldlost {
        match self.bits {
            false => Fieldlost::Disabled,
            true => Fieldlost::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Fieldlost::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Fieldlost::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event FIELDLOST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldlostWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<FieldlostWO> for bool {
    #[inline(always)]
    fn from(variant: FieldlostWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIELDLOST` writer - Write '1' to disable interrupt for event FIELDLOST"]
pub type FieldlostW<'a, REG> = crate::BitWriter<'a, REG, FieldlostWO>;
impl<'a, REG> FieldlostW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(FieldlostWO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event TXFRAMESTART\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txframestart {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Txframestart> for bool {
    #[inline(always)]
    fn from(variant: Txframestart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFRAMESTART` reader - Write '1' to disable interrupt for event TXFRAMESTART"]
pub type TxframestartR = crate::BitReader<Txframestart>;
impl TxframestartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txframestart {
        match self.bits {
            false => Txframestart::Disabled,
            true => Txframestart::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Txframestart::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Txframestart::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event TXFRAMESTART\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxframestartWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<TxframestartWO> for bool {
    #[inline(always)]
    fn from(variant: TxframestartWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFRAMESTART` writer - Write '1' to disable interrupt for event TXFRAMESTART"]
pub type TxframestartW<'a, REG> = crate::BitWriter<'a, REG, TxframestartWO>;
impl<'a, REG> TxframestartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(TxframestartWO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event TXFRAMEEND\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txframeend {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Txframeend> for bool {
    #[inline(always)]
    fn from(variant: Txframeend) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFRAMEEND` reader - Write '1' to disable interrupt for event TXFRAMEEND"]
pub type TxframeendR = crate::BitReader<Txframeend>;
impl TxframeendR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txframeend {
        match self.bits {
            false => Txframeend::Disabled,
            true => Txframeend::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Txframeend::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Txframeend::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event TXFRAMEEND\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxframeendWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<TxframeendWO> for bool {
    #[inline(always)]
    fn from(variant: TxframeendWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFRAMEEND` writer - Write '1' to disable interrupt for event TXFRAMEEND"]
pub type TxframeendW<'a, REG> = crate::BitWriter<'a, REG, TxframeendWO>;
impl<'a, REG> TxframeendW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(TxframeendWO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event RXFRAMESTART\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxframestart {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Rxframestart> for bool {
    #[inline(always)]
    fn from(variant: Rxframestart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFRAMESTART` reader - Write '1' to disable interrupt for event RXFRAMESTART"]
pub type RxframestartR = crate::BitReader<Rxframestart>;
impl RxframestartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxframestart {
        match self.bits {
            false => Rxframestart::Disabled,
            true => Rxframestart::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rxframestart::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rxframestart::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event RXFRAMESTART\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RxframestartWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<RxframestartWO> for bool {
    #[inline(always)]
    fn from(variant: RxframestartWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFRAMESTART` writer - Write '1' to disable interrupt for event RXFRAMESTART"]
pub type RxframestartW<'a, REG> = crate::BitWriter<'a, REG, RxframestartWO>;
impl<'a, REG> RxframestartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(RxframestartWO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event RXFRAMEEND\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxframeend {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Rxframeend> for bool {
    #[inline(always)]
    fn from(variant: Rxframeend) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFRAMEEND` reader - Write '1' to disable interrupt for event RXFRAMEEND"]
pub type RxframeendR = crate::BitReader<Rxframeend>;
impl RxframeendR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxframeend {
        match self.bits {
            false => Rxframeend::Disabled,
            true => Rxframeend::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rxframeend::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rxframeend::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event RXFRAMEEND\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RxframeendWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<RxframeendWO> for bool {
    #[inline(always)]
    fn from(variant: RxframeendWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFRAMEEND` writer - Write '1' to disable interrupt for event RXFRAMEEND"]
pub type RxframeendW<'a, REG> = crate::BitWriter<'a, REG, RxframeendWO>;
impl<'a, REG> RxframeendW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(RxframeendWO::Clear)
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
#[doc = "Write '1' to disable interrupt for event RXERROR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxerror {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Rxerror> for bool {
    #[inline(always)]
    fn from(variant: Rxerror) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXERROR` reader - Write '1' to disable interrupt for event RXERROR"]
pub type RxerrorR = crate::BitReader<Rxerror>;
impl RxerrorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxerror {
        match self.bits {
            false => Rxerror::Disabled,
            true => Rxerror::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rxerror::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rxerror::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event RXERROR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RxerrorWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<RxerrorWO> for bool {
    #[inline(always)]
    fn from(variant: RxerrorWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXERROR` writer - Write '1' to disable interrupt for event RXERROR"]
pub type RxerrorW<'a, REG> = crate::BitWriter<'a, REG, RxerrorWO>;
impl<'a, REG> RxerrorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(RxerrorWO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event ENDRX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endrx {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Endrx> for bool {
    #[inline(always)]
    fn from(variant: Endrx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDRX` reader - Write '1' to disable interrupt for event ENDRX"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Endrx::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Endrx::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event ENDRX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EndrxWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<EndrxWO> for bool {
    #[inline(always)]
    fn from(variant: EndrxWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDRX` writer - Write '1' to disable interrupt for event ENDRX"]
pub type EndrxW<'a, REG> = crate::BitWriter<'a, REG, EndrxWO>;
impl<'a, REG> EndrxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(EndrxWO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event ENDTX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endtx {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Endtx> for bool {
    #[inline(always)]
    fn from(variant: Endtx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDTX` reader - Write '1' to disable interrupt for event ENDTX"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Endtx::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Endtx::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event ENDTX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EndtxWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<EndtxWO> for bool {
    #[inline(always)]
    fn from(variant: EndtxWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDTX` writer - Write '1' to disable interrupt for event ENDTX"]
pub type EndtxW<'a, REG> = crate::BitWriter<'a, REG, EndtxWO>;
impl<'a, REG> EndtxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(EndtxWO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event AUTOCOLRESSTARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Autocolresstarted {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Autocolresstarted> for bool {
    #[inline(always)]
    fn from(variant: Autocolresstarted) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUTOCOLRESSTARTED` reader - Write '1' to disable interrupt for event AUTOCOLRESSTARTED"]
pub type AutocolresstartedR = crate::BitReader<Autocolresstarted>;
impl AutocolresstartedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Autocolresstarted {
        match self.bits {
            false => Autocolresstarted::Disabled,
            true => Autocolresstarted::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Autocolresstarted::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Autocolresstarted::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event AUTOCOLRESSTARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AutocolresstartedWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<AutocolresstartedWO> for bool {
    #[inline(always)]
    fn from(variant: AutocolresstartedWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUTOCOLRESSTARTED` writer - Write '1' to disable interrupt for event AUTOCOLRESSTARTED"]
pub type AutocolresstartedW<'a, REG> = crate::BitWriter<'a, REG, AutocolresstartedWO>;
impl<'a, REG> AutocolresstartedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(AutocolresstartedWO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event COLLISION\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Collision {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Collision> for bool {
    #[inline(always)]
    fn from(variant: Collision) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COLLISION` reader - Write '1' to disable interrupt for event COLLISION"]
pub type CollisionR = crate::BitReader<Collision>;
impl CollisionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Collision {
        match self.bits {
            false => Collision::Disabled,
            true => Collision::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Collision::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Collision::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event COLLISION\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CollisionWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<CollisionWO> for bool {
    #[inline(always)]
    fn from(variant: CollisionWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COLLISION` writer - Write '1' to disable interrupt for event COLLISION"]
pub type CollisionW<'a, REG> = crate::BitWriter<'a, REG, CollisionWO>;
impl<'a, REG> CollisionW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CollisionWO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event SELECTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Selected {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Selected> for bool {
    #[inline(always)]
    fn from(variant: Selected) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SELECTED` reader - Write '1' to disable interrupt for event SELECTED"]
pub type SelectedR = crate::BitReader<Selected>;
impl SelectedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Selected {
        match self.bits {
            false => Selected::Disabled,
            true => Selected::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Selected::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Selected::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event SELECTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SelectedWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<SelectedWO> for bool {
    #[inline(always)]
    fn from(variant: SelectedWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SELECTED` writer - Write '1' to disable interrupt for event SELECTED"]
pub type SelectedW<'a, REG> = crate::BitWriter<'a, REG, SelectedWO>;
impl<'a, REG> SelectedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(SelectedWO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event STARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Started {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Started> for bool {
    #[inline(always)]
    fn from(variant: Started) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STARTED` reader - Write '1' to disable interrupt for event STARTED"]
pub type StartedR = crate::BitReader<Started>;
impl StartedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Started {
        match self.bits {
            false => Started::Disabled,
            true => Started::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Started::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Started::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event STARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StartedWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<StartedWO> for bool {
    #[inline(always)]
    fn from(variant: StartedWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STARTED` writer - Write '1' to disable interrupt for event STARTED"]
pub type StartedW<'a, REG> = crate::BitWriter<'a, REG, StartedWO>;
impl<'a, REG> StartedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(StartedWO::Clear)
    }
}
impl R {
    #[doc = "Bit 0 - Write '1' to disable interrupt for event READY"]
    #[inline(always)]
    pub fn ready(&self) -> ReadyR {
        ReadyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write '1' to disable interrupt for event FIELDDETECTED"]
    #[inline(always)]
    pub fn fielddetected(&self) -> FielddetectedR {
        FielddetectedR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write '1' to disable interrupt for event FIELDLOST"]
    #[inline(always)]
    pub fn fieldlost(&self) -> FieldlostR {
        FieldlostR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write '1' to disable interrupt for event TXFRAMESTART"]
    #[inline(always)]
    pub fn txframestart(&self) -> TxframestartR {
        TxframestartR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write '1' to disable interrupt for event TXFRAMEEND"]
    #[inline(always)]
    pub fn txframeend(&self) -> TxframeendR {
        TxframeendR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write '1' to disable interrupt for event RXFRAMESTART"]
    #[inline(always)]
    pub fn rxframestart(&self) -> RxframestartR {
        RxframestartR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write '1' to disable interrupt for event RXFRAMEEND"]
    #[inline(always)]
    pub fn rxframeend(&self) -> RxframeendR {
        RxframeendR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write '1' to disable interrupt for event ERROR"]
    #[inline(always)]
    pub fn error(&self) -> ErrorR {
        ErrorR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - Write '1' to disable interrupt for event RXERROR"]
    #[inline(always)]
    pub fn rxerror(&self) -> RxerrorR {
        RxerrorR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Write '1' to disable interrupt for event ENDRX"]
    #[inline(always)]
    pub fn endrx(&self) -> EndrxR {
        EndrxR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Write '1' to disable interrupt for event ENDTX"]
    #[inline(always)]
    pub fn endtx(&self) -> EndtxR {
        EndtxR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Write '1' to disable interrupt for event AUTOCOLRESSTARTED"]
    #[inline(always)]
    pub fn autocolresstarted(&self) -> AutocolresstartedR {
        AutocolresstartedR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 18 - Write '1' to disable interrupt for event COLLISION"]
    #[inline(always)]
    pub fn collision(&self) -> CollisionR {
        CollisionR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Write '1' to disable interrupt for event SELECTED"]
    #[inline(always)]
    pub fn selected(&self) -> SelectedR {
        SelectedR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Write '1' to disable interrupt for event STARTED"]
    #[inline(always)]
    pub fn started(&self) -> StartedR {
        StartedR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to disable interrupt for event READY"]
    #[inline(always)]
    pub fn ready(&mut self) -> ReadyW<'_, IntenclrSpec> {
        ReadyW::new(self, 0)
    }
    #[doc = "Bit 1 - Write '1' to disable interrupt for event FIELDDETECTED"]
    #[inline(always)]
    pub fn fielddetected(&mut self) -> FielddetectedW<'_, IntenclrSpec> {
        FielddetectedW::new(self, 1)
    }
    #[doc = "Bit 2 - Write '1' to disable interrupt for event FIELDLOST"]
    #[inline(always)]
    pub fn fieldlost(&mut self) -> FieldlostW<'_, IntenclrSpec> {
        FieldlostW::new(self, 2)
    }
    #[doc = "Bit 3 - Write '1' to disable interrupt for event TXFRAMESTART"]
    #[inline(always)]
    pub fn txframestart(&mut self) -> TxframestartW<'_, IntenclrSpec> {
        TxframestartW::new(self, 3)
    }
    #[doc = "Bit 4 - Write '1' to disable interrupt for event TXFRAMEEND"]
    #[inline(always)]
    pub fn txframeend(&mut self) -> TxframeendW<'_, IntenclrSpec> {
        TxframeendW::new(self, 4)
    }
    #[doc = "Bit 5 - Write '1' to disable interrupt for event RXFRAMESTART"]
    #[inline(always)]
    pub fn rxframestart(&mut self) -> RxframestartW<'_, IntenclrSpec> {
        RxframestartW::new(self, 5)
    }
    #[doc = "Bit 6 - Write '1' to disable interrupt for event RXFRAMEEND"]
    #[inline(always)]
    pub fn rxframeend(&mut self) -> RxframeendW<'_, IntenclrSpec> {
        RxframeendW::new(self, 6)
    }
    #[doc = "Bit 7 - Write '1' to disable interrupt for event ERROR"]
    #[inline(always)]
    pub fn error(&mut self) -> ErrorW<'_, IntenclrSpec> {
        ErrorW::new(self, 7)
    }
    #[doc = "Bit 10 - Write '1' to disable interrupt for event RXERROR"]
    #[inline(always)]
    pub fn rxerror(&mut self) -> RxerrorW<'_, IntenclrSpec> {
        RxerrorW::new(self, 10)
    }
    #[doc = "Bit 11 - Write '1' to disable interrupt for event ENDRX"]
    #[inline(always)]
    pub fn endrx(&mut self) -> EndrxW<'_, IntenclrSpec> {
        EndrxW::new(self, 11)
    }
    #[doc = "Bit 12 - Write '1' to disable interrupt for event ENDTX"]
    #[inline(always)]
    pub fn endtx(&mut self) -> EndtxW<'_, IntenclrSpec> {
        EndtxW::new(self, 12)
    }
    #[doc = "Bit 14 - Write '1' to disable interrupt for event AUTOCOLRESSTARTED"]
    #[inline(always)]
    pub fn autocolresstarted(&mut self) -> AutocolresstartedW<'_, IntenclrSpec> {
        AutocolresstartedW::new(self, 14)
    }
    #[doc = "Bit 18 - Write '1' to disable interrupt for event COLLISION"]
    #[inline(always)]
    pub fn collision(&mut self) -> CollisionW<'_, IntenclrSpec> {
        CollisionW::new(self, 18)
    }
    #[doc = "Bit 19 - Write '1' to disable interrupt for event SELECTED"]
    #[inline(always)]
    pub fn selected(&mut self) -> SelectedW<'_, IntenclrSpec> {
        SelectedW::new(self, 19)
    }
    #[doc = "Bit 20 - Write '1' to disable interrupt for event STARTED"]
    #[inline(always)]
    pub fn started(&mut self) -> StartedW<'_, IntenclrSpec> {
        StartedW::new(self, 20)
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
