#[doc = "Register `SHORTS` reader"]
pub type R = crate::R<ShortsSpec>;
#[doc = "Register `SHORTS` writer"]
pub type W = crate::W<ShortsSpec>;
#[doc = "Shortcut between event SEQEND\\[0\\]
and task STOP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Seqend0Stop {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<Seqend0Stop> for bool {
    #[inline(always)]
    fn from(variant: Seqend0Stop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEQEND0_STOP` reader - Shortcut between event SEQEND\\[0\\]
and task STOP"]
pub type Seqend0StopR = crate::BitReader<Seqend0Stop>;
impl Seqend0StopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Seqend0Stop {
        match self.bits {
            false => Seqend0Stop::Disabled,
            true => Seqend0Stop::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Seqend0Stop::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Seqend0Stop::Enabled
    }
}
#[doc = "Field `SEQEND0_STOP` writer - Shortcut between event SEQEND\\[0\\]
and task STOP"]
pub type Seqend0StopW<'a, REG> = crate::BitWriter<'a, REG, Seqend0Stop>;
impl<'a, REG> Seqend0StopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Seqend0Stop::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Seqend0Stop::Enabled)
    }
}
#[doc = "Shortcut between event SEQEND\\[1\\]
and task STOP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Seqend1Stop {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<Seqend1Stop> for bool {
    #[inline(always)]
    fn from(variant: Seqend1Stop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEQEND1_STOP` reader - Shortcut between event SEQEND\\[1\\]
and task STOP"]
pub type Seqend1StopR = crate::BitReader<Seqend1Stop>;
impl Seqend1StopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Seqend1Stop {
        match self.bits {
            false => Seqend1Stop::Disabled,
            true => Seqend1Stop::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Seqend1Stop::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Seqend1Stop::Enabled
    }
}
#[doc = "Field `SEQEND1_STOP` writer - Shortcut between event SEQEND\\[1\\]
and task STOP"]
pub type Seqend1StopW<'a, REG> = crate::BitWriter<'a, REG, Seqend1Stop>;
impl<'a, REG> Seqend1StopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Seqend1Stop::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Seqend1Stop::Enabled)
    }
}
#[doc = "Shortcut between event LOOPSDONE and task SEQSTART\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LoopsdoneSeqstart0 {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<LoopsdoneSeqstart0> for bool {
    #[inline(always)]
    fn from(variant: LoopsdoneSeqstart0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOOPSDONE_SEQSTART0` reader - Shortcut between event LOOPSDONE and task SEQSTART\\[0\\]"]
pub type LoopsdoneSeqstart0R = crate::BitReader<LoopsdoneSeqstart0>;
impl LoopsdoneSeqstart0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LoopsdoneSeqstart0 {
        match self.bits {
            false => LoopsdoneSeqstart0::Disabled,
            true => LoopsdoneSeqstart0::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LoopsdoneSeqstart0::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LoopsdoneSeqstart0::Enabled
    }
}
#[doc = "Field `LOOPSDONE_SEQSTART0` writer - Shortcut between event LOOPSDONE and task SEQSTART\\[0\\]"]
pub type LoopsdoneSeqstart0W<'a, REG> = crate::BitWriter<'a, REG, LoopsdoneSeqstart0>;
impl<'a, REG> LoopsdoneSeqstart0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LoopsdoneSeqstart0::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LoopsdoneSeqstart0::Enabled)
    }
}
#[doc = "Shortcut between event LOOPSDONE and task SEQSTART\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LoopsdoneSeqstart1 {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<LoopsdoneSeqstart1> for bool {
    #[inline(always)]
    fn from(variant: LoopsdoneSeqstart1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOOPSDONE_SEQSTART1` reader - Shortcut between event LOOPSDONE and task SEQSTART\\[1\\]"]
pub type LoopsdoneSeqstart1R = crate::BitReader<LoopsdoneSeqstart1>;
impl LoopsdoneSeqstart1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LoopsdoneSeqstart1 {
        match self.bits {
            false => LoopsdoneSeqstart1::Disabled,
            true => LoopsdoneSeqstart1::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LoopsdoneSeqstart1::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LoopsdoneSeqstart1::Enabled
    }
}
#[doc = "Field `LOOPSDONE_SEQSTART1` writer - Shortcut between event LOOPSDONE and task SEQSTART\\[1\\]"]
pub type LoopsdoneSeqstart1W<'a, REG> = crate::BitWriter<'a, REG, LoopsdoneSeqstart1>;
impl<'a, REG> LoopsdoneSeqstart1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LoopsdoneSeqstart1::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LoopsdoneSeqstart1::Enabled)
    }
}
#[doc = "Shortcut between event LOOPSDONE and task STOP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LoopsdoneStop {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<LoopsdoneStop> for bool {
    #[inline(always)]
    fn from(variant: LoopsdoneStop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOOPSDONE_STOP` reader - Shortcut between event LOOPSDONE and task STOP"]
pub type LoopsdoneStopR = crate::BitReader<LoopsdoneStop>;
impl LoopsdoneStopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LoopsdoneStop {
        match self.bits {
            false => LoopsdoneStop::Disabled,
            true => LoopsdoneStop::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LoopsdoneStop::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LoopsdoneStop::Enabled
    }
}
#[doc = "Field `LOOPSDONE_STOP` writer - Shortcut between event LOOPSDONE and task STOP"]
pub type LoopsdoneStopW<'a, REG> = crate::BitWriter<'a, REG, LoopsdoneStop>;
impl<'a, REG> LoopsdoneStopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LoopsdoneStop::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LoopsdoneStop::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Shortcut between event SEQEND\\[0\\]
and task STOP"]
    #[inline(always)]
    pub fn seqend0_stop(&self) -> Seqend0StopR {
        Seqend0StopR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Shortcut between event SEQEND\\[1\\]
and task STOP"]
    #[inline(always)]
    pub fn seqend1_stop(&self) -> Seqend1StopR {
        Seqend1StopR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Shortcut between event LOOPSDONE and task SEQSTART\\[0\\]"]
    #[inline(always)]
    pub fn loopsdone_seqstart0(&self) -> LoopsdoneSeqstart0R {
        LoopsdoneSeqstart0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Shortcut between event LOOPSDONE and task SEQSTART\\[1\\]"]
    #[inline(always)]
    pub fn loopsdone_seqstart1(&self) -> LoopsdoneSeqstart1R {
        LoopsdoneSeqstart1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Shortcut between event LOOPSDONE and task STOP"]
    #[inline(always)]
    pub fn loopsdone_stop(&self) -> LoopsdoneStopR {
        LoopsdoneStopR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Shortcut between event SEQEND\\[0\\]
and task STOP"]
    #[inline(always)]
    #[must_use]
    pub fn seqend0_stop(&mut self) -> Seqend0StopW<ShortsSpec> {
        Seqend0StopW::new(self, 0)
    }
    #[doc = "Bit 1 - Shortcut between event SEQEND\\[1\\]
and task STOP"]
    #[inline(always)]
    #[must_use]
    pub fn seqend1_stop(&mut self) -> Seqend1StopW<ShortsSpec> {
        Seqend1StopW::new(self, 1)
    }
    #[doc = "Bit 2 - Shortcut between event LOOPSDONE and task SEQSTART\\[0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn loopsdone_seqstart0(&mut self) -> LoopsdoneSeqstart0W<ShortsSpec> {
        LoopsdoneSeqstart0W::new(self, 2)
    }
    #[doc = "Bit 3 - Shortcut between event LOOPSDONE and task SEQSTART\\[1\\]"]
    #[inline(always)]
    #[must_use]
    pub fn loopsdone_seqstart1(&mut self) -> LoopsdoneSeqstart1W<ShortsSpec> {
        LoopsdoneSeqstart1W::new(self, 3)
    }
    #[doc = "Bit 4 - Shortcut between event LOOPSDONE and task STOP"]
    #[inline(always)]
    #[must_use]
    pub fn loopsdone_stop(&mut self) -> LoopsdoneStopW<ShortsSpec> {
        LoopsdoneStopW::new(self, 4)
    }
}
#[doc = "Shortcuts between local events and tasks\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shorts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shorts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShortsSpec;
impl crate::RegisterSpec for ShortsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shorts::R`](R) reader structure"]
impl crate::Readable for ShortsSpec {}
#[doc = "`write(|w| ..)` method takes [`shorts::W`](W) writer structure"]
impl crate::Writable for ShortsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHORTS to value 0"]
impl crate::Resettable for ShortsSpec {
    const RESET_VALUE: u32 = 0;
}
