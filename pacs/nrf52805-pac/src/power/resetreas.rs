#[doc = "Register `RESETREAS` reader"]
pub type R = crate::R<ResetreasSpec>;
#[doc = "Register `RESETREAS` writer"]
pub type W = crate::W<ResetreasSpec>;
#[doc = "Reset from pin-reset detected\n\nValue on reset: 0"]
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
#[doc = "Field `RESETPIN` reader - Reset from pin-reset detected"]
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
#[doc = "Field `RESETPIN` writer - Reset from pin-reset detected"]
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
#[doc = "Reset from watchdog detected\n\nValue on reset: 0"]
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
#[doc = "Field `DOG` reader - Reset from watchdog detected"]
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
#[doc = "Field `DOG` writer - Reset from watchdog detected"]
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
#[doc = "Reset from soft reset detected\n\nValue on reset: 0"]
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
#[doc = "Field `SREQ` reader - Reset from soft reset detected"]
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
#[doc = "Field `SREQ` writer - Reset from soft reset detected"]
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
#[doc = "Reset due to wake up from System OFF mode when wakeup is triggered from DETECT signal from GPIO\n\nValue on reset: 0"]
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
#[doc = "Field `OFF` reader - Reset due to wake up from System OFF mode when wakeup is triggered from DETECT signal from GPIO"]
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
#[doc = "Field `OFF` writer - Reset due to wake up from System OFF mode when wakeup is triggered from DETECT signal from GPIO"]
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
#[doc = "Reset due to wake up from System OFF mode when wakeup is triggered from entering into debug interface mode\n\nValue on reset: 0"]
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
#[doc = "Field `DIF` reader - Reset due to wake up from System OFF mode when wakeup is triggered from entering into debug interface mode"]
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
#[doc = "Field `DIF` writer - Reset due to wake up from System OFF mode when wakeup is triggered from entering into debug interface mode"]
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
impl R {
    #[doc = "Bit 0 - Reset from pin-reset detected"]
    #[inline(always)]
    pub fn resetpin(&self) -> ResetpinR {
        ResetpinR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reset from watchdog detected"]
    #[inline(always)]
    pub fn dog(&self) -> DogR {
        DogR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reset from soft reset detected"]
    #[inline(always)]
    pub fn sreq(&self) -> SreqR {
        SreqR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reset from CPU lock-up detected"]
    #[inline(always)]
    pub fn lockup(&self) -> LockupR {
        LockupR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - Reset due to wake up from System OFF mode when wakeup is triggered from DETECT signal from GPIO"]
    #[inline(always)]
    pub fn off(&self) -> OffR {
        OffR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Reset due to wake up from System OFF mode when wakeup is triggered from entering into debug interface mode"]
    #[inline(always)]
    pub fn dif(&self) -> DifR {
        DifR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reset from pin-reset detected"]
    #[inline(always)]
    pub fn resetpin(&mut self) -> ResetpinW<'_, ResetreasSpec> {
        ResetpinW::new(self, 0)
    }
    #[doc = "Bit 1 - Reset from watchdog detected"]
    #[inline(always)]
    pub fn dog(&mut self) -> DogW<'_, ResetreasSpec> {
        DogW::new(self, 1)
    }
    #[doc = "Bit 2 - Reset from soft reset detected"]
    #[inline(always)]
    pub fn sreq(&mut self) -> SreqW<'_, ResetreasSpec> {
        SreqW::new(self, 2)
    }
    #[doc = "Bit 3 - Reset from CPU lock-up detected"]
    #[inline(always)]
    pub fn lockup(&mut self) -> LockupW<'_, ResetreasSpec> {
        LockupW::new(self, 3)
    }
    #[doc = "Bit 16 - Reset due to wake up from System OFF mode when wakeup is triggered from DETECT signal from GPIO"]
    #[inline(always)]
    pub fn off(&mut self) -> OffW<'_, ResetreasSpec> {
        OffW::new(self, 16)
    }
    #[doc = "Bit 18 - Reset due to wake up from System OFF mode when wakeup is triggered from entering into debug interface mode"]
    #[inline(always)]
    pub fn dif(&mut self) -> DifW<'_, ResetreasSpec> {
        DifW::new(self, 18)
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
