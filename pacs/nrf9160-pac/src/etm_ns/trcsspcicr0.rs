#[doc = "Register `TRCSSPCICR0` reader"]
pub type R = crate::R<Trcsspcicr0Spec>;
#[doc = "Register `TRCSSPCICR0` writer"]
pub type W = crate::W<Trcsspcicr0Spec>;
#[doc = "Selects processor comparator 0 inputs for Single-shot control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pc0 {
    #[doc = "0: Processor comparator 0 is not selected for Single-shot control."]
    Disabled = 0,
    #[doc = "1: Processor comparator 0 is selected for Single-shot control."]
    Enabled = 1,
}
impl From<Pc0> for bool {
    #[inline(always)]
    fn from(variant: Pc0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PC_0` reader - Selects processor comparator 0 inputs for Single-shot control"]
pub type Pc0R = crate::BitReader<Pc0>;
impl Pc0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pc0 {
        match self.bits {
            false => Pc0::Disabled,
            true => Pc0::Enabled,
        }
    }
    #[doc = "Processor comparator 0 is not selected for Single-shot control."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pc0::Disabled
    }
    #[doc = "Processor comparator 0 is selected for Single-shot control."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pc0::Enabled
    }
}
#[doc = "Field `PC_0` writer - Selects processor comparator 0 inputs for Single-shot control"]
pub type Pc0W<'a, REG> = crate::BitWriter<'a, REG, Pc0>;
impl<'a, REG> Pc0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Processor comparator 0 is not selected for Single-shot control."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pc0::Disabled)
    }
    #[doc = "Processor comparator 0 is selected for Single-shot control."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pc0::Enabled)
    }
}
#[doc = "Selects processor comparator 1 inputs for Single-shot control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pc1 {
    #[doc = "0: Processor comparator 1 is not selected for Single-shot control."]
    Disabled = 0,
    #[doc = "1: Processor comparator 1 is selected for Single-shot control."]
    Enabled = 1,
}
impl From<Pc1> for bool {
    #[inline(always)]
    fn from(variant: Pc1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PC_1` reader - Selects processor comparator 1 inputs for Single-shot control"]
pub type Pc1R = crate::BitReader<Pc1>;
impl Pc1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pc1 {
        match self.bits {
            false => Pc1::Disabled,
            true => Pc1::Enabled,
        }
    }
    #[doc = "Processor comparator 1 is not selected for Single-shot control."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pc1::Disabled
    }
    #[doc = "Processor comparator 1 is selected for Single-shot control."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pc1::Enabled
    }
}
#[doc = "Field `PC_1` writer - Selects processor comparator 1 inputs for Single-shot control"]
pub type Pc1W<'a, REG> = crate::BitWriter<'a, REG, Pc1>;
impl<'a, REG> Pc1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Processor comparator 1 is not selected for Single-shot control."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pc1::Disabled)
    }
    #[doc = "Processor comparator 1 is selected for Single-shot control."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pc1::Enabled)
    }
}
#[doc = "Selects processor comparator 2 inputs for Single-shot control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pc2 {
    #[doc = "0: Processor comparator 2 is not selected for Single-shot control."]
    Disabled = 0,
    #[doc = "1: Processor comparator 2 is selected for Single-shot control."]
    Enabled = 1,
}
impl From<Pc2> for bool {
    #[inline(always)]
    fn from(variant: Pc2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PC_2` reader - Selects processor comparator 2 inputs for Single-shot control"]
pub type Pc2R = crate::BitReader<Pc2>;
impl Pc2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pc2 {
        match self.bits {
            false => Pc2::Disabled,
            true => Pc2::Enabled,
        }
    }
    #[doc = "Processor comparator 2 is not selected for Single-shot control."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pc2::Disabled
    }
    #[doc = "Processor comparator 2 is selected for Single-shot control."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pc2::Enabled
    }
}
#[doc = "Field `PC_2` writer - Selects processor comparator 2 inputs for Single-shot control"]
pub type Pc2W<'a, REG> = crate::BitWriter<'a, REG, Pc2>;
impl<'a, REG> Pc2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Processor comparator 2 is not selected for Single-shot control."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pc2::Disabled)
    }
    #[doc = "Processor comparator 2 is selected for Single-shot control."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pc2::Enabled)
    }
}
#[doc = "Selects processor comparator 3 inputs for Single-shot control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pc3 {
    #[doc = "0: Processor comparator 3 is not selected for Single-shot control."]
    Disabled = 0,
    #[doc = "1: Processor comparator 3 is selected for Single-shot control."]
    Enabled = 1,
}
impl From<Pc3> for bool {
    #[inline(always)]
    fn from(variant: Pc3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PC_3` reader - Selects processor comparator 3 inputs for Single-shot control"]
pub type Pc3R = crate::BitReader<Pc3>;
impl Pc3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pc3 {
        match self.bits {
            false => Pc3::Disabled,
            true => Pc3::Enabled,
        }
    }
    #[doc = "Processor comparator 3 is not selected for Single-shot control."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pc3::Disabled
    }
    #[doc = "Processor comparator 3 is selected for Single-shot control."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pc3::Enabled
    }
}
#[doc = "Field `PC_3` writer - Selects processor comparator 3 inputs for Single-shot control"]
pub type Pc3W<'a, REG> = crate::BitWriter<'a, REG, Pc3>;
impl<'a, REG> Pc3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Processor comparator 3 is not selected for Single-shot control."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pc3::Disabled)
    }
    #[doc = "Processor comparator 3 is selected for Single-shot control."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pc3::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Selects processor comparator 0 inputs for Single-shot control"]
    #[inline(always)]
    pub fn pc_0(&self) -> Pc0R {
        Pc0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Selects processor comparator 1 inputs for Single-shot control"]
    #[inline(always)]
    pub fn pc_1(&self) -> Pc1R {
        Pc1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Selects processor comparator 2 inputs for Single-shot control"]
    #[inline(always)]
    pub fn pc_2(&self) -> Pc2R {
        Pc2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Selects processor comparator 3 inputs for Single-shot control"]
    #[inline(always)]
    pub fn pc_3(&self) -> Pc3R {
        Pc3R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selects processor comparator 0 inputs for Single-shot control"]
    #[inline(always)]
    pub fn pc_0(&mut self) -> Pc0W<'_, Trcsspcicr0Spec> {
        Pc0W::new(self, 0)
    }
    #[doc = "Bit 1 - Selects processor comparator 1 inputs for Single-shot control"]
    #[inline(always)]
    pub fn pc_1(&mut self) -> Pc1W<'_, Trcsspcicr0Spec> {
        Pc1W::new(self, 1)
    }
    #[doc = "Bit 2 - Selects processor comparator 2 inputs for Single-shot control"]
    #[inline(always)]
    pub fn pc_2(&mut self) -> Pc2W<'_, Trcsspcicr0Spec> {
        Pc2W::new(self, 2)
    }
    #[doc = "Bit 3 - Selects processor comparator 3 inputs for Single-shot control"]
    #[inline(always)]
    pub fn pc_3(&mut self) -> Pc3W<'_, Trcsspcicr0Spec> {
        Pc3W::new(self, 3)
    }
}
#[doc = "Selects the processor comparator inputs for Single-shot control.\n\nYou can [`read`](crate::Reg::read) this register and get [`trcsspcicr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcsspcicr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Trcsspcicr0Spec;
impl crate::RegisterSpec for Trcsspcicr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcsspcicr0::R`](R) reader structure"]
impl crate::Readable for Trcsspcicr0Spec {}
#[doc = "`write(|w| ..)` method takes [`trcsspcicr0::W`](W) writer structure"]
impl crate::Writable for Trcsspcicr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRCSSPCICR0 to value 0"]
impl crate::Resettable for Trcsspcicr0Spec {}
