#[doc = "Register `TRIGGERMULTIPLIER` reader"]
pub type R = crate::R<TriggermultiplierSpec>;
#[doc = "Register `TRIGGERMULTIPLIER` writer"]
pub type W = crate::W<TriggermultiplierSpec>;
#[doc = "Multiply the Trigger Counter by 2^n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mult0 {
    #[doc = "0: Multiplier disabled."]
    Disabled = 0,
    #[doc = "1: Multiplier enabled."]
    Enabled = 1,
}
impl From<Mult0> for bool {
    #[inline(always)]
    fn from(variant: Mult0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MULT_0` reader - Multiply the Trigger Counter by 2^n."]
pub type Mult0R = crate::BitReader<Mult0>;
impl Mult0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mult0 {
        match self.bits {
            false => Mult0::Disabled,
            true => Mult0::Enabled,
        }
    }
    #[doc = "Multiplier disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Mult0::Disabled
    }
    #[doc = "Multiplier enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Mult0::Enabled
    }
}
#[doc = "Field `MULT_0` writer - Multiply the Trigger Counter by 2^n."]
pub type Mult0W<'a, REG> = crate::BitWriter<'a, REG, Mult0>;
impl<'a, REG> Mult0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplier disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mult0::Disabled)
    }
    #[doc = "Multiplier enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mult0::Enabled)
    }
}
#[doc = "Multiply the Trigger Counter by 2^n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mult1 {
    #[doc = "0: Multiplier disabled."]
    Disabled = 0,
    #[doc = "1: Multiplier enabled."]
    Enabled = 1,
}
impl From<Mult1> for bool {
    #[inline(always)]
    fn from(variant: Mult1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MULT_1` reader - Multiply the Trigger Counter by 2^n."]
pub type Mult1R = crate::BitReader<Mult1>;
impl Mult1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mult1 {
        match self.bits {
            false => Mult1::Disabled,
            true => Mult1::Enabled,
        }
    }
    #[doc = "Multiplier disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Mult1::Disabled
    }
    #[doc = "Multiplier enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Mult1::Enabled
    }
}
#[doc = "Field `MULT_1` writer - Multiply the Trigger Counter by 2^n."]
pub type Mult1W<'a, REG> = crate::BitWriter<'a, REG, Mult1>;
impl<'a, REG> Mult1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplier disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mult1::Disabled)
    }
    #[doc = "Multiplier enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mult1::Enabled)
    }
}
#[doc = "Multiply the Trigger Counter by 2^n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mult2 {
    #[doc = "0: Multiplier disabled."]
    Disabled = 0,
    #[doc = "1: Multiplier enabled."]
    Enabled = 1,
}
impl From<Mult2> for bool {
    #[inline(always)]
    fn from(variant: Mult2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MULT_2` reader - Multiply the Trigger Counter by 2^n."]
pub type Mult2R = crate::BitReader<Mult2>;
impl Mult2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mult2 {
        match self.bits {
            false => Mult2::Disabled,
            true => Mult2::Enabled,
        }
    }
    #[doc = "Multiplier disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Mult2::Disabled
    }
    #[doc = "Multiplier enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Mult2::Enabled
    }
}
#[doc = "Field `MULT_2` writer - Multiply the Trigger Counter by 2^n."]
pub type Mult2W<'a, REG> = crate::BitWriter<'a, REG, Mult2>;
impl<'a, REG> Mult2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplier disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mult2::Disabled)
    }
    #[doc = "Multiplier enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mult2::Enabled)
    }
}
#[doc = "Multiply the Trigger Counter by 2^n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mult3 {
    #[doc = "0: Multiplier disabled."]
    Disabled = 0,
    #[doc = "1: Multiplier enabled."]
    Enabled = 1,
}
impl From<Mult3> for bool {
    #[inline(always)]
    fn from(variant: Mult3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MULT_3` reader - Multiply the Trigger Counter by 2^n."]
pub type Mult3R = crate::BitReader<Mult3>;
impl Mult3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mult3 {
        match self.bits {
            false => Mult3::Disabled,
            true => Mult3::Enabled,
        }
    }
    #[doc = "Multiplier disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Mult3::Disabled
    }
    #[doc = "Multiplier enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Mult3::Enabled
    }
}
#[doc = "Field `MULT_3` writer - Multiply the Trigger Counter by 2^n."]
pub type Mult3W<'a, REG> = crate::BitWriter<'a, REG, Mult3>;
impl<'a, REG> Mult3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplier disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mult3::Disabled)
    }
    #[doc = "Multiplier enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mult3::Enabled)
    }
}
#[doc = "Multiply the Trigger Counter by 2^n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mult4 {
    #[doc = "0: Multiplier disabled."]
    Disabled = 0,
    #[doc = "1: Multiplier enabled."]
    Enabled = 1,
}
impl From<Mult4> for bool {
    #[inline(always)]
    fn from(variant: Mult4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MULT_4` reader - Multiply the Trigger Counter by 2^n."]
pub type Mult4R = crate::BitReader<Mult4>;
impl Mult4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mult4 {
        match self.bits {
            false => Mult4::Disabled,
            true => Mult4::Enabled,
        }
    }
    #[doc = "Multiplier disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Mult4::Disabled
    }
    #[doc = "Multiplier enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Mult4::Enabled
    }
}
#[doc = "Field `MULT_4` writer - Multiply the Trigger Counter by 2^n."]
pub type Mult4W<'a, REG> = crate::BitWriter<'a, REG, Mult4>;
impl<'a, REG> Mult4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplier disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mult4::Disabled)
    }
    #[doc = "Multiplier enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mult4::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Multiply the Trigger Counter by 2^n."]
    #[inline(always)]
    pub fn mult_0(&self) -> Mult0R {
        Mult0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Multiply the Trigger Counter by 2^n."]
    #[inline(always)]
    pub fn mult_1(&self) -> Mult1R {
        Mult1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Multiply the Trigger Counter by 2^n."]
    #[inline(always)]
    pub fn mult_2(&self) -> Mult2R {
        Mult2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Multiply the Trigger Counter by 2^n."]
    #[inline(always)]
    pub fn mult_3(&self) -> Mult3R {
        Mult3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Multiply the Trigger Counter by 2^n."]
    #[inline(always)]
    pub fn mult_4(&self) -> Mult4R {
        Mult4R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Multiply the Trigger Counter by 2^n."]
    #[inline(always)]
    pub fn mult_0(&mut self) -> Mult0W<'_, TriggermultiplierSpec> {
        Mult0W::new(self, 0)
    }
    #[doc = "Bit 1 - Multiply the Trigger Counter by 2^n."]
    #[inline(always)]
    pub fn mult_1(&mut self) -> Mult1W<'_, TriggermultiplierSpec> {
        Mult1W::new(self, 1)
    }
    #[doc = "Bit 2 - Multiply the Trigger Counter by 2^n."]
    #[inline(always)]
    pub fn mult_2(&mut self) -> Mult2W<'_, TriggermultiplierSpec> {
        Mult2W::new(self, 2)
    }
    #[doc = "Bit 3 - Multiply the Trigger Counter by 2^n."]
    #[inline(always)]
    pub fn mult_3(&mut self) -> Mult3W<'_, TriggermultiplierSpec> {
        Mult3W::new(self, 3)
    }
    #[doc = "Bit 4 - Multiply the Trigger Counter by 2^n."]
    #[inline(always)]
    pub fn mult_4(&mut self) -> Mult4W<'_, TriggermultiplierSpec> {
        Mult4W::new(self, 4)
    }
}
#[doc = "The Trigger_multiplier register contains the selectors for the trigger counter multiplier.\n\nYou can [`read`](crate::Reg::read) this register and get [`triggermultiplier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`triggermultiplier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TriggermultiplierSpec;
impl crate::RegisterSpec for TriggermultiplierSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`triggermultiplier::R`](R) reader structure"]
impl crate::Readable for TriggermultiplierSpec {}
#[doc = "`write(|w| ..)` method takes [`triggermultiplier::W`](W) writer structure"]
impl crate::Writable for TriggermultiplierSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRIGGERMULTIPLIER to value 0"]
impl crate::Resettable for TriggermultiplierSpec {}
