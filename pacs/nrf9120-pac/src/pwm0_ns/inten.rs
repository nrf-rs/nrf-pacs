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
#[doc = "Enable or disable interrupt for event SEQSTARTED\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Seqstarted0 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Seqstarted0> for bool {
    #[inline(always)]
    fn from(variant: Seqstarted0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEQSTARTED0` reader - Enable or disable interrupt for event SEQSTARTED\\[0\\]"]
pub type Seqstarted0R = crate::BitReader<Seqstarted0>;
impl Seqstarted0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Seqstarted0 {
        match self.bits {
            false => Seqstarted0::Disabled,
            true => Seqstarted0::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Seqstarted0::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Seqstarted0::Enabled
    }
}
#[doc = "Field `SEQSTARTED0` writer - Enable or disable interrupt for event SEQSTARTED\\[0\\]"]
pub type Seqstarted0W<'a, REG> = crate::BitWriter<'a, REG, Seqstarted0>;
impl<'a, REG> Seqstarted0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Seqstarted0::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Seqstarted0::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event SEQSTARTED\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Seqstarted1 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Seqstarted1> for bool {
    #[inline(always)]
    fn from(variant: Seqstarted1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEQSTARTED1` reader - Enable or disable interrupt for event SEQSTARTED\\[1\\]"]
pub type Seqstarted1R = crate::BitReader<Seqstarted1>;
impl Seqstarted1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Seqstarted1 {
        match self.bits {
            false => Seqstarted1::Disabled,
            true => Seqstarted1::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Seqstarted1::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Seqstarted1::Enabled
    }
}
#[doc = "Field `SEQSTARTED1` writer - Enable or disable interrupt for event SEQSTARTED\\[1\\]"]
pub type Seqstarted1W<'a, REG> = crate::BitWriter<'a, REG, Seqstarted1>;
impl<'a, REG> Seqstarted1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Seqstarted1::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Seqstarted1::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event SEQEND\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Seqend0 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Seqend0> for bool {
    #[inline(always)]
    fn from(variant: Seqend0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEQEND0` reader - Enable or disable interrupt for event SEQEND\\[0\\]"]
pub type Seqend0R = crate::BitReader<Seqend0>;
impl Seqend0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Seqend0 {
        match self.bits {
            false => Seqend0::Disabled,
            true => Seqend0::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Seqend0::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Seqend0::Enabled
    }
}
#[doc = "Field `SEQEND0` writer - Enable or disable interrupt for event SEQEND\\[0\\]"]
pub type Seqend0W<'a, REG> = crate::BitWriter<'a, REG, Seqend0>;
impl<'a, REG> Seqend0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Seqend0::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Seqend0::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event SEQEND\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Seqend1 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Seqend1> for bool {
    #[inline(always)]
    fn from(variant: Seqend1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEQEND1` reader - Enable or disable interrupt for event SEQEND\\[1\\]"]
pub type Seqend1R = crate::BitReader<Seqend1>;
impl Seqend1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Seqend1 {
        match self.bits {
            false => Seqend1::Disabled,
            true => Seqend1::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Seqend1::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Seqend1::Enabled
    }
}
#[doc = "Field `SEQEND1` writer - Enable or disable interrupt for event SEQEND\\[1\\]"]
pub type Seqend1W<'a, REG> = crate::BitWriter<'a, REG, Seqend1>;
impl<'a, REG> Seqend1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Seqend1::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Seqend1::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event PWMPERIODEND\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwmperiodend {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Pwmperiodend> for bool {
    #[inline(always)]
    fn from(variant: Pwmperiodend) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMPERIODEND` reader - Enable or disable interrupt for event PWMPERIODEND"]
pub type PwmperiodendR = crate::BitReader<Pwmperiodend>;
impl PwmperiodendR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwmperiodend {
        match self.bits {
            false => Pwmperiodend::Disabled,
            true => Pwmperiodend::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pwmperiodend::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pwmperiodend::Enabled
    }
}
#[doc = "Field `PWMPERIODEND` writer - Enable or disable interrupt for event PWMPERIODEND"]
pub type PwmperiodendW<'a, REG> = crate::BitWriter<'a, REG, Pwmperiodend>;
impl<'a, REG> PwmperiodendW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmperiodend::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmperiodend::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event LOOPSDONE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Loopsdone {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Loopsdone> for bool {
    #[inline(always)]
    fn from(variant: Loopsdone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOOPSDONE` reader - Enable or disable interrupt for event LOOPSDONE"]
pub type LoopsdoneR = crate::BitReader<Loopsdone>;
impl LoopsdoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Loopsdone {
        match self.bits {
            false => Loopsdone::Disabled,
            true => Loopsdone::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Loopsdone::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Loopsdone::Enabled
    }
}
#[doc = "Field `LOOPSDONE` writer - Enable or disable interrupt for event LOOPSDONE"]
pub type LoopsdoneW<'a, REG> = crate::BitWriter<'a, REG, Loopsdone>;
impl<'a, REG> LoopsdoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Loopsdone::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Loopsdone::Enabled)
    }
}
impl R {
    #[doc = "Bit 1 - Enable or disable interrupt for event STOPPED"]
    #[inline(always)]
    pub fn stopped(&self) -> StoppedR {
        StoppedR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable or disable interrupt for event SEQSTARTED\\[0\\]"]
    #[inline(always)]
    pub fn seqstarted0(&self) -> Seqstarted0R {
        Seqstarted0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable or disable interrupt for event SEQSTARTED\\[1\\]"]
    #[inline(always)]
    pub fn seqstarted1(&self) -> Seqstarted1R {
        Seqstarted1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable or disable interrupt for event SEQEND\\[0\\]"]
    #[inline(always)]
    pub fn seqend0(&self) -> Seqend0R {
        Seqend0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable or disable interrupt for event SEQEND\\[1\\]"]
    #[inline(always)]
    pub fn seqend1(&self) -> Seqend1R {
        Seqend1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable or disable interrupt for event PWMPERIODEND"]
    #[inline(always)]
    pub fn pwmperiodend(&self) -> PwmperiodendR {
        PwmperiodendR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable or disable interrupt for event LOOPSDONE"]
    #[inline(always)]
    pub fn loopsdone(&self) -> LoopsdoneR {
        LoopsdoneR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Enable or disable interrupt for event STOPPED"]
    #[inline(always)]
    #[must_use]
    pub fn stopped(&mut self) -> StoppedW<IntenSpec> {
        StoppedW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable or disable interrupt for event SEQSTARTED\\[0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn seqstarted0(&mut self) -> Seqstarted0W<IntenSpec> {
        Seqstarted0W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable or disable interrupt for event SEQSTARTED\\[1\\]"]
    #[inline(always)]
    #[must_use]
    pub fn seqstarted1(&mut self) -> Seqstarted1W<IntenSpec> {
        Seqstarted1W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable or disable interrupt for event SEQEND\\[0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn seqend0(&mut self) -> Seqend0W<IntenSpec> {
        Seqend0W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable or disable interrupt for event SEQEND\\[1\\]"]
    #[inline(always)]
    #[must_use]
    pub fn seqend1(&mut self) -> Seqend1W<IntenSpec> {
        Seqend1W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable or disable interrupt for event PWMPERIODEND"]
    #[inline(always)]
    #[must_use]
    pub fn pwmperiodend(&mut self) -> PwmperiodendW<IntenSpec> {
        PwmperiodendW::new(self, 6)
    }
    #[doc = "Bit 7 - Enable or disable interrupt for event LOOPSDONE"]
    #[inline(always)]
    #[must_use]
    pub fn loopsdone(&mut self) -> LoopsdoneW<IntenSpec> {
        LoopsdoneW::new(self, 7)
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
