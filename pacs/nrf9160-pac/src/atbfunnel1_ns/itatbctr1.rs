#[doc = "Register `ITATBCTR1` reader"]
pub type R = crate::R<Itatbctr1Spec>;
#[doc = "Register `ITATBCTR1` writer"]
pub type W = crate::W<Itatbctr1Spec>;
#[doc = "A read returns the value of the atids\\[n\\] signals, where the value of the Control Register at 0x000 defines n. A write outputs the value to the atidm port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Atvalidm0 {
    #[doc = "0: Pin is logic 0."]
    Low = 0,
    #[doc = "1: Pin is logic 1."]
    High = 1,
}
impl From<Atvalidm0> for u8 {
    #[inline(always)]
    fn from(variant: Atvalidm0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Atvalidm0 {
    type Ux = u8;
}
impl crate::IsEnum for Atvalidm0 {}
#[doc = "Field `ATVALIDM0` reader - A read returns the value of the atids\\[n\\] signals, where the value of the Control Register at 0x000 defines n. A write outputs the value to the atidm port."]
pub type Atvalidm0R = crate::FieldReader<Atvalidm0>;
impl Atvalidm0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Atvalidm0> {
        match self.bits {
            0 => Some(Atvalidm0::Low),
            1 => Some(Atvalidm0::High),
            _ => None,
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
#[doc = "Field `ATVALIDM0` writer - A read returns the value of the atids\\[n\\] signals, where the value of the Control Register at 0x000 defines n. A write outputs the value to the atidm port."]
pub type Atvalidm0W<'a, REG> = crate::FieldWriter<'a, REG, 7, Atvalidm0>;
impl<'a, REG> Atvalidm0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
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
impl R {
    #[doc = "Bits 0:6 - A read returns the value of the atids\\[n\\] signals, where the value of the Control Register at 0x000 defines n. A write outputs the value to the atidm port."]
    #[inline(always)]
    pub fn atvalidm0(&self) -> Atvalidm0R {
        Atvalidm0R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - A read returns the value of the atids\\[n\\] signals, where the value of the Control Register at 0x000 defines n. A write outputs the value to the atidm port."]
    #[inline(always)]
    pub fn atvalidm0(&mut self) -> Atvalidm0W<'_, Itatbctr1Spec> {
        Atvalidm0W::new(self, 0)
    }
}
#[doc = "The ITATBCTR1 register performs different functions depending on whether the access is a read or a write.\n\nYou can [`read`](crate::Reg::read) this register and get [`itatbctr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itatbctr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
