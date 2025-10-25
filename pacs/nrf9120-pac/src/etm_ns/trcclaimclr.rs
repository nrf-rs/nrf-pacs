#[doc = "Register `TRCCLAIMCLR` reader"]
pub type R = crate::R<TrcclaimclrSpec>;
#[doc = "Register `TRCCLAIMCLR` writer"]
pub type W = crate::W<TrcclaimclrSpec>;
#[doc = "Claim tag clear register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clr0 {
    #[doc = "0: Claim tag 0 is not set."]
    NotSet = 0,
    #[doc = "1: Claim tag 0 is set."]
    Set = 1,
}
impl From<Clr0> for bool {
    #[inline(always)]
    fn from(variant: Clr0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_0` reader - Claim tag clear register"]
pub type Clr0R = crate::BitReader<Clr0>;
impl Clr0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clr0 {
        match self.bits {
            false => Clr0::NotSet,
            true => Clr0::Set,
        }
    }
    #[doc = "Claim tag 0 is not set."]
    #[inline(always)]
    pub fn is_not_set(&self) -> bool {
        *self == Clr0::NotSet
    }
    #[doc = "Claim tag 0 is set."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Clr0::Set
    }
}
#[doc = "Claim tag clear register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clr0WO {
    #[doc = "1: Clear claim tag 0."]
    Clear = 1,
}
impl From<Clr0WO> for bool {
    #[inline(always)]
    fn from(variant: Clr0WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_0` writer - Claim tag clear register"]
pub type Clr0W<'a, REG> = crate::BitWriter<'a, REG, Clr0WO>;
impl<'a, REG> Clr0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear claim tag 0."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Clr0WO::Clear)
    }
}
#[doc = "Claim tag clear register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clr1 {
    #[doc = "0: Claim tag 1 is not set."]
    NotSet = 0,
    #[doc = "1: Claim tag 1 is set."]
    Set = 1,
}
impl From<Clr1> for bool {
    #[inline(always)]
    fn from(variant: Clr1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_1` reader - Claim tag clear register"]
pub type Clr1R = crate::BitReader<Clr1>;
impl Clr1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clr1 {
        match self.bits {
            false => Clr1::NotSet,
            true => Clr1::Set,
        }
    }
    #[doc = "Claim tag 1 is not set."]
    #[inline(always)]
    pub fn is_not_set(&self) -> bool {
        *self == Clr1::NotSet
    }
    #[doc = "Claim tag 1 is set."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Clr1::Set
    }
}
#[doc = "Claim tag clear register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clr1WO {
    #[doc = "1: Clear claim tag 1."]
    Clear = 1,
}
impl From<Clr1WO> for bool {
    #[inline(always)]
    fn from(variant: Clr1WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_1` writer - Claim tag clear register"]
pub type Clr1W<'a, REG> = crate::BitWriter<'a, REG, Clr1WO>;
impl<'a, REG> Clr1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear claim tag 1."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Clr1WO::Clear)
    }
}
#[doc = "Claim tag clear register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clr2 {
    #[doc = "0: Claim tag 2 is not set."]
    NotSet = 0,
    #[doc = "1: Claim tag 2 is set."]
    Set = 1,
}
impl From<Clr2> for bool {
    #[inline(always)]
    fn from(variant: Clr2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_2` reader - Claim tag clear register"]
pub type Clr2R = crate::BitReader<Clr2>;
impl Clr2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clr2 {
        match self.bits {
            false => Clr2::NotSet,
            true => Clr2::Set,
        }
    }
    #[doc = "Claim tag 2 is not set."]
    #[inline(always)]
    pub fn is_not_set(&self) -> bool {
        *self == Clr2::NotSet
    }
    #[doc = "Claim tag 2 is set."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Clr2::Set
    }
}
#[doc = "Claim tag clear register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clr2WO {
    #[doc = "1: Clear claim tag 2."]
    Clear = 1,
}
impl From<Clr2WO> for bool {
    #[inline(always)]
    fn from(variant: Clr2WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_2` writer - Claim tag clear register"]
pub type Clr2W<'a, REG> = crate::BitWriter<'a, REG, Clr2WO>;
impl<'a, REG> Clr2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear claim tag 2."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Clr2WO::Clear)
    }
}
#[doc = "Claim tag clear register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clr3 {
    #[doc = "0: Claim tag 3 is not set."]
    NotSet = 0,
    #[doc = "1: Claim tag 3 is set."]
    Set = 1,
}
impl From<Clr3> for bool {
    #[inline(always)]
    fn from(variant: Clr3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_3` reader - Claim tag clear register"]
pub type Clr3R = crate::BitReader<Clr3>;
impl Clr3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clr3 {
        match self.bits {
            false => Clr3::NotSet,
            true => Clr3::Set,
        }
    }
    #[doc = "Claim tag 3 is not set."]
    #[inline(always)]
    pub fn is_not_set(&self) -> bool {
        *self == Clr3::NotSet
    }
    #[doc = "Claim tag 3 is set."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Clr3::Set
    }
}
#[doc = "Claim tag clear register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clr3WO {
    #[doc = "1: Clear claim tag 3."]
    Clear = 1,
}
impl From<Clr3WO> for bool {
    #[inline(always)]
    fn from(variant: Clr3WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_3` writer - Claim tag clear register"]
pub type Clr3W<'a, REG> = crate::BitWriter<'a, REG, Clr3WO>;
impl<'a, REG> Clr3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear claim tag 3."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Clr3WO::Clear)
    }
}
impl R {
    #[doc = "Bit 0 - Claim tag clear register"]
    #[inline(always)]
    pub fn clr_0(&self) -> Clr0R {
        Clr0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Claim tag clear register"]
    #[inline(always)]
    pub fn clr_1(&self) -> Clr1R {
        Clr1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Claim tag clear register"]
    #[inline(always)]
    pub fn clr_2(&self) -> Clr2R {
        Clr2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Claim tag clear register"]
    #[inline(always)]
    pub fn clr_3(&self) -> Clr3R {
        Clr3R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Claim tag clear register"]
    #[inline(always)]
    pub fn clr_0(&mut self) -> Clr0W<'_, TrcclaimclrSpec> {
        Clr0W::new(self, 0)
    }
    #[doc = "Bit 1 - Claim tag clear register"]
    #[inline(always)]
    pub fn clr_1(&mut self) -> Clr1W<'_, TrcclaimclrSpec> {
        Clr1W::new(self, 1)
    }
    #[doc = "Bit 2 - Claim tag clear register"]
    #[inline(always)]
    pub fn clr_2(&mut self) -> Clr2W<'_, TrcclaimclrSpec> {
        Clr2W::new(self, 2)
    }
    #[doc = "Bit 3 - Claim tag clear register"]
    #[inline(always)]
    pub fn clr_3(&mut self) -> Clr3W<'_, TrcclaimclrSpec> {
        Clr3W::new(self, 3)
    }
}
#[doc = "Clears bits in the claim tag and determines the current value of the claim tag.\n\nYou can [`read`](crate::Reg::read) this register and get [`trcclaimclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcclaimclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrcclaimclrSpec;
impl crate::RegisterSpec for TrcclaimclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcclaimclr::R`](R) reader structure"]
impl crate::Readable for TrcclaimclrSpec {}
#[doc = "`write(|w| ..)` method takes [`trcclaimclr::W`](W) writer structure"]
impl crate::Writable for TrcclaimclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRCCLAIMCLR to value 0"]
impl crate::Resettable for TrcclaimclrSpec {}
