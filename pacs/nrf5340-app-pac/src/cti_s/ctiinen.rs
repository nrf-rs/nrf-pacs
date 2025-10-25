#[doc = "Register `CTIINEN[%s]` reader"]
pub type R = crate::R<CtiinenSpec>;
#[doc = "Register `CTIINEN[%s]` writer"]
pub type W = crate::W<CtiinenSpec>;
#[doc = "Enables a cross trigger event to channel 0 when a ctitrigin input is activated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Triginen0 {
    #[doc = "0: Input trigger n events are ignored by channel 0."]
    Disabled = 0,
    #[doc = "1: When an event is received on input trigger n (ctitrigin\\[n\\]), generate an event on channel 0."]
    Enabled = 1,
}
impl From<Triginen0> for bool {
    #[inline(always)]
    fn from(variant: Triginen0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGINEN_0` reader - Enables a cross trigger event to channel 0 when a ctitrigin input is activated."]
pub type Triginen0R = crate::BitReader<Triginen0>;
impl Triginen0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Triginen0 {
        match self.bits {
            false => Triginen0::Disabled,
            true => Triginen0::Enabled,
        }
    }
    #[doc = "Input trigger n events are ignored by channel 0."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Triginen0::Disabled
    }
    #[doc = "When an event is received on input trigger n (ctitrigin\\[n\\]), generate an event on channel 0."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Triginen0::Enabled
    }
}
#[doc = "Field `TRIGINEN_0` writer - Enables a cross trigger event to channel 0 when a ctitrigin input is activated."]
pub type Triginen0W<'a, REG> = crate::BitWriter<'a, REG, Triginen0>;
impl<'a, REG> Triginen0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input trigger n events are ignored by channel 0."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Triginen0::Disabled)
    }
    #[doc = "When an event is received on input trigger n (ctitrigin\\[n\\]), generate an event on channel 0."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Triginen0::Enabled)
    }
}
#[doc = "Enables a cross trigger event to channel 1 when a ctitrigin input is activated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Triginen1 {
    #[doc = "0: Input trigger n events are ignored by channel 1."]
    Disabled = 0,
    #[doc = "1: When an event is received on input trigger n (ctitrigin\\[n\\]), generate an event on channel 1."]
    Enabled = 1,
}
impl From<Triginen1> for bool {
    #[inline(always)]
    fn from(variant: Triginen1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGINEN_1` reader - Enables a cross trigger event to channel 1 when a ctitrigin input is activated."]
pub type Triginen1R = crate::BitReader<Triginen1>;
impl Triginen1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Triginen1 {
        match self.bits {
            false => Triginen1::Disabled,
            true => Triginen1::Enabled,
        }
    }
    #[doc = "Input trigger n events are ignored by channel 1."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Triginen1::Disabled
    }
    #[doc = "When an event is received on input trigger n (ctitrigin\\[n\\]), generate an event on channel 1."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Triginen1::Enabled
    }
}
#[doc = "Field `TRIGINEN_1` writer - Enables a cross trigger event to channel 1 when a ctitrigin input is activated."]
pub type Triginen1W<'a, REG> = crate::BitWriter<'a, REG, Triginen1>;
impl<'a, REG> Triginen1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input trigger n events are ignored by channel 1."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Triginen1::Disabled)
    }
    #[doc = "When an event is received on input trigger n (ctitrigin\\[n\\]), generate an event on channel 1."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Triginen1::Enabled)
    }
}
#[doc = "Enables a cross trigger event to channel 2 when a ctitrigin input is activated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Triginen2 {
    #[doc = "0: Input trigger n events are ignored by channel 2."]
    Disabled = 0,
    #[doc = "1: When an event is received on input trigger n (ctitrigin\\[n\\]), generate an event on channel 2."]
    Enabled = 1,
}
impl From<Triginen2> for bool {
    #[inline(always)]
    fn from(variant: Triginen2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGINEN_2` reader - Enables a cross trigger event to channel 2 when a ctitrigin input is activated."]
pub type Triginen2R = crate::BitReader<Triginen2>;
impl Triginen2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Triginen2 {
        match self.bits {
            false => Triginen2::Disabled,
            true => Triginen2::Enabled,
        }
    }
    #[doc = "Input trigger n events are ignored by channel 2."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Triginen2::Disabled
    }
    #[doc = "When an event is received on input trigger n (ctitrigin\\[n\\]), generate an event on channel 2."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Triginen2::Enabled
    }
}
#[doc = "Field `TRIGINEN_2` writer - Enables a cross trigger event to channel 2 when a ctitrigin input is activated."]
pub type Triginen2W<'a, REG> = crate::BitWriter<'a, REG, Triginen2>;
impl<'a, REG> Triginen2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input trigger n events are ignored by channel 2."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Triginen2::Disabled)
    }
    #[doc = "When an event is received on input trigger n (ctitrigin\\[n\\]), generate an event on channel 2."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Triginen2::Enabled)
    }
}
#[doc = "Enables a cross trigger event to channel 3 when a ctitrigin input is activated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Triginen3 {
    #[doc = "0: Input trigger n events are ignored by channel 3."]
    Disabled = 0,
    #[doc = "1: When an event is received on input trigger n (ctitrigin\\[n\\]), generate an event on channel 3."]
    Enabled = 1,
}
impl From<Triginen3> for bool {
    #[inline(always)]
    fn from(variant: Triginen3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGINEN_3` reader - Enables a cross trigger event to channel 3 when a ctitrigin input is activated."]
pub type Triginen3R = crate::BitReader<Triginen3>;
impl Triginen3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Triginen3 {
        match self.bits {
            false => Triginen3::Disabled,
            true => Triginen3::Enabled,
        }
    }
    #[doc = "Input trigger n events are ignored by channel 3."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Triginen3::Disabled
    }
    #[doc = "When an event is received on input trigger n (ctitrigin\\[n\\]), generate an event on channel 3."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Triginen3::Enabled
    }
}
#[doc = "Field `TRIGINEN_3` writer - Enables a cross trigger event to channel 3 when a ctitrigin input is activated."]
pub type Triginen3W<'a, REG> = crate::BitWriter<'a, REG, Triginen3>;
impl<'a, REG> Triginen3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input trigger n events are ignored by channel 3."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Triginen3::Disabled)
    }
    #[doc = "When an event is received on input trigger n (ctitrigin\\[n\\]), generate an event on channel 3."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Triginen3::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Enables a cross trigger event to channel 0 when a ctitrigin input is activated."]
    #[inline(always)]
    pub fn triginen_0(&self) -> Triginen0R {
        Triginen0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enables a cross trigger event to channel 1 when a ctitrigin input is activated."]
    #[inline(always)]
    pub fn triginen_1(&self) -> Triginen1R {
        Triginen1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enables a cross trigger event to channel 2 when a ctitrigin input is activated."]
    #[inline(always)]
    pub fn triginen_2(&self) -> Triginen2R {
        Triginen2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enables a cross trigger event to channel 3 when a ctitrigin input is activated."]
    #[inline(always)]
    pub fn triginen_3(&self) -> Triginen3R {
        Triginen3R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables a cross trigger event to channel 0 when a ctitrigin input is activated."]
    #[inline(always)]
    pub fn triginen_0(&mut self) -> Triginen0W<'_, CtiinenSpec> {
        Triginen0W::new(self, 0)
    }
    #[doc = "Bit 1 - Enables a cross trigger event to channel 1 when a ctitrigin input is activated."]
    #[inline(always)]
    pub fn triginen_1(&mut self) -> Triginen1W<'_, CtiinenSpec> {
        Triginen1W::new(self, 1)
    }
    #[doc = "Bit 2 - Enables a cross trigger event to channel 2 when a ctitrigin input is activated."]
    #[inline(always)]
    pub fn triginen_2(&mut self) -> Triginen2W<'_, CtiinenSpec> {
        Triginen2W::new(self, 2)
    }
    #[doc = "Bit 3 - Enables a cross trigger event to channel 3 when a ctitrigin input is activated."]
    #[inline(always)]
    pub fn triginen_3(&mut self) -> Triginen3W<'_, CtiinenSpec> {
        Triginen3W::new(self, 3)
    }
}
#[doc = "Description collection: CTI Trigger input\n\nYou can [`read`](crate::Reg::read) this register and get [`ctiinen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctiinen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtiinenSpec;
impl crate::RegisterSpec for CtiinenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctiinen::R`](R) reader structure"]
impl crate::Readable for CtiinenSpec {}
#[doc = "`write(|w| ..)` method takes [`ctiinen::W`](W) writer structure"]
impl crate::Writable for CtiinenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTIINEN[%s] to value 0"]
impl crate::Resettable for CtiinenSpec {}
