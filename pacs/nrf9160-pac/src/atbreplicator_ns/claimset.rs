#[doc = "Register `CLAIMSET` reader"]
pub type R = crate::R<ClaimsetSpec>;
#[doc = "Register `CLAIMSET` writer"]
pub type W = crate::W<ClaimsetSpec>;
#[doc = "Set claim bit 0 and check if bit is implemented or not.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bit0 {
    #[doc = "0: Claim bit 0 is not implemented."]
    NotImplemented = 0,
    #[doc = "1: Claim bit 0 is implemented."]
    Implemented = 1,
}
impl From<Bit0> for bool {
    #[inline(always)]
    fn from(variant: Bit0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BIT_0` reader - Set claim bit 0 and check if bit is implemented or not."]
pub type Bit0R = crate::BitReader<Bit0>;
impl Bit0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bit0 {
        match self.bits {
            false => Bit0::NotImplemented,
            true => Bit0::Implemented,
        }
    }
    #[doc = "Claim bit 0 is not implemented."]
    #[inline(always)]
    pub fn is_not_implemented(&self) -> bool {
        *self == Bit0::NotImplemented
    }
    #[doc = "Claim bit 0 is implemented."]
    #[inline(always)]
    pub fn is_implemented(&self) -> bool {
        *self == Bit0::Implemented
    }
}
#[doc = "Set claim bit 0 and check if bit is implemented or not.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bit0WO {
    #[doc = "1: Set claim bit 0."]
    Set = 1,
}
impl From<Bit0WO> for bool {
    #[inline(always)]
    fn from(variant: Bit0WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BIT_0` writer - Set claim bit 0 and check if bit is implemented or not."]
pub type Bit0W<'a, REG> = crate::BitWriter<'a, REG, Bit0WO>;
impl<'a, REG> Bit0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set claim bit 0."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Bit0WO::Set)
    }
}
#[doc = "Set claim bit 1 and check if bit is implemented or not.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bit1 {
    #[doc = "0: Claim bit 1 is not implemented."]
    NotImplemented = 0,
    #[doc = "1: Claim bit 1 is implemented."]
    Implemented = 1,
}
impl From<Bit1> for bool {
    #[inline(always)]
    fn from(variant: Bit1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BIT_1` reader - Set claim bit 1 and check if bit is implemented or not."]
pub type Bit1R = crate::BitReader<Bit1>;
impl Bit1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bit1 {
        match self.bits {
            false => Bit1::NotImplemented,
            true => Bit1::Implemented,
        }
    }
    #[doc = "Claim bit 1 is not implemented."]
    #[inline(always)]
    pub fn is_not_implemented(&self) -> bool {
        *self == Bit1::NotImplemented
    }
    #[doc = "Claim bit 1 is implemented."]
    #[inline(always)]
    pub fn is_implemented(&self) -> bool {
        *self == Bit1::Implemented
    }
}
#[doc = "Set claim bit 1 and check if bit is implemented or not.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bit1WO {
    #[doc = "1: Set claim bit 1."]
    Set = 1,
}
impl From<Bit1WO> for bool {
    #[inline(always)]
    fn from(variant: Bit1WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BIT_1` writer - Set claim bit 1 and check if bit is implemented or not."]
pub type Bit1W<'a, REG> = crate::BitWriter<'a, REG, Bit1WO>;
impl<'a, REG> Bit1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set claim bit 1."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Bit1WO::Set)
    }
}
#[doc = "Set claim bit 2 and check if bit is implemented or not.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bit2 {
    #[doc = "0: Claim bit 2 is not implemented."]
    NotImplemented = 0,
    #[doc = "1: Claim bit 2 is implemented."]
    Implemented = 1,
}
impl From<Bit2> for bool {
    #[inline(always)]
    fn from(variant: Bit2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BIT_2` reader - Set claim bit 2 and check if bit is implemented or not."]
pub type Bit2R = crate::BitReader<Bit2>;
impl Bit2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bit2 {
        match self.bits {
            false => Bit2::NotImplemented,
            true => Bit2::Implemented,
        }
    }
    #[doc = "Claim bit 2 is not implemented."]
    #[inline(always)]
    pub fn is_not_implemented(&self) -> bool {
        *self == Bit2::NotImplemented
    }
    #[doc = "Claim bit 2 is implemented."]
    #[inline(always)]
    pub fn is_implemented(&self) -> bool {
        *self == Bit2::Implemented
    }
}
#[doc = "Set claim bit 2 and check if bit is implemented or not.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bit2WO {
    #[doc = "1: Set claim bit 2."]
    Set = 1,
}
impl From<Bit2WO> for bool {
    #[inline(always)]
    fn from(variant: Bit2WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BIT_2` writer - Set claim bit 2 and check if bit is implemented or not."]
pub type Bit2W<'a, REG> = crate::BitWriter<'a, REG, Bit2WO>;
impl<'a, REG> Bit2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set claim bit 2."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Bit2WO::Set)
    }
}
#[doc = "Set claim bit 3 and check if bit is implemented or not.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bit3 {
    #[doc = "0: Claim bit 3 is not implemented."]
    NotImplemented = 0,
    #[doc = "1: Claim bit 3 is implemented."]
    Implemented = 1,
}
impl From<Bit3> for bool {
    #[inline(always)]
    fn from(variant: Bit3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BIT_3` reader - Set claim bit 3 and check if bit is implemented or not."]
pub type Bit3R = crate::BitReader<Bit3>;
impl Bit3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bit3 {
        match self.bits {
            false => Bit3::NotImplemented,
            true => Bit3::Implemented,
        }
    }
    #[doc = "Claim bit 3 is not implemented."]
    #[inline(always)]
    pub fn is_not_implemented(&self) -> bool {
        *self == Bit3::NotImplemented
    }
    #[doc = "Claim bit 3 is implemented."]
    #[inline(always)]
    pub fn is_implemented(&self) -> bool {
        *self == Bit3::Implemented
    }
}
#[doc = "Set claim bit 3 and check if bit is implemented or not.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bit3WO {
    #[doc = "1: Set claim bit 3."]
    Set = 1,
}
impl From<Bit3WO> for bool {
    #[inline(always)]
    fn from(variant: Bit3WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BIT_3` writer - Set claim bit 3 and check if bit is implemented or not."]
pub type Bit3W<'a, REG> = crate::BitWriter<'a, REG, Bit3WO>;
impl<'a, REG> Bit3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set claim bit 3."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Bit3WO::Set)
    }
}
impl R {
    #[doc = "Bit 0 - Set claim bit 0 and check if bit is implemented or not."]
    #[inline(always)]
    pub fn bit_0(&self) -> Bit0R {
        Bit0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set claim bit 1 and check if bit is implemented or not."]
    #[inline(always)]
    pub fn bit_1(&self) -> Bit1R {
        Bit1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set claim bit 2 and check if bit is implemented or not."]
    #[inline(always)]
    pub fn bit_2(&self) -> Bit2R {
        Bit2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set claim bit 3 and check if bit is implemented or not."]
    #[inline(always)]
    pub fn bit_3(&self) -> Bit3R {
        Bit3R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set claim bit 0 and check if bit is implemented or not."]
    #[inline(always)]
    pub fn bit_0(&mut self) -> Bit0W<'_, ClaimsetSpec> {
        Bit0W::new(self, 0)
    }
    #[doc = "Bit 1 - Set claim bit 1 and check if bit is implemented or not."]
    #[inline(always)]
    pub fn bit_1(&mut self) -> Bit1W<'_, ClaimsetSpec> {
        Bit1W::new(self, 1)
    }
    #[doc = "Bit 2 - Set claim bit 2 and check if bit is implemented or not."]
    #[inline(always)]
    pub fn bit_2(&mut self) -> Bit2W<'_, ClaimsetSpec> {
        Bit2W::new(self, 2)
    }
    #[doc = "Bit 3 - Set claim bit 3 and check if bit is implemented or not."]
    #[inline(always)]
    pub fn bit_3(&mut self) -> Bit3W<'_, ClaimsetSpec> {
        Bit3W::new(self, 3)
    }
}
#[doc = "Software can use the claim tag to coordinate application and debugger access to trace unit functionality. The claim tags have no effect on the operation of the component. The CLAIMSET register sets bits in the claim tag, and determines the number of claim bits implemented.\n\nYou can [`read`](crate::Reg::read) this register and get [`claimset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`claimset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClaimsetSpec;
impl crate::RegisterSpec for ClaimsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`claimset::R`](R) reader structure"]
impl crate::Readable for ClaimsetSpec {}
#[doc = "`write(|w| ..)` method takes [`claimset::W`](W) writer structure"]
impl crate::Writable for ClaimsetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLAIMSET to value 0"]
impl crate::Resettable for ClaimsetSpec {}
