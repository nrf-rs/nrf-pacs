#[doc = "Register `ITATBCTR0` reader"]
pub type R = crate::R<Itatbctr0Spec>;
#[doc = "Register `ITATBCTR0` writer"]
pub type W = crate::W<Itatbctr0Spec>;
#[doc = "Reads the value of atvalids.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Atvalid {
    #[doc = "0: Pin is logic 0."]
    Low = 0,
    #[doc = "1: Pin is logic 1."]
    High = 1,
}
impl From<Atvalid> for bool {
    #[inline(always)]
    fn from(variant: Atvalid) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ATVALID` reader - Reads the value of atvalids."]
pub type AtvalidR = crate::BitReader<Atvalid>;
impl AtvalidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Atvalid {
        match self.bits {
            false => Atvalid::Low,
            true => Atvalid::High,
        }
    }
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Atvalid::Low
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Atvalid::High
    }
}
#[doc = "Field `ATVALID` writer - Reads the value of atvalids."]
pub type AtvalidW<'a, REG> = crate::BitWriter<'a, REG, Atvalid>;
impl<'a, REG> AtvalidW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Atvalid::Low)
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Atvalid::High)
    }
}
#[doc = "Reads the value of afreadys.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Afready {
    #[doc = "0: Pin is logic 0."]
    Low = 0,
    #[doc = "1: Pin is logic 1."]
    High = 1,
}
impl From<Afready> for bool {
    #[inline(always)]
    fn from(variant: Afready) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AFREADY` reader - Reads the value of afreadys."]
pub type AfreadyR = crate::BitReader<Afready>;
impl AfreadyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Afready {
        match self.bits {
            false => Afready::Low,
            true => Afready::High,
        }
    }
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Afready::Low
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Afready::High
    }
}
#[doc = "Field `AFREADY` writer - Reads the value of afreadys."]
pub type AfreadyW<'a, REG> = crate::BitWriter<'a, REG, Afready>;
impl<'a, REG> AfreadyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Afready::Low)
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Afready::High)
    }
}
#[doc = "Reads the value of atbytess.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Atbytes {
    #[doc = "0: Pin is logic 0."]
    Low = 0,
    #[doc = "1: Pin is logic 1."]
    High = 1,
}
impl From<Atbytes> for u8 {
    #[inline(always)]
    fn from(variant: Atbytes) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Atbytes {
    type Ux = u8;
}
impl crate::IsEnum for Atbytes {}
#[doc = "Field `ATBYTES` reader - Reads the value of atbytess."]
pub type AtbytesR = crate::FieldReader<Atbytes>;
impl AtbytesR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Atbytes> {
        match self.bits {
            0 => Some(Atbytes::Low),
            1 => Some(Atbytes::High),
            _ => None,
        }
    }
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Atbytes::Low
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Atbytes::High
    }
}
#[doc = "Field `ATBYTES` writer - Reads the value of atbytess."]
pub type AtbytesW<'a, REG> = crate::FieldWriter<'a, REG, 2, Atbytes>;
impl<'a, REG> AtbytesW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Atbytes::Low)
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Atbytes::High)
    }
}
impl R {
    #[doc = "Bit 0 - Reads the value of atvalids."]
    #[inline(always)]
    pub fn atvalid(&self) -> AtvalidR {
        AtvalidR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Reads the value of afreadys."]
    #[inline(always)]
    pub fn afready(&self) -> AfreadyR {
        AfreadyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Reads the value of atbytess."]
    #[inline(always)]
    pub fn atbytes(&self) -> AtbytesR {
        AtbytesR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Reads the value of atvalids."]
    #[inline(always)]
    pub fn atvalid(&mut self) -> AtvalidW<'_, Itatbctr0Spec> {
        AtvalidW::new(self, 0)
    }
    #[doc = "Bit 2 - Reads the value of afreadys."]
    #[inline(always)]
    pub fn afready(&mut self) -> AfreadyW<'_, Itatbctr0Spec> {
        AfreadyW::new(self, 2)
    }
    #[doc = "Bits 8:9 - Reads the value of atbytess."]
    #[inline(always)]
    pub fn atbytes(&mut self) -> AtbytesW<'_, Itatbctr0Spec> {
        AtbytesW::new(self, 8)
    }
}
#[doc = "The ITATBCTR0 register captures the values of the atvalids, afreadys, and atbytess inputs to the TPIU. To ensure the integration registers work correctly in a system, the value of atbytess is only valid when atvalids, bit\\[0\\], is HIGH.\n\nYou can [`read`](crate::Reg::read) this register and get [`itatbctr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itatbctr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
