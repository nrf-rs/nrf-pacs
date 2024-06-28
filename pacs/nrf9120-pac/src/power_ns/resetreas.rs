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
#[doc = "Reset from global watchdog detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dog {
    #[doc = "0: Not detected"]
    NotDetected = 0,
    #[doc = "1: Detected"]
    Detected = 1,
}
impl From<Dog> for bool {
    #[inline(always)]
    fn from(variant: Dog) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOG` reader - Reset from global watchdog detected"]
pub type DogR = crate::BitReader<Dog>;
impl DogR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dog {
        match self.bits {
            false => Dog::NotDetected,
            true => Dog::Detected,
        }
    }
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == Dog::NotDetected
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == Dog::Detected
    }
}
#[doc = "Field `DOG` writer - Reset from global watchdog detected"]
pub type DogW<'a, REG> = crate::BitWriter<'a, REG, Dog>;
impl<'a, REG> DogW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut crate::W<REG> {
        self.variant(Dog::NotDetected)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut crate::W<REG> {
        self.variant(Dog::Detected)
    }
}
#[doc = "Reset due to wakeup from System OFF mode, when wakeup is triggered by DETECT signal from GPIO\n\nValue on reset: 0"]
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
#[doc = "Field `OFF` reader - Reset due to wakeup from System OFF mode, when wakeup is triggered by DETECT signal from GPIO"]
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
#[doc = "Field `OFF` writer - Reset due to wakeup from System OFF mode, when wakeup is triggered by DETECT signal from GPIO"]
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
#[doc = "Reset due to wakeup from System OFF mode, when wakeup is triggered by entering debug interface mode\n\nValue on reset: 0"]
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
#[doc = "Field `DIF` reader - Reset due to wakeup from System OFF mode, when wakeup is triggered by entering debug interface mode"]
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
#[doc = "Field `DIF` writer - Reset due to wakeup from System OFF mode, when wakeup is triggered by entering debug interface mode"]
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
#[doc = "Reset from AIRCR.SYSRESETREQ detected\n\nValue on reset: 0"]
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
#[doc = "Field `SREQ` reader - Reset from AIRCR.SYSRESETREQ detected"]
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
#[doc = "Field `SREQ` writer - Reset from AIRCR.SYSRESETREQ detected"]
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
#[doc = "Reset from CPU lock-up detected\n\nValue on reset: 0"]
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
#[doc = "Field `LOCKUP` reader - Reset from CPU lock-up detected"]
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
#[doc = "Field `LOCKUP` writer - Reset from CPU lock-up detected"]
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
#[doc = "Reset triggered through CTRL-AP\n\nValue on reset: 0"]
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
#[doc = "Field `CTRLAP` reader - Reset triggered through CTRL-AP"]
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
#[doc = "Field `CTRLAP` writer - Reset triggered through CTRL-AP"]
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
impl R {
    #[doc = "Bit 0 - Reset from pin reset detected"]
    #[inline(always)]
    pub fn resetpin(&self) -> ResetpinR {
        ResetpinR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reset from global watchdog detected"]
    #[inline(always)]
    pub fn dog(&self) -> DogR {
        DogR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reset due to wakeup from System OFF mode, when wakeup is triggered by DETECT signal from GPIO"]
    #[inline(always)]
    pub fn off(&self) -> OffR {
        OffR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Reset due to wakeup from System OFF mode, when wakeup is triggered by entering debug interface mode"]
    #[inline(always)]
    pub fn dif(&self) -> DifR {
        DifR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - Reset from AIRCR.SYSRESETREQ detected"]
    #[inline(always)]
    pub fn sreq(&self) -> SreqR {
        SreqR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Reset from CPU lock-up detected"]
    #[inline(always)]
    pub fn lockup(&self) -> LockupR {
        LockupR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Reset triggered through CTRL-AP"]
    #[inline(always)]
    pub fn ctrlap(&self) -> CtrlapR {
        CtrlapR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reset from pin reset detected"]
    #[inline(always)]
    #[must_use]
    pub fn resetpin(&mut self) -> ResetpinW<ResetreasSpec> {
        ResetpinW::new(self, 0)
    }
    #[doc = "Bit 1 - Reset from global watchdog detected"]
    #[inline(always)]
    #[must_use]
    pub fn dog(&mut self) -> DogW<ResetreasSpec> {
        DogW::new(self, 1)
    }
    #[doc = "Bit 2 - Reset due to wakeup from System OFF mode, when wakeup is triggered by DETECT signal from GPIO"]
    #[inline(always)]
    #[must_use]
    pub fn off(&mut self) -> OffW<ResetreasSpec> {
        OffW::new(self, 2)
    }
    #[doc = "Bit 4 - Reset due to wakeup from System OFF mode, when wakeup is triggered by entering debug interface mode"]
    #[inline(always)]
    #[must_use]
    pub fn dif(&mut self) -> DifW<ResetreasSpec> {
        DifW::new(self, 4)
    }
    #[doc = "Bit 16 - Reset from AIRCR.SYSRESETREQ detected"]
    #[inline(always)]
    #[must_use]
    pub fn sreq(&mut self) -> SreqW<ResetreasSpec> {
        SreqW::new(self, 16)
    }
    #[doc = "Bit 17 - Reset from CPU lock-up detected"]
    #[inline(always)]
    #[must_use]
    pub fn lockup(&mut self) -> LockupW<ResetreasSpec> {
        LockupW::new(self, 17)
    }
    #[doc = "Bit 18 - Reset triggered through CTRL-AP"]
    #[inline(always)]
    #[must_use]
    pub fn ctrlap(&mut self) -> CtrlapW<ResetreasSpec> {
        CtrlapW::new(self, 18)
    }
}
#[doc = "Reset reason\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resetreas::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`resetreas::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ResetreasSpec;
impl crate::RegisterSpec for ResetreasSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resetreas::R`](R) reader structure"]
impl crate::Readable for ResetreasSpec {}
#[doc = "`write(|w| ..)` method takes [`resetreas::W`](W) writer structure"]
impl crate::Writable for ResetreasSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RESETREAS to value 0"]
impl crate::Resettable for ResetreasSpec {
    const RESET_VALUE: u32 = 0;
}
