#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<IntenclrSpec>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
#[doc = "Write '1' to disable interrupt for event STOPPED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stopped {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Stopped> for bool {
    #[inline(always)]
    fn from(variant: Stopped) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPPED` reader - Write '1' to disable interrupt for event STOPPED"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Stopped::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Stopped::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event STOPPED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StoppedWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<StoppedWO> for bool {
    #[inline(always)]
    fn from(variant: StoppedWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPPED` writer - Write '1' to disable interrupt for event STOPPED"]
pub type StoppedW<'a, REG> = crate::BitWriter<'a, REG, StoppedWO>;
impl<'a, REG> StoppedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(StoppedWO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event SEQSTARTED\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Seqstarted0 {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Seqstarted0> for bool {
    #[inline(always)]
    fn from(variant: Seqstarted0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEQSTARTED0` reader - Write '1' to disable interrupt for event SEQSTARTED\\[0\\]"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Seqstarted0::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Seqstarted0::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event SEQSTARTED\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Seqstarted0WO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<Seqstarted0WO> for bool {
    #[inline(always)]
    fn from(variant: Seqstarted0WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEQSTARTED0` writer - Write '1' to disable interrupt for event SEQSTARTED\\[0\\]"]
pub type Seqstarted0W<'a, REG> = crate::BitWriter<'a, REG, Seqstarted0WO>;
impl<'a, REG> Seqstarted0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Seqstarted0WO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event SEQSTARTED\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Seqstarted1 {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Seqstarted1> for bool {
    #[inline(always)]
    fn from(variant: Seqstarted1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEQSTARTED1` reader - Write '1' to disable interrupt for event SEQSTARTED\\[1\\]"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Seqstarted1::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Seqstarted1::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event SEQSTARTED\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Seqstarted1WO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<Seqstarted1WO> for bool {
    #[inline(always)]
    fn from(variant: Seqstarted1WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEQSTARTED1` writer - Write '1' to disable interrupt for event SEQSTARTED\\[1\\]"]
pub type Seqstarted1W<'a, REG> = crate::BitWriter<'a, REG, Seqstarted1WO>;
impl<'a, REG> Seqstarted1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Seqstarted1WO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event SEQEND\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Seqend0 {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Seqend0> for bool {
    #[inline(always)]
    fn from(variant: Seqend0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEQEND0` reader - Write '1' to disable interrupt for event SEQEND\\[0\\]"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Seqend0::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Seqend0::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event SEQEND\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Seqend0WO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<Seqend0WO> for bool {
    #[inline(always)]
    fn from(variant: Seqend0WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEQEND0` writer - Write '1' to disable interrupt for event SEQEND\\[0\\]"]
pub type Seqend0W<'a, REG> = crate::BitWriter<'a, REG, Seqend0WO>;
impl<'a, REG> Seqend0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Seqend0WO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event SEQEND\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Seqend1 {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Seqend1> for bool {
    #[inline(always)]
    fn from(variant: Seqend1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEQEND1` reader - Write '1' to disable interrupt for event SEQEND\\[1\\]"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Seqend1::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Seqend1::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event SEQEND\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Seqend1WO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<Seqend1WO> for bool {
    #[inline(always)]
    fn from(variant: Seqend1WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEQEND1` writer - Write '1' to disable interrupt for event SEQEND\\[1\\]"]
pub type Seqend1W<'a, REG> = crate::BitWriter<'a, REG, Seqend1WO>;
impl<'a, REG> Seqend1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Seqend1WO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event PWMPERIODEND\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwmperiodend {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Pwmperiodend> for bool {
    #[inline(always)]
    fn from(variant: Pwmperiodend) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMPERIODEND` reader - Write '1' to disable interrupt for event PWMPERIODEND"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pwmperiodend::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pwmperiodend::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event PWMPERIODEND\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PwmperiodendWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<PwmperiodendWO> for bool {
    #[inline(always)]
    fn from(variant: PwmperiodendWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMPERIODEND` writer - Write '1' to disable interrupt for event PWMPERIODEND"]
pub type PwmperiodendW<'a, REG> = crate::BitWriter<'a, REG, PwmperiodendWO>;
impl<'a, REG> PwmperiodendW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(PwmperiodendWO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event LOOPSDONE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Loopsdone {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Loopsdone> for bool {
    #[inline(always)]
    fn from(variant: Loopsdone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOOPSDONE` reader - Write '1' to disable interrupt for event LOOPSDONE"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Loopsdone::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Loopsdone::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event LOOPSDONE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LoopsdoneWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<LoopsdoneWO> for bool {
    #[inline(always)]
    fn from(variant: LoopsdoneWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOOPSDONE` writer - Write '1' to disable interrupt for event LOOPSDONE"]
pub type LoopsdoneW<'a, REG> = crate::BitWriter<'a, REG, LoopsdoneWO>;
impl<'a, REG> LoopsdoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(LoopsdoneWO::Clear)
    }
}
impl R {
    #[doc = "Bit 1 - Write '1' to disable interrupt for event STOPPED"]
    #[inline(always)]
    pub fn stopped(&self) -> StoppedR {
        StoppedR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write '1' to disable interrupt for event SEQSTARTED\\[0\\]"]
    #[inline(always)]
    pub fn seqstarted0(&self) -> Seqstarted0R {
        Seqstarted0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write '1' to disable interrupt for event SEQSTARTED\\[1\\]"]
    #[inline(always)]
    pub fn seqstarted1(&self) -> Seqstarted1R {
        Seqstarted1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write '1' to disable interrupt for event SEQEND\\[0\\]"]
    #[inline(always)]
    pub fn seqend0(&self) -> Seqend0R {
        Seqend0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write '1' to disable interrupt for event SEQEND\\[1\\]"]
    #[inline(always)]
    pub fn seqend1(&self) -> Seqend1R {
        Seqend1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write '1' to disable interrupt for event PWMPERIODEND"]
    #[inline(always)]
    pub fn pwmperiodend(&self) -> PwmperiodendR {
        PwmperiodendR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write '1' to disable interrupt for event LOOPSDONE"]
    #[inline(always)]
    pub fn loopsdone(&self) -> LoopsdoneR {
        LoopsdoneR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Write '1' to disable interrupt for event STOPPED"]
    #[inline(always)]
    #[must_use]
    pub fn stopped(&mut self) -> StoppedW<IntenclrSpec> {
        StoppedW::new(self, 1)
    }
    #[doc = "Bit 2 - Write '1' to disable interrupt for event SEQSTARTED\\[0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn seqstarted0(&mut self) -> Seqstarted0W<IntenclrSpec> {
        Seqstarted0W::new(self, 2)
    }
    #[doc = "Bit 3 - Write '1' to disable interrupt for event SEQSTARTED\\[1\\]"]
    #[inline(always)]
    #[must_use]
    pub fn seqstarted1(&mut self) -> Seqstarted1W<IntenclrSpec> {
        Seqstarted1W::new(self, 3)
    }
    #[doc = "Bit 4 - Write '1' to disable interrupt for event SEQEND\\[0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn seqend0(&mut self) -> Seqend0W<IntenclrSpec> {
        Seqend0W::new(self, 4)
    }
    #[doc = "Bit 5 - Write '1' to disable interrupt for event SEQEND\\[1\\]"]
    #[inline(always)]
    #[must_use]
    pub fn seqend1(&mut self) -> Seqend1W<IntenclrSpec> {
        Seqend1W::new(self, 5)
    }
    #[doc = "Bit 6 - Write '1' to disable interrupt for event PWMPERIODEND"]
    #[inline(always)]
    #[must_use]
    pub fn pwmperiodend(&mut self) -> PwmperiodendW<IntenclrSpec> {
        PwmperiodendW::new(self, 6)
    }
    #[doc = "Bit 7 - Write '1' to disable interrupt for event LOOPSDONE"]
    #[inline(always)]
    #[must_use]
    pub fn loopsdone(&mut self) -> LoopsdoneW<IntenclrSpec> {
        LoopsdoneW::new(self, 7)
    }
}
#[doc = "Disable interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intenclr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intenclr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenclrSpec;
impl crate::RegisterSpec for IntenclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenclr::R`](R) reader structure"]
impl crate::Readable for IntenclrSpec {}
#[doc = "`write(|w| ..)` method takes [`intenclr::W`](W) writer structure"]
impl crate::Writable for IntenclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for IntenclrSpec {
    const RESET_VALUE: u32 = 0;
}
