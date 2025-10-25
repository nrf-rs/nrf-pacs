#[doc = "Register `ITATBCTR1` reader"]
pub type R = crate::R<Itatbctr1Spec>;
#[doc = "Register `ITATBCTR1` writer"]
pub type W = crate::W<Itatbctr1Spec>;
#[doc = "Reads the value of the atreadym0 input.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Atreadym0 {
    #[doc = "0: Pin is logic 0."]
    Low = 0,
    #[doc = "1: Pin is logic 1."]
    High = 1,
}
impl From<Atreadym0> for bool {
    #[inline(always)]
    fn from(variant: Atreadym0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ATREADYM0` reader - Reads the value of the atreadym0 input."]
pub type Atreadym0R = crate::BitReader<Atreadym0>;
impl Atreadym0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Atreadym0 {
        match self.bits {
            false => Atreadym0::Low,
            true => Atreadym0::High,
        }
    }
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Atreadym0::Low
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Atreadym0::High
    }
}
#[doc = "Field `ATREADYM0` writer - Reads the value of the atreadym0 input."]
pub type Atreadym0W<'a, REG> = crate::BitWriter<'a, REG, Atreadym0>;
impl<'a, REG> Atreadym0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Atreadym0::Low)
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Atreadym0::High)
    }
}
#[doc = "Reads the value of the atreadym1 input.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Atreadym1 {
    #[doc = "0: Pin is logic 0."]
    Low = 0,
    #[doc = "1: Pin is logic 1."]
    High = 1,
}
impl From<Atreadym1> for bool {
    #[inline(always)]
    fn from(variant: Atreadym1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ATREADYM1` reader - Reads the value of the atreadym1 input."]
pub type Atreadym1R = crate::BitReader<Atreadym1>;
impl Atreadym1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Atreadym1 {
        match self.bits {
            false => Atreadym1::Low,
            true => Atreadym1::High,
        }
    }
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Atreadym1::Low
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Atreadym1::High
    }
}
#[doc = "Field `ATREADYM1` writer - Reads the value of the atreadym1 input."]
pub type Atreadym1W<'a, REG> = crate::BitWriter<'a, REG, Atreadym1>;
impl<'a, REG> Atreadym1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Atreadym1::Low)
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Atreadym1::High)
    }
}
#[doc = "Reads the value of the atvalids input.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Atvalids {
    #[doc = "0: Pin is logic 0."]
    Low = 0,
    #[doc = "1: Pin is logic 1."]
    High = 1,
}
impl From<Atvalids> for bool {
    #[inline(always)]
    fn from(variant: Atvalids) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ATVALIDS` reader - Reads the value of the atvalids input."]
pub type AtvalidsR = crate::BitReader<Atvalids>;
impl AtvalidsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Atvalids {
        match self.bits {
            false => Atvalids::Low,
            true => Atvalids::High,
        }
    }
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Atvalids::Low
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Atvalids::High
    }
}
#[doc = "Field `ATVALIDS` writer - Reads the value of the atvalids input."]
pub type AtvalidsW<'a, REG> = crate::BitWriter<'a, REG, Atvalids>;
impl<'a, REG> AtvalidsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Atvalids::Low)
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Atvalids::High)
    }
}
impl R {
    #[doc = "Bit 0 - Reads the value of the atreadym0 input."]
    #[inline(always)]
    pub fn atreadym0(&self) -> Atreadym0R {
        Atreadym0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reads the value of the atreadym1 input."]
    #[inline(always)]
    pub fn atreadym1(&self) -> Atreadym1R {
        Atreadym1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Reads the value of the atvalids input."]
    #[inline(always)]
    pub fn atvalids(&self) -> AtvalidsR {
        AtvalidsR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reads the value of the atreadym0 input."]
    #[inline(always)]
    pub fn atreadym0(&mut self) -> Atreadym0W<'_, Itatbctr1Spec> {
        Atreadym0W::new(self, 0)
    }
    #[doc = "Bit 1 - Reads the value of the atreadym1 input."]
    #[inline(always)]
    pub fn atreadym1(&mut self) -> Atreadym1W<'_, Itatbctr1Spec> {
        Atreadym1W::new(self, 1)
    }
    #[doc = "Bit 3 - Reads the value of the atvalids input."]
    #[inline(always)]
    pub fn atvalids(&mut self) -> AtvalidsW<'_, Itatbctr1Spec> {
        AtvalidsW::new(self, 3)
    }
}
#[doc = "The ITATBCTR1 register returns the value of the atreadym0, atreadym1, and atvalids inputs in integration mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`itatbctr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itatbctr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Itatbctr1Spec;
impl crate::RegisterSpec for Itatbctr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itatbctr1::R`](R) reader structure"]
impl crate::Readable for Itatbctr1Spec {}
#[doc = "`write(|w| ..)` method takes [`itatbctr1::W`](W) writer structure"]
impl crate::Writable for Itatbctr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ITATBCTR1 to value 0"]
impl crate::Resettable for Itatbctr1Spec {}
