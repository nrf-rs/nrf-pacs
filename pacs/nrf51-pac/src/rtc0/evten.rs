#[doc = "Register `EVTEN` reader"]
pub type R = crate::R<EvtenSpec>;
#[doc = "Register `EVTEN` writer"]
pub type W = crate::W<EvtenSpec>;
#[doc = "TICK event enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tick {
    #[doc = "0: Event disabled."]
    Disabled = 0,
    #[doc = "1: Event enabled."]
    Enabled = 1,
}
impl From<Tick> for bool {
    #[inline(always)]
    fn from(variant: Tick) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TICK` reader - TICK event enable."]
pub type TickR = crate::BitReader<Tick>;
impl TickR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tick {
        match self.bits {
            false => Tick::Disabled,
            true => Tick::Enabled,
        }
    }
    #[doc = "Event disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Tick::Disabled
    }
    #[doc = "Event enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Tick::Enabled
    }
}
#[doc = "Field `TICK` writer - TICK event enable."]
pub type TickW<'a, REG> = crate::BitWriter<'a, REG, Tick>;
impl<'a, REG> TickW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Tick::Disabled)
    }
    #[doc = "Event enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Tick::Enabled)
    }
}
#[doc = "OVRFLW event enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ovrflw {
    #[doc = "0: Event disabled."]
    Disabled = 0,
    #[doc = "1: Event enabled."]
    Enabled = 1,
}
impl From<Ovrflw> for bool {
    #[inline(always)]
    fn from(variant: Ovrflw) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVRFLW` reader - OVRFLW event enable."]
pub type OvrflwR = crate::BitReader<Ovrflw>;
impl OvrflwR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ovrflw {
        match self.bits {
            false => Ovrflw::Disabled,
            true => Ovrflw::Enabled,
        }
    }
    #[doc = "Event disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ovrflw::Disabled
    }
    #[doc = "Event enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ovrflw::Enabled
    }
}
#[doc = "Field `OVRFLW` writer - OVRFLW event enable."]
pub type OvrflwW<'a, REG> = crate::BitWriter<'a, REG, Ovrflw>;
impl<'a, REG> OvrflwW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ovrflw::Disabled)
    }
    #[doc = "Event enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ovrflw::Enabled)
    }
}
#[doc = "COMPARE\\[0\\] event enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compare0 {
    #[doc = "0: Event disabled."]
    Disabled = 0,
    #[doc = "1: Event enabled."]
    Enabled = 1,
}
impl From<Compare0> for bool {
    #[inline(always)]
    fn from(variant: Compare0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE0` reader - COMPARE\\[0\\] event enable."]
pub type Compare0R = crate::BitReader<Compare0>;
impl Compare0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Compare0 {
        match self.bits {
            false => Compare0::Disabled,
            true => Compare0::Enabled,
        }
    }
    #[doc = "Event disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Compare0::Disabled
    }
    #[doc = "Event enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Compare0::Enabled
    }
}
#[doc = "Field `COMPARE0` writer - COMPARE\\[0\\] event enable."]
pub type Compare0W<'a, REG> = crate::BitWriter<'a, REG, Compare0>;
impl<'a, REG> Compare0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Compare0::Disabled)
    }
    #[doc = "Event enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Compare0::Enabled)
    }
}
#[doc = "COMPARE\\[1\\] event enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compare1 {
    #[doc = "0: Event disabled."]
    Disabled = 0,
    #[doc = "1: Event enabled."]
    Enabled = 1,
}
impl From<Compare1> for bool {
    #[inline(always)]
    fn from(variant: Compare1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE1` reader - COMPARE\\[1\\] event enable."]
pub type Compare1R = crate::BitReader<Compare1>;
impl Compare1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Compare1 {
        match self.bits {
            false => Compare1::Disabled,
            true => Compare1::Enabled,
        }
    }
    #[doc = "Event disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Compare1::Disabled
    }
    #[doc = "Event enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Compare1::Enabled
    }
}
#[doc = "Field `COMPARE1` writer - COMPARE\\[1\\] event enable."]
pub type Compare1W<'a, REG> = crate::BitWriter<'a, REG, Compare1>;
impl<'a, REG> Compare1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Compare1::Disabled)
    }
    #[doc = "Event enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Compare1::Enabled)
    }
}
#[doc = "COMPARE\\[2\\] event enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compare2 {
    #[doc = "0: Event disabled."]
    Disabled = 0,
    #[doc = "1: Event enabled."]
    Enabled = 1,
}
impl From<Compare2> for bool {
    #[inline(always)]
    fn from(variant: Compare2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE2` reader - COMPARE\\[2\\] event enable."]
pub type Compare2R = crate::BitReader<Compare2>;
impl Compare2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Compare2 {
        match self.bits {
            false => Compare2::Disabled,
            true => Compare2::Enabled,
        }
    }
    #[doc = "Event disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Compare2::Disabled
    }
    #[doc = "Event enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Compare2::Enabled
    }
}
#[doc = "Field `COMPARE2` writer - COMPARE\\[2\\] event enable."]
pub type Compare2W<'a, REG> = crate::BitWriter<'a, REG, Compare2>;
impl<'a, REG> Compare2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Compare2::Disabled)
    }
    #[doc = "Event enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Compare2::Enabled)
    }
}
#[doc = "COMPARE\\[3\\] event enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compare3 {
    #[doc = "0: Event disabled."]
    Disabled = 0,
    #[doc = "1: Event enabled."]
    Enabled = 1,
}
impl From<Compare3> for bool {
    #[inline(always)]
    fn from(variant: Compare3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE3` reader - COMPARE\\[3\\] event enable."]
pub type Compare3R = crate::BitReader<Compare3>;
impl Compare3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Compare3 {
        match self.bits {
            false => Compare3::Disabled,
            true => Compare3::Enabled,
        }
    }
    #[doc = "Event disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Compare3::Disabled
    }
    #[doc = "Event enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Compare3::Enabled
    }
}
#[doc = "Field `COMPARE3` writer - COMPARE\\[3\\] event enable."]
pub type Compare3W<'a, REG> = crate::BitWriter<'a, REG, Compare3>;
impl<'a, REG> Compare3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Compare3::Disabled)
    }
    #[doc = "Event enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Compare3::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - TICK event enable."]
    #[inline(always)]
    pub fn tick(&self) -> TickR {
        TickR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OVRFLW event enable."]
    #[inline(always)]
    pub fn ovrflw(&self) -> OvrflwR {
        OvrflwR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - COMPARE\\[0\\] event enable."]
    #[inline(always)]
    pub fn compare0(&self) -> Compare0R {
        Compare0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - COMPARE\\[1\\] event enable."]
    #[inline(always)]
    pub fn compare1(&self) -> Compare1R {
        Compare1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - COMPARE\\[2\\] event enable."]
    #[inline(always)]
    pub fn compare2(&self) -> Compare2R {
        Compare2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - COMPARE\\[3\\] event enable."]
    #[inline(always)]
    pub fn compare3(&self) -> Compare3R {
        Compare3R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TICK event enable."]
    #[inline(always)]
    pub fn tick(&mut self) -> TickW<'_, EvtenSpec> {
        TickW::new(self, 0)
    }
    #[doc = "Bit 1 - OVRFLW event enable."]
    #[inline(always)]
    pub fn ovrflw(&mut self) -> OvrflwW<'_, EvtenSpec> {
        OvrflwW::new(self, 1)
    }
    #[doc = "Bit 16 - COMPARE\\[0\\] event enable."]
    #[inline(always)]
    pub fn compare0(&mut self) -> Compare0W<'_, EvtenSpec> {
        Compare0W::new(self, 16)
    }
    #[doc = "Bit 17 - COMPARE\\[1\\] event enable."]
    #[inline(always)]
    pub fn compare1(&mut self) -> Compare1W<'_, EvtenSpec> {
        Compare1W::new(self, 17)
    }
    #[doc = "Bit 18 - COMPARE\\[2\\] event enable."]
    #[inline(always)]
    pub fn compare2(&mut self) -> Compare2W<'_, EvtenSpec> {
        Compare2W::new(self, 18)
    }
    #[doc = "Bit 19 - COMPARE\\[3\\] event enable."]
    #[inline(always)]
    pub fn compare3(&mut self) -> Compare3W<'_, EvtenSpec> {
        Compare3W::new(self, 19)
    }
}
#[doc = "Configures event enable routing to PPI for each RTC event.\n\nYou can [`read`](crate::Reg::read) this register and get [`evten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvtenSpec;
impl crate::RegisterSpec for EvtenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evten::R`](R) reader structure"]
impl crate::Readable for EvtenSpec {}
#[doc = "`write(|w| ..)` method takes [`evten::W`](W) writer structure"]
impl crate::Writable for EvtenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVTEN to value 0"]
impl crate::Resettable for EvtenSpec {}
