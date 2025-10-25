#[doc = "Register `ITATBCTR0` reader"]
pub type R = crate::R<Itatbctr0Spec>;
#[doc = "Register `ITATBCTR0` writer"]
pub type W = crate::W<Itatbctr0Spec>;
#[doc = "Sets the value of the atvalidm0 output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Atvalidm0 {
    #[doc = "0: Pin is logic 0."]
    Low = 0,
    #[doc = "1: Pin is logic 1."]
    High = 1,
}
impl From<Atvalidm0> for bool {
    #[inline(always)]
    fn from(variant: Atvalidm0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ATVALIDM0` reader - Sets the value of the atvalidm0 output."]
pub type Atvalidm0R = crate::BitReader<Atvalidm0>;
impl Atvalidm0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Atvalidm0 {
        match self.bits {
            false => Atvalidm0::Low,
            true => Atvalidm0::High,
        }
    }
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Atvalidm0::Low
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Atvalidm0::High
    }
}
#[doc = "Field `ATVALIDM0` writer - Sets the value of the atvalidm0 output."]
pub type Atvalidm0W<'a, REG> = crate::BitWriter<'a, REG, Atvalidm0>;
impl<'a, REG> Atvalidm0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Atvalidm0::Low)
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Atvalidm0::High)
    }
}
#[doc = "Sets the value of the atvalidm1 output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Atvalidm1 {
    #[doc = "0: Pin is logic 0."]
    Low = 0,
    #[doc = "1: Pin is logic 1."]
    High = 1,
}
impl From<Atvalidm1> for bool {
    #[inline(always)]
    fn from(variant: Atvalidm1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ATVALIDM1` reader - Sets the value of the atvalidm1 output."]
pub type Atvalidm1R = crate::BitReader<Atvalidm1>;
impl Atvalidm1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Atvalidm1 {
        match self.bits {
            false => Atvalidm1::Low,
            true => Atvalidm1::High,
        }
    }
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Atvalidm1::Low
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Atvalidm1::High
    }
}
#[doc = "Field `ATVALIDM1` writer - Sets the value of the atvalidm1 output."]
pub type Atvalidm1W<'a, REG> = crate::BitWriter<'a, REG, Atvalidm1>;
impl<'a, REG> Atvalidm1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Atvalidm1::Low)
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Atvalidm1::High)
    }
}
#[doc = "Sets the value of the atreadys output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Atreadys {
    #[doc = "0: Pin is logic 0."]
    Low = 0,
    #[doc = "1: Pin is logic 1."]
    High = 1,
}
impl From<Atreadys> for bool {
    #[inline(always)]
    fn from(variant: Atreadys) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ATREADYS` reader - Sets the value of the atreadys output."]
pub type AtreadysR = crate::BitReader<Atreadys>;
impl AtreadysR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Atreadys {
        match self.bits {
            false => Atreadys::Low,
            true => Atreadys::High,
        }
    }
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Atreadys::Low
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Atreadys::High
    }
}
#[doc = "Field `ATREADYS` writer - Sets the value of the atreadys output."]
pub type AtreadysW<'a, REG> = crate::BitWriter<'a, REG, Atreadys>;
impl<'a, REG> AtreadysW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Atreadys::Low)
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Atreadys::High)
    }
}
impl R {
    #[doc = "Bit 0 - Sets the value of the atvalidm0 output."]
    #[inline(always)]
    pub fn atvalidm0(&self) -> Atvalidm0R {
        Atvalidm0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Sets the value of the atvalidm1 output."]
    #[inline(always)]
    pub fn atvalidm1(&self) -> Atvalidm1R {
        Atvalidm1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Sets the value of the atreadys output."]
    #[inline(always)]
    pub fn atreadys(&self) -> AtreadysR {
        AtreadysR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sets the value of the atvalidm0 output."]
    #[inline(always)]
    pub fn atvalidm0(&mut self) -> Atvalidm0W<'_, Itatbctr0Spec> {
        Atvalidm0W::new(self, 0)
    }
    #[doc = "Bit 2 - Sets the value of the atvalidm1 output."]
    #[inline(always)]
    pub fn atvalidm1(&mut self) -> Atvalidm1W<'_, Itatbctr0Spec> {
        Atvalidm1W::new(self, 2)
    }
    #[doc = "Bit 3 - Sets the value of the atreadys output."]
    #[inline(always)]
    pub fn atreadys(&mut self) -> AtreadysW<'_, Itatbctr0Spec> {
        AtreadysW::new(self, 3)
    }
}
#[doc = "The ITATBCTR0 register controls the value of the atvalidm0, atvalidm1, and atreadys outputs in integration mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`itatbctr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itatbctr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Itatbctr0Spec;
impl crate::RegisterSpec for Itatbctr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itatbctr0::R`](R) reader structure"]
impl crate::Readable for Itatbctr0Spec {}
#[doc = "`write(|w| ..)` method takes [`itatbctr0::W`](W) writer structure"]
impl crate::Writable for Itatbctr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ITATBCTR0 to value 0"]
impl crate::Resettable for Itatbctr0Spec {}
