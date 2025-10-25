#[doc = "Register `ITATBCTR1` reader"]
pub type R = crate::R<Itatbctr1Spec>;
#[doc = "Register `ITATBCTR1` writer"]
pub type W = crate::W<Itatbctr1Spec>;
#[doc = "Reads the value of atids.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Atid {
    #[doc = "0: Pin is logic 0."]
    Low = 0,
    #[doc = "1: Pin is logic 1."]
    High = 1,
}
impl From<Atid> for u8 {
    #[inline(always)]
    fn from(variant: Atid) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Atid {
    type Ux = u8;
}
impl crate::IsEnum for Atid {}
#[doc = "Field `ATID` reader - Reads the value of atids."]
pub type AtidR = crate::FieldReader<Atid>;
impl AtidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Atid> {
        match self.bits {
            0 => Some(Atid::Low),
            1 => Some(Atid::High),
            _ => None,
        }
    }
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Atid::Low
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Atid::High
    }
}
#[doc = "Field `ATID` writer - Reads the value of atids."]
pub type AtidW<'a, REG> = crate::FieldWriter<'a, REG, 7, Atid>;
impl<'a, REG> AtidW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Atid::Low)
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Atid::High)
    }
}
impl R {
    #[doc = "Bits 0:6 - Reads the value of atids."]
    #[inline(always)]
    pub fn atid(&self) -> AtidR {
        AtidR::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Reads the value of atids."]
    #[inline(always)]
    pub fn atid(&mut self) -> AtidW<'_, Itatbctr1Spec> {
        AtidW::new(self, 0)
    }
}
#[doc = "The ITATBCTR1 register contains the value of the atids input to the TPIU. This is only valid when atvalids is HIGH.\n\nYou can [`read`](crate::Reg::read) this register and get [`itatbctr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itatbctr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
