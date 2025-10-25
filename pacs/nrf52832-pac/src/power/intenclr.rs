#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<IntenclrSpec>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
#[doc = "Write '1' to Disable interrupt for POFWARN event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pofwarn {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Pofwarn> for bool {
    #[inline(always)]
    fn from(variant: Pofwarn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POFWARN` reader - Write '1' to Disable interrupt for POFWARN event"]
pub type PofwarnR = crate::BitReader<Pofwarn>;
impl PofwarnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pofwarn {
        match self.bits {
            false => Pofwarn::Disabled,
            true => Pofwarn::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pofwarn::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pofwarn::Enabled
    }
}
#[doc = "Write '1' to Disable interrupt for POFWARN event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PofwarnWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<PofwarnWO> for bool {
    #[inline(always)]
    fn from(variant: PofwarnWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POFWARN` writer - Write '1' to Disable interrupt for POFWARN event"]
pub type PofwarnW<'a, REG> = crate::BitWriter<'a, REG, PofwarnWO>;
impl<'a, REG> PofwarnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(PofwarnWO::Clear)
    }
}
#[doc = "Write '1' to Disable interrupt for SLEEPENTER event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sleepenter {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Sleepenter> for bool {
    #[inline(always)]
    fn from(variant: Sleepenter) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLEEPENTER` reader - Write '1' to Disable interrupt for SLEEPENTER event"]
pub type SleepenterR = crate::BitReader<Sleepenter>;
impl SleepenterR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sleepenter {
        match self.bits {
            false => Sleepenter::Disabled,
            true => Sleepenter::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Sleepenter::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Sleepenter::Enabled
    }
}
#[doc = "Write '1' to Disable interrupt for SLEEPENTER event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SleepenterWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<SleepenterWO> for bool {
    #[inline(always)]
    fn from(variant: SleepenterWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLEEPENTER` writer - Write '1' to Disable interrupt for SLEEPENTER event"]
pub type SleepenterW<'a, REG> = crate::BitWriter<'a, REG, SleepenterWO>;
impl<'a, REG> SleepenterW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(SleepenterWO::Clear)
    }
}
#[doc = "Write '1' to Disable interrupt for SLEEPEXIT event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sleepexit {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Sleepexit> for bool {
    #[inline(always)]
    fn from(variant: Sleepexit) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLEEPEXIT` reader - Write '1' to Disable interrupt for SLEEPEXIT event"]
pub type SleepexitR = crate::BitReader<Sleepexit>;
impl SleepexitR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sleepexit {
        match self.bits {
            false => Sleepexit::Disabled,
            true => Sleepexit::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Sleepexit::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Sleepexit::Enabled
    }
}
#[doc = "Write '1' to Disable interrupt for SLEEPEXIT event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SleepexitWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<SleepexitWO> for bool {
    #[inline(always)]
    fn from(variant: SleepexitWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLEEPEXIT` writer - Write '1' to Disable interrupt for SLEEPEXIT event"]
pub type SleepexitW<'a, REG> = crate::BitWriter<'a, REG, SleepexitWO>;
impl<'a, REG> SleepexitW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(SleepexitWO::Clear)
    }
}
impl R {
    #[doc = "Bit 2 - Write '1' to Disable interrupt for POFWARN event"]
    #[inline(always)]
    pub fn pofwarn(&self) -> PofwarnR {
        PofwarnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Write '1' to Disable interrupt for SLEEPENTER event"]
    #[inline(always)]
    pub fn sleepenter(&self) -> SleepenterR {
        SleepenterR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write '1' to Disable interrupt for SLEEPEXIT event"]
    #[inline(always)]
    pub fn sleepexit(&self) -> SleepexitR {
        SleepexitR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Write '1' to Disable interrupt for POFWARN event"]
    #[inline(always)]
    pub fn pofwarn(&mut self) -> PofwarnW<'_, IntenclrSpec> {
        PofwarnW::new(self, 2)
    }
    #[doc = "Bit 5 - Write '1' to Disable interrupt for SLEEPENTER event"]
    #[inline(always)]
    pub fn sleepenter(&mut self) -> SleepenterW<'_, IntenclrSpec> {
        SleepenterW::new(self, 5)
    }
    #[doc = "Bit 6 - Write '1' to Disable interrupt for SLEEPEXIT event"]
    #[inline(always)]
    pub fn sleepexit(&mut self) -> SleepexitW<'_, IntenclrSpec> {
        SleepexitW::new(self, 6)
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
