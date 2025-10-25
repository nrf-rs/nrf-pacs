#[doc = "Register `CURRENTTESTPATTERNMODES` reader"]
pub type R = crate::R<CurrenttestpatternmodesSpec>;
#[doc = "Register `CURRENTTESTPATTERNMODES` writer"]
pub type W = crate::W<CurrenttestpatternmodesSpec>;
#[doc = "Indicates whether the walking 1s pattern is supported as output over the trace port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Patw1 {
    #[doc = "0: Test pattern is disabled."]
    Disabled = 0,
    #[doc = "1: Test pattern is enabled."]
    Enabled = 1,
}
impl From<Patw1> for bool {
    #[inline(always)]
    fn from(variant: Patw1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PATW1` reader - Indicates whether the walking 1s pattern is supported as output over the trace port."]
pub type Patw1R = crate::BitReader<Patw1>;
impl Patw1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Patw1 {
        match self.bits {
            false => Patw1::Disabled,
            true => Patw1::Enabled,
        }
    }
    #[doc = "Test pattern is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Patw1::Disabled
    }
    #[doc = "Test pattern is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Patw1::Enabled
    }
}
#[doc = "Field `PATW1` writer - Indicates whether the walking 1s pattern is supported as output over the trace port."]
pub type Patw1W<'a, REG> = crate::BitWriter<'a, REG, Patw1>;
impl<'a, REG> Patw1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Test pattern is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Patw1::Disabled)
    }
    #[doc = "Test pattern is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Patw1::Enabled)
    }
}
#[doc = "Indicates whether the walking 0s pattern is supported as output over the trace port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Patw0 {
    #[doc = "0: Test pattern is disabled."]
    Disabled = 0,
    #[doc = "1: Test pattern is enabled."]
    Enabled = 1,
}
impl From<Patw0> for bool {
    #[inline(always)]
    fn from(variant: Patw0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PATW0` reader - Indicates whether the walking 0s pattern is supported as output over the trace port."]
pub type Patw0R = crate::BitReader<Patw0>;
impl Patw0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Patw0 {
        match self.bits {
            false => Patw0::Disabled,
            true => Patw0::Enabled,
        }
    }
    #[doc = "Test pattern is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Patw0::Disabled
    }
    #[doc = "Test pattern is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Patw0::Enabled
    }
}
#[doc = "Field `PATW0` writer - Indicates whether the walking 0s pattern is supported as output over the trace port."]
pub type Patw0W<'a, REG> = crate::BitWriter<'a, REG, Patw0>;
impl<'a, REG> Patw0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Test pattern is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Patw0::Disabled)
    }
    #[doc = "Test pattern is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Patw0::Enabled)
    }
}
#[doc = "Indicates whether the AA/55 pattern is supported as output over the trace port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pata5 {
    #[doc = "0: Test pattern is disabled."]
    Disabled = 0,
    #[doc = "1: Test pattern is enabled."]
    Enabled = 1,
}
impl From<Pata5> for bool {
    #[inline(always)]
    fn from(variant: Pata5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PATA5` reader - Indicates whether the AA/55 pattern is supported as output over the trace port."]
pub type Pata5R = crate::BitReader<Pata5>;
impl Pata5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pata5 {
        match self.bits {
            false => Pata5::Disabled,
            true => Pata5::Enabled,
        }
    }
    #[doc = "Test pattern is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pata5::Disabled
    }
    #[doc = "Test pattern is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pata5::Enabled
    }
}
#[doc = "Field `PATA5` writer - Indicates whether the AA/55 pattern is supported as output over the trace port."]
pub type Pata5W<'a, REG> = crate::BitWriter<'a, REG, Pata5>;
impl<'a, REG> Pata5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Test pattern is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pata5::Disabled)
    }
    #[doc = "Test pattern is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pata5::Enabled)
    }
}
#[doc = "Indicates whether the FF/00 pattern is supported as output over the trace port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Patf0 {
    #[doc = "0: Test pattern is disabled."]
    Disabled = 0,
    #[doc = "1: Test pattern is enabled."]
    Enabled = 1,
}
impl From<Patf0> for bool {
    #[inline(always)]
    fn from(variant: Patf0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PATF0` reader - Indicates whether the FF/00 pattern is supported as output over the trace port."]
pub type Patf0R = crate::BitReader<Patf0>;
impl Patf0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Patf0 {
        match self.bits {
            false => Patf0::Disabled,
            true => Patf0::Enabled,
        }
    }
    #[doc = "Test pattern is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Patf0::Disabled
    }
    #[doc = "Test pattern is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Patf0::Enabled
    }
}
#[doc = "Field `PATF0` writer - Indicates whether the FF/00 pattern is supported as output over the trace port."]
pub type Patf0W<'a, REG> = crate::BitWriter<'a, REG, Patf0>;
impl<'a, REG> Patf0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Test pattern is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Patf0::Disabled)
    }
    #[doc = "Test pattern is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Patf0::Enabled)
    }
}
#[doc = "Indicates whether timed mode is supported.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ptimeen {
    #[doc = "0: Mode is disabled."]
    Disabled = 0,
    #[doc = "1: Mode is enabled."]
    Enabled = 1,
}
impl From<Ptimeen> for bool {
    #[inline(always)]
    fn from(variant: Ptimeen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTIMEEN` reader - Indicates whether timed mode is supported."]
pub type PtimeenR = crate::BitReader<Ptimeen>;
impl PtimeenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ptimeen {
        match self.bits {
            false => Ptimeen::Disabled,
            true => Ptimeen::Enabled,
        }
    }
    #[doc = "Mode is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ptimeen::Disabled
    }
    #[doc = "Mode is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ptimeen::Enabled
    }
}
#[doc = "Field `PTIMEEN` writer - Indicates whether timed mode is supported."]
pub type PtimeenW<'a, REG> = crate::BitWriter<'a, REG, Ptimeen>;
impl<'a, REG> PtimeenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Mode is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ptimeen::Disabled)
    }
    #[doc = "Mode is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ptimeen::Enabled)
    }
}
#[doc = "Indicates whether continuous mode is supported.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pconten {
    #[doc = "0: Mode is disabled."]
    Disabled = 0,
    #[doc = "1: Mode is enabled."]
    Enabled = 1,
}
impl From<Pconten> for bool {
    #[inline(always)]
    fn from(variant: Pconten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCONTEN` reader - Indicates whether continuous mode is supported."]
pub type PcontenR = crate::BitReader<Pconten>;
impl PcontenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pconten {
        match self.bits {
            false => Pconten::Disabled,
            true => Pconten::Enabled,
        }
    }
    #[doc = "Mode is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pconten::Disabled
    }
    #[doc = "Mode is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pconten::Enabled
    }
}
#[doc = "Field `PCONTEN` writer - Indicates whether continuous mode is supported."]
pub type PcontenW<'a, REG> = crate::BitWriter<'a, REG, Pconten>;
impl<'a, REG> PcontenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Mode is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pconten::Disabled)
    }
    #[doc = "Mode is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pconten::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Indicates whether the walking 1s pattern is supported as output over the trace port."]
    #[inline(always)]
    pub fn patw1(&self) -> Patw1R {
        Patw1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates whether the walking 0s pattern is supported as output over the trace port."]
    #[inline(always)]
    pub fn patw0(&self) -> Patw0R {
        Patw0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Indicates whether the AA/55 pattern is supported as output over the trace port."]
    #[inline(always)]
    pub fn pata5(&self) -> Pata5R {
        Pata5R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Indicates whether the FF/00 pattern is supported as output over the trace port."]
    #[inline(always)]
    pub fn patf0(&self) -> Patf0R {
        Patf0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - Indicates whether timed mode is supported."]
    #[inline(always)]
    pub fn ptimeen(&self) -> PtimeenR {
        PtimeenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Indicates whether continuous mode is supported."]
    #[inline(always)]
    pub fn pconten(&self) -> PcontenR {
        PcontenR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates whether the walking 1s pattern is supported as output over the trace port."]
    #[inline(always)]
    pub fn patw1(&mut self) -> Patw1W<'_, CurrenttestpatternmodesSpec> {
        Patw1W::new(self, 0)
    }
    #[doc = "Bit 1 - Indicates whether the walking 0s pattern is supported as output over the trace port."]
    #[inline(always)]
    pub fn patw0(&mut self) -> Patw0W<'_, CurrenttestpatternmodesSpec> {
        Patw0W::new(self, 1)
    }
    #[doc = "Bit 2 - Indicates whether the AA/55 pattern is supported as output over the trace port."]
    #[inline(always)]
    pub fn pata5(&mut self) -> Pata5W<'_, CurrenttestpatternmodesSpec> {
        Pata5W::new(self, 2)
    }
    #[doc = "Bit 3 - Indicates whether the FF/00 pattern is supported as output over the trace port."]
    #[inline(always)]
    pub fn patf0(&mut self) -> Patf0W<'_, CurrenttestpatternmodesSpec> {
        Patf0W::new(self, 3)
    }
    #[doc = "Bit 16 - Indicates whether timed mode is supported."]
    #[inline(always)]
    pub fn ptimeen(&mut self) -> PtimeenW<'_, CurrenttestpatternmodesSpec> {
        PtimeenW::new(self, 16)
    }
    #[doc = "Bit 17 - Indicates whether continuous mode is supported."]
    #[inline(always)]
    pub fn pconten(&mut self) -> PcontenW<'_, CurrenttestpatternmodesSpec> {
        PcontenW::new(self, 17)
    }
}
#[doc = "Current_test_pattern_mode indicates the current test pattern or mode selected.\n\nYou can [`read`](crate::Reg::read) this register and get [`currenttestpatternmodes::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`currenttestpatternmodes::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CurrenttestpatternmodesSpec;
impl crate::RegisterSpec for CurrenttestpatternmodesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`currenttestpatternmodes::R`](R) reader structure"]
impl crate::Readable for CurrenttestpatternmodesSpec {}
#[doc = "`write(|w| ..)` method takes [`currenttestpatternmodes::W`](W) writer structure"]
impl crate::Writable for CurrenttestpatternmodesSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CURRENTTESTPATTERNMODES to value 0"]
impl crate::Resettable for CurrenttestpatternmodesSpec {}
