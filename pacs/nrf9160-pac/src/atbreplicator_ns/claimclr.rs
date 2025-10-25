#[doc = "Register `CLAIMCLR` reader"]
pub type R = crate::R<ClaimclrSpec>;
#[doc = "Register `CLAIMCLR` writer"]
pub type W = crate::W<ClaimclrSpec>;
#[doc = "Read or clear claim bit 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bit0 {
    #[doc = "0: Claim bit 0 is not set."]
    Cleared = 0,
    #[doc = "1: Claim bit 0 is set."]
    Set = 1,
}
impl From<Bit0> for bool {
    #[inline(always)]
    fn from(variant: Bit0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BIT_0` reader - Read or clear claim bit 0."]
pub type Bit0R = crate::BitReader<Bit0>;
impl Bit0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bit0 {
        match self.bits {
            false => Bit0::Cleared,
            true => Bit0::Set,
        }
    }
    #[doc = "Claim bit 0 is not set."]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == Bit0::Cleared
    }
    #[doc = "Claim bit 0 is set."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Bit0::Set
    }
}
#[doc = "Read or clear claim bit 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bit0WO {
    #[doc = "1: Clear claim bit 0."]
    Clear = 1,
}
impl From<Bit0WO> for bool {
    #[inline(always)]
    fn from(variant: Bit0WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BIT_0` writer - Read or clear claim bit 0."]
pub type Bit0W<'a, REG> = crate::BitWriter<'a, REG, Bit0WO>;
impl<'a, REG> Bit0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear claim bit 0."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Bit0WO::Clear)
    }
}
#[doc = "Read or clear claim bit 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bit1 {
    #[doc = "0: Claim bit 1 is not set."]
    Cleared = 0,
    #[doc = "1: Claim bit 1 is set."]
    Set = 1,
}
impl From<Bit1> for bool {
    #[inline(always)]
    fn from(variant: Bit1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BIT_1` reader - Read or clear claim bit 1."]
pub type Bit1R = crate::BitReader<Bit1>;
impl Bit1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bit1 {
        match self.bits {
            false => Bit1::Cleared,
            true => Bit1::Set,
        }
    }
    #[doc = "Claim bit 1 is not set."]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == Bit1::Cleared
    }
    #[doc = "Claim bit 1 is set."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Bit1::Set
    }
}
#[doc = "Read or clear claim bit 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bit1WO {
    #[doc = "1: Clear claim bit 1."]
    Clear = 1,
}
impl From<Bit1WO> for bool {
    #[inline(always)]
    fn from(variant: Bit1WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BIT_1` writer - Read or clear claim bit 1."]
pub type Bit1W<'a, REG> = crate::BitWriter<'a, REG, Bit1WO>;
impl<'a, REG> Bit1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear claim bit 1."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Bit1WO::Clear)
    }
}
#[doc = "Read or clear claim bit 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bit2 {
    #[doc = "0: Claim bit 2 is not set."]
    Cleared = 0,
    #[doc = "1: Claim bit 2 is set."]
    Set = 1,
}
impl From<Bit2> for bool {
    #[inline(always)]
    fn from(variant: Bit2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BIT_2` reader - Read or clear claim bit 2."]
pub type Bit2R = crate::BitReader<Bit2>;
impl Bit2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bit2 {
        match self.bits {
            false => Bit2::Cleared,
            true => Bit2::Set,
        }
    }
    #[doc = "Claim bit 2 is not set."]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == Bit2::Cleared
    }
    #[doc = "Claim bit 2 is set."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Bit2::Set
    }
}
#[doc = "Read or clear claim bit 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bit2WO {
    #[doc = "1: Clear claim bit 2."]
    Clear = 1,
}
impl From<Bit2WO> for bool {
    #[inline(always)]
    fn from(variant: Bit2WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BIT_2` writer - Read or clear claim bit 2."]
pub type Bit2W<'a, REG> = crate::BitWriter<'a, REG, Bit2WO>;
impl<'a, REG> Bit2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear claim bit 2."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Bit2WO::Clear)
    }
}
#[doc = "Read or clear claim bit 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bit3 {
    #[doc = "0: Claim bit 3 is not set."]
    Cleared = 0,
    #[doc = "1: Claim bit 3 is set."]
    Set = 1,
}
impl From<Bit3> for bool {
    #[inline(always)]
    fn from(variant: Bit3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BIT_3` reader - Read or clear claim bit 3."]
pub type Bit3R = crate::BitReader<Bit3>;
impl Bit3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bit3 {
        match self.bits {
            false => Bit3::Cleared,
            true => Bit3::Set,
        }
    }
    #[doc = "Claim bit 3 is not set."]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == Bit3::Cleared
    }
    #[doc = "Claim bit 3 is set."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Bit3::Set
    }
}
#[doc = "Read or clear claim bit 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bit3WO {
    #[doc = "1: Clear claim bit 3."]
    Clear = 1,
}
impl From<Bit3WO> for bool {
    #[inline(always)]
    fn from(variant: Bit3WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BIT_3` writer - Read or clear claim bit 3."]
pub type Bit3W<'a, REG> = crate::BitWriter<'a, REG, Bit3WO>;
impl<'a, REG> Bit3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear claim bit 3."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Bit3WO::Clear)
    }
}
impl R {
    #[doc = "Bit 0 - Read or clear claim bit 0."]
    #[inline(always)]
    pub fn bit_0(&self) -> Bit0R {
        Bit0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Read or clear claim bit 1."]
    #[inline(always)]
    pub fn bit_1(&self) -> Bit1R {
        Bit1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Read or clear claim bit 2."]
    #[inline(always)]
    pub fn bit_2(&self) -> Bit2R {
        Bit2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Read or clear claim bit 3."]
    #[inline(always)]
    pub fn bit_3(&self) -> Bit3R {
        Bit3R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Read or clear claim bit 0."]
    #[inline(always)]
    pub fn bit_0(&mut self) -> Bit0W<'_, ClaimclrSpec> {
        Bit0W::new(self, 0)
    }
    #[doc = "Bit 1 - Read or clear claim bit 1."]
    #[inline(always)]
    pub fn bit_1(&mut self) -> Bit1W<'_, ClaimclrSpec> {
        Bit1W::new(self, 1)
    }
    #[doc = "Bit 2 - Read or clear claim bit 2."]
    #[inline(always)]
    pub fn bit_2(&mut self) -> Bit2W<'_, ClaimclrSpec> {
        Bit2W::new(self, 2)
    }
    #[doc = "Bit 3 - Read or clear claim bit 3."]
    #[inline(always)]
    pub fn bit_3(&mut self) -> Bit3W<'_, ClaimclrSpec> {
        Bit3W::new(self, 3)
    }
}
#[doc = "Software can use the claim tag to coordinate application and debugger access to trace unit functionality. The claim tags have no effect on the operation of the component. The CLAIMCLR register sets the bits in the claim tag to 0 and determines the current value of the claim tag.\n\nYou can [`read`](crate::Reg::read) this register and get [`claimclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`claimclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClaimclrSpec;
impl crate::RegisterSpec for ClaimclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`claimclr::R`](R) reader structure"]
impl crate::Readable for ClaimclrSpec {}
#[doc = "`write(|w| ..)` method takes [`claimclr::W`](W) writer structure"]
impl crate::Writable for ClaimclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLAIMCLR to value 0"]
impl crate::Resettable for ClaimclrSpec {}
