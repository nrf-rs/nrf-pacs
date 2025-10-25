#[doc = "Register `TRCCLAIMSET` reader"]
pub type R = crate::R<TrcclaimsetSpec>;
#[doc = "Register `TRCCLAIMSET` writer"]
pub type W = crate::W<TrcclaimsetSpec>;
#[doc = "Claim tag set register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Set0 {
    #[doc = "0: Claim tag 0 is not set."]
    NotSet = 0,
    #[doc = "1: Claim tag 0 is set."]
    Set = 1,
}
impl From<Set0> for bool {
    #[inline(always)]
    fn from(variant: Set0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SET_0` reader - Claim tag set register"]
pub type Set0R = crate::BitReader<Set0>;
impl Set0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Set0 {
        match self.bits {
            false => Set0::NotSet,
            true => Set0::Set,
        }
    }
    #[doc = "Claim tag 0 is not set."]
    #[inline(always)]
    pub fn is_not_set(&self) -> bool {
        *self == Set0::NotSet
    }
    #[doc = "Claim tag 0 is set."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Set0::Set
    }
}
#[doc = "Claim tag set register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Set0WO {
    #[doc = "1: Set claim tag 0."]
    Claim = 1,
}
impl From<Set0WO> for bool {
    #[inline(always)]
    fn from(variant: Set0WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SET_0` writer - Claim tag set register"]
pub type Set0W<'a, REG> = crate::BitWriter<'a, REG, Set0WO>;
impl<'a, REG> Set0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set claim tag 0."]
    #[inline(always)]
    pub fn claim(self) -> &'a mut crate::W<REG> {
        self.variant(Set0WO::Claim)
    }
}
#[doc = "Claim tag set register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Set1 {
    #[doc = "0: Claim tag 1 is not set."]
    NotSet = 0,
    #[doc = "1: Claim tag 1 is set."]
    Set = 1,
}
impl From<Set1> for bool {
    #[inline(always)]
    fn from(variant: Set1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SET_1` reader - Claim tag set register"]
pub type Set1R = crate::BitReader<Set1>;
impl Set1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Set1 {
        match self.bits {
            false => Set1::NotSet,
            true => Set1::Set,
        }
    }
    #[doc = "Claim tag 1 is not set."]
    #[inline(always)]
    pub fn is_not_set(&self) -> bool {
        *self == Set1::NotSet
    }
    #[doc = "Claim tag 1 is set."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Set1::Set
    }
}
#[doc = "Claim tag set register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Set1WO {
    #[doc = "1: Set claim tag 1."]
    Claim = 1,
}
impl From<Set1WO> for bool {
    #[inline(always)]
    fn from(variant: Set1WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SET_1` writer - Claim tag set register"]
pub type Set1W<'a, REG> = crate::BitWriter<'a, REG, Set1WO>;
impl<'a, REG> Set1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set claim tag 1."]
    #[inline(always)]
    pub fn claim(self) -> &'a mut crate::W<REG> {
        self.variant(Set1WO::Claim)
    }
}
#[doc = "Claim tag set register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Set2 {
    #[doc = "0: Claim tag 2 is not set."]
    NotSet = 0,
    #[doc = "1: Claim tag 2 is set."]
    Set = 1,
}
impl From<Set2> for bool {
    #[inline(always)]
    fn from(variant: Set2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SET_2` reader - Claim tag set register"]
pub type Set2R = crate::BitReader<Set2>;
impl Set2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Set2 {
        match self.bits {
            false => Set2::NotSet,
            true => Set2::Set,
        }
    }
    #[doc = "Claim tag 2 is not set."]
    #[inline(always)]
    pub fn is_not_set(&self) -> bool {
        *self == Set2::NotSet
    }
    #[doc = "Claim tag 2 is set."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Set2::Set
    }
}
#[doc = "Claim tag set register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Set2WO {
    #[doc = "1: Set claim tag 2."]
    Claim = 1,
}
impl From<Set2WO> for bool {
    #[inline(always)]
    fn from(variant: Set2WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SET_2` writer - Claim tag set register"]
pub type Set2W<'a, REG> = crate::BitWriter<'a, REG, Set2WO>;
impl<'a, REG> Set2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set claim tag 2."]
    #[inline(always)]
    pub fn claim(self) -> &'a mut crate::W<REG> {
        self.variant(Set2WO::Claim)
    }
}
#[doc = "Claim tag set register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Set3 {
    #[doc = "0: Claim tag 3 is not set."]
    NotSet = 0,
    #[doc = "1: Claim tag 3 is set."]
    Set = 1,
}
impl From<Set3> for bool {
    #[inline(always)]
    fn from(variant: Set3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SET_3` reader - Claim tag set register"]
pub type Set3R = crate::BitReader<Set3>;
impl Set3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Set3 {
        match self.bits {
            false => Set3::NotSet,
            true => Set3::Set,
        }
    }
    #[doc = "Claim tag 3 is not set."]
    #[inline(always)]
    pub fn is_not_set(&self) -> bool {
        *self == Set3::NotSet
    }
    #[doc = "Claim tag 3 is set."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Set3::Set
    }
}
#[doc = "Claim tag set register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Set3WO {
    #[doc = "1: Set claim tag 3."]
    Claim = 1,
}
impl From<Set3WO> for bool {
    #[inline(always)]
    fn from(variant: Set3WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SET_3` writer - Claim tag set register"]
pub type Set3W<'a, REG> = crate::BitWriter<'a, REG, Set3WO>;
impl<'a, REG> Set3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set claim tag 3."]
    #[inline(always)]
    pub fn claim(self) -> &'a mut crate::W<REG> {
        self.variant(Set3WO::Claim)
    }
}
impl R {
    #[doc = "Bit 0 - Claim tag set register"]
    #[inline(always)]
    pub fn set_0(&self) -> Set0R {
        Set0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Claim tag set register"]
    #[inline(always)]
    pub fn set_1(&self) -> Set1R {
        Set1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Claim tag set register"]
    #[inline(always)]
    pub fn set_2(&self) -> Set2R {
        Set2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Claim tag set register"]
    #[inline(always)]
    pub fn set_3(&self) -> Set3R {
        Set3R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Claim tag set register"]
    #[inline(always)]
    pub fn set_0(&mut self) -> Set0W<'_, TrcclaimsetSpec> {
        Set0W::new(self, 0)
    }
    #[doc = "Bit 1 - Claim tag set register"]
    #[inline(always)]
    pub fn set_1(&mut self) -> Set1W<'_, TrcclaimsetSpec> {
        Set1W::new(self, 1)
    }
    #[doc = "Bit 2 - Claim tag set register"]
    #[inline(always)]
    pub fn set_2(&mut self) -> Set2W<'_, TrcclaimsetSpec> {
        Set2W::new(self, 2)
    }
    #[doc = "Bit 3 - Claim tag set register"]
    #[inline(always)]
    pub fn set_3(&mut self) -> Set3W<'_, TrcclaimsetSpec> {
        Set3W::new(self, 3)
    }
}
#[doc = "Sets bits in the claim tag and determines the number of claim tag bits implemented.\n\nYou can [`read`](crate::Reg::read) this register and get [`trcclaimset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcclaimset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrcclaimsetSpec;
impl crate::RegisterSpec for TrcclaimsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcclaimset::R`](R) reader structure"]
impl crate::Readable for TrcclaimsetSpec {}
#[doc = "`write(|w| ..)` method takes [`trcclaimset::W`](W) writer structure"]
impl crate::Writable for TrcclaimsetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRCCLAIMSET to value 0"]
impl crate::Resettable for TrcclaimsetSpec {}
