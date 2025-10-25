#[doc = "Register `ITATBCTR2` reader"]
pub type R = crate::R<Itatbctr2Spec>;
#[doc = "Register `ITATBCTR2` writer"]
pub type W = crate::W<Itatbctr2Spec>;
#[doc = "Sets the value of afvalid.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Atready {
    #[doc = "0: Pin is logic 0."]
    Low = 0,
    #[doc = "1: Pin is logic 1."]
    High = 1,
}
impl From<Atready> for bool {
    #[inline(always)]
    fn from(variant: Atready) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ATREADY` reader - Sets the value of afvalid."]
pub type AtreadyR = crate::BitReader<Atready>;
impl AtreadyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Atready {
        match self.bits {
            false => Atready::Low,
            true => Atready::High,
        }
    }
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Atready::Low
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Atready::High
    }
}
#[doc = "Field `ATREADY` writer - Sets the value of afvalid."]
pub type AtreadyW<'a, REG> = crate::BitWriter<'a, REG, Atready>;
impl<'a, REG> AtreadyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Atready::Low)
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Atready::High)
    }
}
#[doc = "Sets the value of atready.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Afvalid {
    #[doc = "0: Pin is logic 0."]
    Low = 0,
    #[doc = "1: Pin is logic 1."]
    High = 1,
}
impl From<Afvalid> for bool {
    #[inline(always)]
    fn from(variant: Afvalid) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AFVALID` reader - Sets the value of atready."]
pub type AfvalidR = crate::BitReader<Afvalid>;
impl AfvalidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Afvalid {
        match self.bits {
            false => Afvalid::Low,
            true => Afvalid::High,
        }
    }
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Afvalid::Low
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Afvalid::High
    }
}
#[doc = "Field `AFVALID` writer - Sets the value of atready."]
pub type AfvalidW<'a, REG> = crate::BitWriter<'a, REG, Afvalid>;
impl<'a, REG> AfvalidW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Afvalid::Low)
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Afvalid::High)
    }
}
impl R {
    #[doc = "Bit 0 - Sets the value of afvalid."]
    #[inline(always)]
    pub fn atready(&self) -> AtreadyR {
        AtreadyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sets the value of atready."]
    #[inline(always)]
    pub fn afvalid(&self) -> AfvalidR {
        AfvalidR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sets the value of afvalid."]
    #[inline(always)]
    pub fn atready(&mut self) -> AtreadyW<'_, Itatbctr2Spec> {
        AtreadyW::new(self, 0)
    }
    #[doc = "Bit 1 - Sets the value of atready."]
    #[inline(always)]
    pub fn afvalid(&mut self) -> AfvalidW<'_, Itatbctr2Spec> {
        AfvalidW::new(self, 1)
    }
}
#[doc = "Enables control of the atreadys and afvalids outputs of the TPIU.\n\nYou can [`read`](crate::Reg::read) this register and get [`itatbctr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itatbctr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Itatbctr2Spec;
impl crate::RegisterSpec for Itatbctr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itatbctr2::R`](R) reader structure"]
impl crate::Readable for Itatbctr2Spec {}
#[doc = "`write(|w| ..)` method takes [`itatbctr2::W`](W) writer structure"]
impl crate::Writable for Itatbctr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ITATBCTR2 to value 0"]
impl crate::Resettable for Itatbctr2Spec {}
