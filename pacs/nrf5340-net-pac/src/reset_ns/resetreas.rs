#[doc = "Register `RESETREAS` reader"]
pub type R = crate::R<ResetreasSpec>;
#[doc = "Register `RESETREAS` writer"]
pub type W = crate::W<ResetreasSpec>;
#[doc = "Reset from pin reset detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Resetpin {
    #[doc = "0: Not detected"]
    NotDetected = 0,
    #[doc = "1: Detected"]
    Detected = 1,
}
impl From<Resetpin> for bool {
    #[inline(always)]
    fn from(variant: Resetpin) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESETPIN` reader - Reset from pin reset detected"]
pub type ResetpinR = crate::BitReader<Resetpin>;
impl ResetpinR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Resetpin {
        match self.bits {
            false => Resetpin::NotDetected,
            true => Resetpin::Detected,
        }
    }
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == Resetpin::NotDetected
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == Resetpin::Detected
    }
}
#[doc = "Field `RESETPIN` writer - Reset from pin reset detected"]
pub type ResetpinW<'a, REG> = crate::BitWriter<'a, REG, Resetpin>;
impl<'a, REG> ResetpinW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut crate::W<REG> {
        self.variant(Resetpin::NotDetected)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut crate::W<REG> {
        self.variant(Resetpin::Detected)
    }
}
#[doc = "Reset from application watchdog timer 0 detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dog0 {
    #[doc = "0: Not detected"]
    NotDetected = 0,
    #[doc = "1: Detected"]
    Detected = 1,
}
impl From<Dog0> for bool {
    #[inline(always)]
    fn from(variant: Dog0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOG0` reader - Reset from application watchdog timer 0 detected"]
pub type Dog0R = crate::BitReader<Dog0>;
impl Dog0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dog0 {
        match self.bits {
            false => Dog0::NotDetected,
            true => Dog0::Detected,
        }
    }
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == Dog0::NotDetected
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == Dog0::Detected
    }
}
#[doc = "Field `DOG0` writer - Reset from application watchdog timer 0 detected"]
pub type Dog0W<'a, REG> = crate::BitWriter<'a, REG, Dog0>;
impl<'a, REG> Dog0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut crate::W<REG> {
        self.variant(Dog0::NotDetected)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut crate::W<REG> {
        self.variant(Dog0::Detected)
    }
}
#[doc = "Reset from application CTRL-AP detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctrlap {
    #[doc = "0: Not detected"]
    NotDetected = 0,
    #[doc = "1: Detected"]
    Detected = 1,
}
impl From<Ctrlap> for bool {
    #[inline(always)]
    fn from(variant: Ctrlap) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTRLAP` reader - Reset from application CTRL-AP detected"]
pub type CtrlapR = crate::BitReader<Ctrlap>;
impl CtrlapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctrlap {
        match self.bits {
            false => Ctrlap::NotDetected,
            true => Ctrlap::Detected,
        }
    }
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == Ctrlap::NotDetected
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == Ctrlap::Detected
    }
}
#[doc = "Field `CTRLAP` writer - Reset from application CTRL-AP detected"]
pub type CtrlapW<'a, REG> = crate::BitWriter<'a, REG, Ctrlap>;
impl<'a, REG> CtrlapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut crate::W<REG> {
        self.variant(Ctrlap::NotDetected)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut crate::W<REG> {
        self.variant(Ctrlap::Detected)
    }
}
#[doc = "Reset from application soft reset detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sreq {
    #[doc = "0: Not detected"]
    NotDetected = 0,
    #[doc = "1: Detected"]
    Detected = 1,
}
impl From<Sreq> for bool {
    #[inline(always)]
    fn from(variant: Sreq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SREQ` reader - Reset from application soft reset detected"]
pub type SreqR = crate::BitReader<Sreq>;
impl SreqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sreq {
        match self.bits {
            false => Sreq::NotDetected,
            true => Sreq::Detected,
        }
    }
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == Sreq::NotDetected
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == Sreq::Detected
    }
}
#[doc = "Field `SREQ` writer - Reset from application soft reset detected"]
pub type SreqW<'a, REG> = crate::BitWriter<'a, REG, Sreq>;
impl<'a, REG> SreqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut crate::W<REG> {
        self.variant(Sreq::NotDetected)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut crate::W<REG> {
        self.variant(Sreq::Detected)
    }
}
#[doc = "Reset from application CPU lockup detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lockup {
    #[doc = "0: Not detected"]
    NotDetected = 0,
    #[doc = "1: Detected"]
    Detected = 1,
}
impl From<Lockup> for bool {
    #[inline(always)]
    fn from(variant: Lockup) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCKUP` reader - Reset from application CPU lockup detected"]
pub type LockupR = crate::BitReader<Lockup>;
impl LockupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lockup {
        match self.bits {
            false => Lockup::NotDetected,
            true => Lockup::Detected,
        }
    }
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == Lockup::NotDetected
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == Lockup::Detected
    }
}
#[doc = "Field `LOCKUP` writer - Reset from application CPU lockup detected"]
pub type LockupW<'a, REG> = crate::BitWriter<'a, REG, Lockup>;
impl<'a, REG> LockupW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut crate::W<REG> {
        self.variant(Lockup::NotDetected)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut crate::W<REG> {
        self.variant(Lockup::Detected)
    }
}
#[doc = "Reset due to wakeup from System OFF mode when wakeup is triggered by DETECT signal from GPIO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Off {
    #[doc = "0: Not detected"]
    NotDetected = 0,
    #[doc = "1: Detected"]
    Detected = 1,
}
impl From<Off> for bool {
    #[inline(always)]
    fn from(variant: Off) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OFF` reader - Reset due to wakeup from System OFF mode when wakeup is triggered by DETECT signal from GPIO"]
pub type OffR = crate::BitReader<Off>;
impl OffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Off {
        match self.bits {
            false => Off::NotDetected,
            true => Off::Detected,
        }
    }
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == Off::NotDetected
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == Off::Detected
    }
}
#[doc = "Field `OFF` writer - Reset due to wakeup from System OFF mode when wakeup is triggered by DETECT signal from GPIO"]
pub type OffW<'a, REG> = crate::BitWriter<'a, REG, Off>;
impl<'a, REG> OffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut crate::W<REG> {
        self.variant(Off::NotDetected)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut crate::W<REG> {
        self.variant(Off::Detected)
    }
}
#[doc = "Reset due to wakeup from System OFF mode when wakeup is triggered by ANADETECT signal from LPCOMP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lpcomp {
    #[doc = "0: Not detected"]
    NotDetected = 0,
    #[doc = "1: Detected"]
    Detected = 1,
}
impl From<Lpcomp> for bool {
    #[inline(always)]
    fn from(variant: Lpcomp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPCOMP` reader - Reset due to wakeup from System OFF mode when wakeup is triggered by ANADETECT signal from LPCOMP"]
pub type LpcompR = crate::BitReader<Lpcomp>;
impl LpcompR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpcomp {
        match self.bits {
            false => Lpcomp::NotDetected,
            true => Lpcomp::Detected,
        }
    }
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == Lpcomp::NotDetected
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == Lpcomp::Detected
    }
}
#[doc = "Field `LPCOMP` writer - Reset due to wakeup from System OFF mode when wakeup is triggered by ANADETECT signal from LPCOMP"]
pub type LpcompW<'a, REG> = crate::BitWriter<'a, REG, Lpcomp>;
impl<'a, REG> LpcompW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut crate::W<REG> {
        self.variant(Lpcomp::NotDetected)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut crate::W<REG> {
        self.variant(Lpcomp::Detected)
    }
}
#[doc = "Reset due to wakeup from System OFF mode when wakeup is triggered by entering the Debug Interface mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dif {
    #[doc = "0: Not detected"]
    NotDetected = 0,
    #[doc = "1: Detected"]
    Detected = 1,
}
impl From<Dif> for bool {
    #[inline(always)]
    fn from(variant: Dif) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIF` reader - Reset due to wakeup from System OFF mode when wakeup is triggered by entering the Debug Interface mode"]
pub type DifR = crate::BitReader<Dif>;
impl DifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dif {
        match self.bits {
            false => Dif::NotDetected,
            true => Dif::Detected,
        }
    }
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == Dif::NotDetected
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == Dif::Detected
    }
}
#[doc = "Field `DIF` writer - Reset due to wakeup from System OFF mode when wakeup is triggered by entering the Debug Interface mode"]
pub type DifW<'a, REG> = crate::BitWriter<'a, REG, Dif>;
impl<'a, REG> DifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut crate::W<REG> {
        self.variant(Dif::NotDetected)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut crate::W<REG> {
        self.variant(Dif::Detected)
    }
}
#[doc = "Reset from network soft reset detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lsreq {
    #[doc = "0: Not detected"]
    NotDetected = 0,
    #[doc = "1: Detected"]
    Detected = 1,
}
impl From<Lsreq> for bool {
    #[inline(always)]
    fn from(variant: Lsreq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSREQ` reader - Reset from network soft reset detected"]
pub type LsreqR = crate::BitReader<Lsreq>;
impl LsreqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lsreq {
        match self.bits {
            false => Lsreq::NotDetected,
            true => Lsreq::Detected,
        }
    }
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == Lsreq::NotDetected
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == Lsreq::Detected
    }
}
#[doc = "Field `LSREQ` writer - Reset from network soft reset detected"]
pub type LsreqW<'a, REG> = crate::BitWriter<'a, REG, Lsreq>;
impl<'a, REG> LsreqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut crate::W<REG> {
        self.variant(Lsreq::NotDetected)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut crate::W<REG> {
        self.variant(Lsreq::Detected)
    }
}
#[doc = "Reset from network CPU lockup detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Llockup {
    #[doc = "0: Not detected"]
    NotDetected = 0,
    #[doc = "1: Detected"]
    Detected = 1,
}
impl From<Llockup> for bool {
    #[inline(always)]
    fn from(variant: Llockup) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LLOCKUP` reader - Reset from network CPU lockup detected"]
pub type LlockupR = crate::BitReader<Llockup>;
impl LlockupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Llockup {
        match self.bits {
            false => Llockup::NotDetected,
            true => Llockup::Detected,
        }
    }
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == Llockup::NotDetected
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == Llockup::Detected
    }
}
#[doc = "Field `LLOCKUP` writer - Reset from network CPU lockup detected"]
pub type LlockupW<'a, REG> = crate::BitWriter<'a, REG, Llockup>;
impl<'a, REG> LlockupW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut crate::W<REG> {
        self.variant(Llockup::NotDetected)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut crate::W<REG> {
        self.variant(Llockup::Detected)
    }
}
#[doc = "Reset from network watchdog timer detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ldog {
    #[doc = "0: Not detected"]
    NotDetected = 0,
    #[doc = "1: Detected"]
    Detected = 1,
}
impl From<Ldog> for bool {
    #[inline(always)]
    fn from(variant: Ldog) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LDOG` reader - Reset from network watchdog timer detected"]
pub type LdogR = crate::BitReader<Ldog>;
impl LdogR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ldog {
        match self.bits {
            false => Ldog::NotDetected,
            true => Ldog::Detected,
        }
    }
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == Ldog::NotDetected
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == Ldog::Detected
    }
}
#[doc = "Field `LDOG` writer - Reset from network watchdog timer detected"]
pub type LdogW<'a, REG> = crate::BitWriter<'a, REG, Ldog>;
impl<'a, REG> LdogW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut crate::W<REG> {
        self.variant(Ldog::NotDetected)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut crate::W<REG> {
        self.variant(Ldog::Detected)
    }
}
#[doc = "Force-OFF reset from application core detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mforceoff {
    #[doc = "0: Not detected"]
    NotDetected = 0,
    #[doc = "1: Detected"]
    Detected = 1,
}
impl From<Mforceoff> for bool {
    #[inline(always)]
    fn from(variant: Mforceoff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MFORCEOFF` reader - Force-OFF reset from application core detected"]
pub type MforceoffR = crate::BitReader<Mforceoff>;
impl MforceoffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mforceoff {
        match self.bits {
            false => Mforceoff::NotDetected,
            true => Mforceoff::Detected,
        }
    }
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == Mforceoff::NotDetected
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == Mforceoff::Detected
    }
}
#[doc = "Field `MFORCEOFF` writer - Force-OFF reset from application core detected"]
pub type MforceoffW<'a, REG> = crate::BitWriter<'a, REG, Mforceoff>;
impl<'a, REG> MforceoffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut crate::W<REG> {
        self.variant(Mforceoff::NotDetected)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut crate::W<REG> {
        self.variant(Mforceoff::Detected)
    }
}
#[doc = "Reset after wakeup from System OFF mode due to NFC field being detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nfc {
    #[doc = "0: Not detected"]
    NotDetected = 0,
    #[doc = "1: Detected"]
    Detected = 1,
}
impl From<Nfc> for bool {
    #[inline(always)]
    fn from(variant: Nfc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NFC` reader - Reset after wakeup from System OFF mode due to NFC field being detected"]
pub type NfcR = crate::BitReader<Nfc>;
impl NfcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nfc {
        match self.bits {
            false => Nfc::NotDetected,
            true => Nfc::Detected,
        }
    }
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == Nfc::NotDetected
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == Nfc::Detected
    }
}
#[doc = "Field `NFC` writer - Reset after wakeup from System OFF mode due to NFC field being detected"]
pub type NfcW<'a, REG> = crate::BitWriter<'a, REG, Nfc>;
impl<'a, REG> NfcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut crate::W<REG> {
        self.variant(Nfc::NotDetected)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut crate::W<REG> {
        self.variant(Nfc::Detected)
    }
}
#[doc = "Reset from application watchdog timer 1 detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dog1 {
    #[doc = "0: Not detected"]
    NotDetected = 0,
    #[doc = "1: Detected"]
    Detected = 1,
}
impl From<Dog1> for bool {
    #[inline(always)]
    fn from(variant: Dog1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOG1` reader - Reset from application watchdog timer 1 detected"]
pub type Dog1R = crate::BitReader<Dog1>;
impl Dog1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dog1 {
        match self.bits {
            false => Dog1::NotDetected,
            true => Dog1::Detected,
        }
    }
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == Dog1::NotDetected
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == Dog1::Detected
    }
}
#[doc = "Field `DOG1` writer - Reset from application watchdog timer 1 detected"]
pub type Dog1W<'a, REG> = crate::BitWriter<'a, REG, Dog1>;
impl<'a, REG> Dog1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut crate::W<REG> {
        self.variant(Dog1::NotDetected)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut crate::W<REG> {
        self.variant(Dog1::Detected)
    }
}
#[doc = "Reset after wakeup from System OFF mode due to VBUS rising into valid range\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vbus {
    #[doc = "0: Not detected"]
    NotDetected = 0,
    #[doc = "1: Detected"]
    Detected = 1,
}
impl From<Vbus> for bool {
    #[inline(always)]
    fn from(variant: Vbus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBUS` reader - Reset after wakeup from System OFF mode due to VBUS rising into valid range"]
pub type VbusR = crate::BitReader<Vbus>;
impl VbusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vbus {
        match self.bits {
            false => Vbus::NotDetected,
            true => Vbus::Detected,
        }
    }
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == Vbus::NotDetected
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == Vbus::Detected
    }
}
#[doc = "Field `VBUS` writer - Reset after wakeup from System OFF mode due to VBUS rising into valid range"]
pub type VbusW<'a, REG> = crate::BitWriter<'a, REG, Vbus>;
impl<'a, REG> VbusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut crate::W<REG> {
        self.variant(Vbus::NotDetected)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut crate::W<REG> {
        self.variant(Vbus::Detected)
    }
}
#[doc = "Reset from network CTRL-AP detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lctrlap {
    #[doc = "0: Not detected"]
    NotDetected = 0,
    #[doc = "1: Detected"]
    Detected = 1,
}
impl From<Lctrlap> for bool {
    #[inline(always)]
    fn from(variant: Lctrlap) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCTRLAP` reader - Reset from network CTRL-AP detected"]
pub type LctrlapR = crate::BitReader<Lctrlap>;
impl LctrlapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lctrlap {
        match self.bits {
            false => Lctrlap::NotDetected,
            true => Lctrlap::Detected,
        }
    }
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == Lctrlap::NotDetected
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == Lctrlap::Detected
    }
}
#[doc = "Field `LCTRLAP` writer - Reset from network CTRL-AP detected"]
pub type LctrlapW<'a, REG> = crate::BitWriter<'a, REG, Lctrlap>;
impl<'a, REG> LctrlapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut crate::W<REG> {
        self.variant(Lctrlap::NotDetected)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut crate::W<REG> {
        self.variant(Lctrlap::Detected)
    }
}
impl R {
    #[doc = "Bit 0 - Reset from pin reset detected"]
    #[inline(always)]
    pub fn resetpin(&self) -> ResetpinR {
        ResetpinR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reset from application watchdog timer 0 detected"]
    #[inline(always)]
    pub fn dog0(&self) -> Dog0R {
        Dog0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reset from application CTRL-AP detected"]
    #[inline(always)]
    pub fn ctrlap(&self) -> CtrlapR {
        CtrlapR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reset from application soft reset detected"]
    #[inline(always)]
    pub fn sreq(&self) -> SreqR {
        SreqR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reset from application CPU lockup detected"]
    #[inline(always)]
    pub fn lockup(&self) -> LockupR {
        LockupR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Reset due to wakeup from System OFF mode when wakeup is triggered by DETECT signal from GPIO"]
    #[inline(always)]
    pub fn off(&self) -> OffR {
        OffR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Reset due to wakeup from System OFF mode when wakeup is triggered by ANADETECT signal from LPCOMP"]
    #[inline(always)]
    pub fn lpcomp(&self) -> LpcompR {
        LpcompR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reset due to wakeup from System OFF mode when wakeup is triggered by entering the Debug Interface mode"]
    #[inline(always)]
    pub fn dif(&self) -> DifR {
        DifR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - Reset from network soft reset detected"]
    #[inline(always)]
    pub fn lsreq(&self) -> LsreqR {
        LsreqR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Reset from network CPU lockup detected"]
    #[inline(always)]
    pub fn llockup(&self) -> LlockupR {
        LlockupR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Reset from network watchdog timer detected"]
    #[inline(always)]
    pub fn ldog(&self) -> LdogR {
        LdogR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 23 - Force-OFF reset from application core detected"]
    #[inline(always)]
    pub fn mforceoff(&self) -> MforceoffR {
        MforceoffR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Reset after wakeup from System OFF mode due to NFC field being detected"]
    #[inline(always)]
    pub fn nfc(&self) -> NfcR {
        NfcR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Reset from application watchdog timer 1 detected"]
    #[inline(always)]
    pub fn dog1(&self) -> Dog1R {
        Dog1R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Reset after wakeup from System OFF mode due to VBUS rising into valid range"]
    #[inline(always)]
    pub fn vbus(&self) -> VbusR {
        VbusR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Reset from network CTRL-AP detected"]
    #[inline(always)]
    pub fn lctrlap(&self) -> LctrlapR {
        LctrlapR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reset from pin reset detected"]
    #[inline(always)]
    pub fn resetpin(&mut self) -> ResetpinW<'_, ResetreasSpec> {
        ResetpinW::new(self, 0)
    }
    #[doc = "Bit 1 - Reset from application watchdog timer 0 detected"]
    #[inline(always)]
    pub fn dog0(&mut self) -> Dog0W<'_, ResetreasSpec> {
        Dog0W::new(self, 1)
    }
    #[doc = "Bit 2 - Reset from application CTRL-AP detected"]
    #[inline(always)]
    pub fn ctrlap(&mut self) -> CtrlapW<'_, ResetreasSpec> {
        CtrlapW::new(self, 2)
    }
    #[doc = "Bit 3 - Reset from application soft reset detected"]
    #[inline(always)]
    pub fn sreq(&mut self) -> SreqW<'_, ResetreasSpec> {
        SreqW::new(self, 3)
    }
    #[doc = "Bit 4 - Reset from application CPU lockup detected"]
    #[inline(always)]
    pub fn lockup(&mut self) -> LockupW<'_, ResetreasSpec> {
        LockupW::new(self, 4)
    }
    #[doc = "Bit 5 - Reset due to wakeup from System OFF mode when wakeup is triggered by DETECT signal from GPIO"]
    #[inline(always)]
    pub fn off(&mut self) -> OffW<'_, ResetreasSpec> {
        OffW::new(self, 5)
    }
    #[doc = "Bit 6 - Reset due to wakeup from System OFF mode when wakeup is triggered by ANADETECT signal from LPCOMP"]
    #[inline(always)]
    pub fn lpcomp(&mut self) -> LpcompW<'_, ResetreasSpec> {
        LpcompW::new(self, 6)
    }
    #[doc = "Bit 7 - Reset due to wakeup from System OFF mode when wakeup is triggered by entering the Debug Interface mode"]
    #[inline(always)]
    pub fn dif(&mut self) -> DifW<'_, ResetreasSpec> {
        DifW::new(self, 7)
    }
    #[doc = "Bit 16 - Reset from network soft reset detected"]
    #[inline(always)]
    pub fn lsreq(&mut self) -> LsreqW<'_, ResetreasSpec> {
        LsreqW::new(self, 16)
    }
    #[doc = "Bit 17 - Reset from network CPU lockup detected"]
    #[inline(always)]
    pub fn llockup(&mut self) -> LlockupW<'_, ResetreasSpec> {
        LlockupW::new(self, 17)
    }
    #[doc = "Bit 18 - Reset from network watchdog timer detected"]
    #[inline(always)]
    pub fn ldog(&mut self) -> LdogW<'_, ResetreasSpec> {
        LdogW::new(self, 18)
    }
    #[doc = "Bit 23 - Force-OFF reset from application core detected"]
    #[inline(always)]
    pub fn mforceoff(&mut self) -> MforceoffW<'_, ResetreasSpec> {
        MforceoffW::new(self, 23)
    }
    #[doc = "Bit 24 - Reset after wakeup from System OFF mode due to NFC field being detected"]
    #[inline(always)]
    pub fn nfc(&mut self) -> NfcW<'_, ResetreasSpec> {
        NfcW::new(self, 24)
    }
    #[doc = "Bit 25 - Reset from application watchdog timer 1 detected"]
    #[inline(always)]
    pub fn dog1(&mut self) -> Dog1W<'_, ResetreasSpec> {
        Dog1W::new(self, 25)
    }
    #[doc = "Bit 26 - Reset after wakeup from System OFF mode due to VBUS rising into valid range"]
    #[inline(always)]
    pub fn vbus(&mut self) -> VbusW<'_, ResetreasSpec> {
        VbusW::new(self, 26)
    }
    #[doc = "Bit 27 - Reset from network CTRL-AP detected"]
    #[inline(always)]
    pub fn lctrlap(&mut self) -> LctrlapW<'_, ResetreasSpec> {
        LctrlapW::new(self, 27)
    }
}
#[doc = "Reset reason\n\nYou can [`read`](crate::Reg::read) this register and get [`resetreas::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`resetreas::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ResetreasSpec;
impl crate::RegisterSpec for ResetreasSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resetreas::R`](R) reader structure"]
impl crate::Readable for ResetreasSpec {}
#[doc = "`write(|w| ..)` method takes [`resetreas::W`](W) writer structure"]
impl crate::Writable for ResetreasSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RESETREAS to value 0"]
impl crate::Resettable for ResetreasSpec {}
