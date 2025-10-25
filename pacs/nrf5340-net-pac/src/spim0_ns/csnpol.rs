#[doc = "Register `CSNPOL` reader"]
pub type R = crate::R<CsnpolSpec>;
#[doc = "Register `CSNPOL` writer"]
pub type W = crate::W<CsnpolSpec>;
#[doc = "Polarity of CSN output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Csnpol {
    #[doc = "0: Active low (idle state high)"]
    Low = 0,
    #[doc = "1: Active high (idle state low)"]
    High = 1,
}
impl From<Csnpol> for bool {
    #[inline(always)]
    fn from(variant: Csnpol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSNPOL` reader - Polarity of CSN output"]
pub type CsnpolR = crate::BitReader<Csnpol>;
impl CsnpolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Csnpol {
        match self.bits {
            false => Csnpol::Low,
            true => Csnpol::High,
        }
    }
    #[doc = "Active low (idle state high)"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Csnpol::Low
    }
    #[doc = "Active high (idle state low)"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Csnpol::High
    }
}
#[doc = "Field `CSNPOL` writer - Polarity of CSN output"]
pub type CsnpolW<'a, REG> = crate::BitWriter<'a, REG, Csnpol>;
impl<'a, REG> CsnpolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Active low (idle state high)"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Csnpol::Low)
    }
    #[doc = "Active high (idle state low)"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Csnpol::High)
    }
}
impl R {
    #[doc = "Bit 0 - Polarity of CSN output"]
    #[inline(always)]
    pub fn csnpol(&self) -> CsnpolR {
        CsnpolR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Polarity of CSN output"]
    #[inline(always)]
    pub fn csnpol(&mut self) -> CsnpolW<'_, CsnpolSpec> {
        CsnpolW::new(self, 0)
    }
}
#[doc = "Polarity of CSN output\n\nYou can [`read`](crate::Reg::read) this register and get [`csnpol::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csnpol::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsnpolSpec;
impl crate::RegisterSpec for CsnpolSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csnpol::R`](R) reader structure"]
impl crate::Readable for CsnpolSpec {}
#[doc = "`write(|w| ..)` method takes [`csnpol::W`](W) writer structure"]
impl crate::Writable for CsnpolSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSNPOL to value 0"]
impl crate::Resettable for CsnpolSpec {}
