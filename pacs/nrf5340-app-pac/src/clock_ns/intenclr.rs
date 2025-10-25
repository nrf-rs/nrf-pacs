#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<IntenclrSpec>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
#[doc = "Write '1' to disable interrupt for event HFCLKSTARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hfclkstarted {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Hfclkstarted> for bool {
    #[inline(always)]
    fn from(variant: Hfclkstarted) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HFCLKSTARTED` reader - Write '1' to disable interrupt for event HFCLKSTARTED"]
pub type HfclkstartedR = crate::BitReader<Hfclkstarted>;
impl HfclkstartedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hfclkstarted {
        match self.bits {
            false => Hfclkstarted::Disabled,
            true => Hfclkstarted::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Hfclkstarted::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Hfclkstarted::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event HFCLKSTARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HfclkstartedWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<HfclkstartedWO> for bool {
    #[inline(always)]
    fn from(variant: HfclkstartedWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HFCLKSTARTED` writer - Write '1' to disable interrupt for event HFCLKSTARTED"]
pub type HfclkstartedW<'a, REG> = crate::BitWriter<'a, REG, HfclkstartedWO>;
impl<'a, REG> HfclkstartedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(HfclkstartedWO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event LFCLKSTARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lfclkstarted {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Lfclkstarted> for bool {
    #[inline(always)]
    fn from(variant: Lfclkstarted) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LFCLKSTARTED` reader - Write '1' to disable interrupt for event LFCLKSTARTED"]
pub type LfclkstartedR = crate::BitReader<Lfclkstarted>;
impl LfclkstartedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lfclkstarted {
        match self.bits {
            false => Lfclkstarted::Disabled,
            true => Lfclkstarted::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Lfclkstarted::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Lfclkstarted::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event LFCLKSTARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LfclkstartedWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<LfclkstartedWO> for bool {
    #[inline(always)]
    fn from(variant: LfclkstartedWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LFCLKSTARTED` writer - Write '1' to disable interrupt for event LFCLKSTARTED"]
pub type LfclkstartedW<'a, REG> = crate::BitWriter<'a, REG, LfclkstartedWO>;
impl<'a, REG> LfclkstartedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(LfclkstartedWO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event DONE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Done {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Done> for bool {
    #[inline(always)]
    fn from(variant: Done) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DONE` reader - Write '1' to disable interrupt for event DONE"]
pub type DoneR = crate::BitReader<Done>;
impl DoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Done {
        match self.bits {
            false => Done::Disabled,
            true => Done::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Done::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Done::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event DONE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DoneWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<DoneWO> for bool {
    #[inline(always)]
    fn from(variant: DoneWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DONE` writer - Write '1' to disable interrupt for event DONE"]
pub type DoneW<'a, REG> = crate::BitWriter<'a, REG, DoneWO>;
impl<'a, REG> DoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(DoneWO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event HFCLKAUDIOSTARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hfclkaudiostarted {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Hfclkaudiostarted> for bool {
    #[inline(always)]
    fn from(variant: Hfclkaudiostarted) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HFCLKAUDIOSTARTED` reader - Write '1' to disable interrupt for event HFCLKAUDIOSTARTED"]
pub type HfclkaudiostartedR = crate::BitReader<Hfclkaudiostarted>;
impl HfclkaudiostartedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hfclkaudiostarted {
        match self.bits {
            false => Hfclkaudiostarted::Disabled,
            true => Hfclkaudiostarted::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Hfclkaudiostarted::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Hfclkaudiostarted::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event HFCLKAUDIOSTARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HfclkaudiostartedWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<HfclkaudiostartedWO> for bool {
    #[inline(always)]
    fn from(variant: HfclkaudiostartedWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HFCLKAUDIOSTARTED` writer - Write '1' to disable interrupt for event HFCLKAUDIOSTARTED"]
pub type HfclkaudiostartedW<'a, REG> = crate::BitWriter<'a, REG, HfclkaudiostartedWO>;
impl<'a, REG> HfclkaudiostartedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(HfclkaudiostartedWO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event HFCLK192MSTARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hfclk192mstarted {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Hfclk192mstarted> for bool {
    #[inline(always)]
    fn from(variant: Hfclk192mstarted) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HFCLK192MSTARTED` reader - Write '1' to disable interrupt for event HFCLK192MSTARTED"]
pub type Hfclk192mstartedR = crate::BitReader<Hfclk192mstarted>;
impl Hfclk192mstartedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hfclk192mstarted {
        match self.bits {
            false => Hfclk192mstarted::Disabled,
            true => Hfclk192mstarted::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Hfclk192mstarted::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Hfclk192mstarted::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event HFCLK192MSTARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hfclk192mstartedWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<Hfclk192mstartedWO> for bool {
    #[inline(always)]
    fn from(variant: Hfclk192mstartedWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HFCLK192MSTARTED` writer - Write '1' to disable interrupt for event HFCLK192MSTARTED"]
pub type Hfclk192mstartedW<'a, REG> = crate::BitWriter<'a, REG, Hfclk192mstartedWO>;
impl<'a, REG> Hfclk192mstartedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Hfclk192mstartedWO::Clear)
    }
}
impl R {
    #[doc = "Bit 0 - Write '1' to disable interrupt for event HFCLKSTARTED"]
    #[inline(always)]
    pub fn hfclkstarted(&self) -> HfclkstartedR {
        HfclkstartedR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write '1' to disable interrupt for event LFCLKSTARTED"]
    #[inline(always)]
    pub fn lfclkstarted(&self) -> LfclkstartedR {
        LfclkstartedR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 7 - Write '1' to disable interrupt for event DONE"]
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Write '1' to disable interrupt for event HFCLKAUDIOSTARTED"]
    #[inline(always)]
    pub fn hfclkaudiostarted(&self) -> HfclkaudiostartedR {
        HfclkaudiostartedR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Write '1' to disable interrupt for event HFCLK192MSTARTED"]
    #[inline(always)]
    pub fn hfclk192mstarted(&self) -> Hfclk192mstartedR {
        Hfclk192mstartedR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to disable interrupt for event HFCLKSTARTED"]
    #[inline(always)]
    pub fn hfclkstarted(&mut self) -> HfclkstartedW<'_, IntenclrSpec> {
        HfclkstartedW::new(self, 0)
    }
    #[doc = "Bit 1 - Write '1' to disable interrupt for event LFCLKSTARTED"]
    #[inline(always)]
    pub fn lfclkstarted(&mut self) -> LfclkstartedW<'_, IntenclrSpec> {
        LfclkstartedW::new(self, 1)
    }
    #[doc = "Bit 7 - Write '1' to disable interrupt for event DONE"]
    #[inline(always)]
    pub fn done(&mut self) -> DoneW<'_, IntenclrSpec> {
        DoneW::new(self, 7)
    }
    #[doc = "Bit 8 - Write '1' to disable interrupt for event HFCLKAUDIOSTARTED"]
    #[inline(always)]
    pub fn hfclkaudiostarted(&mut self) -> HfclkaudiostartedW<'_, IntenclrSpec> {
        HfclkaudiostartedW::new(self, 8)
    }
    #[doc = "Bit 9 - Write '1' to disable interrupt for event HFCLK192MSTARTED"]
    #[inline(always)]
    pub fn hfclk192mstarted(&mut self) -> Hfclk192mstartedW<'_, IntenclrSpec> {
        Hfclk192mstartedW::new(self, 9)
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
