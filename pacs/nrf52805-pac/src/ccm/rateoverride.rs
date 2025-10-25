#[doc = "Register `RATEOVERRIDE` reader"]
pub type R = crate::R<RateoverrideSpec>;
#[doc = "Register `RATEOVERRIDE` writer"]
pub type W = crate::W<RateoverrideSpec>;
#[doc = "Data rate override setting.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rateoverride {
    #[doc = "0: 1 Mbps"]
    _1mbit = 0,
    #[doc = "1: 2 Mbps"]
    _2mbit = 1,
    #[doc = "2: 125 Kbps"]
    _125kbps = 2,
    #[doc = "3: 500 Kbps"]
    _500kbps = 3,
}
impl From<Rateoverride> for u8 {
    #[inline(always)]
    fn from(variant: Rateoverride) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rateoverride {
    type Ux = u8;
}
impl crate::IsEnum for Rateoverride {}
#[doc = "Field `RATEOVERRIDE` reader - Data rate override setting."]
pub type RateoverrideR = crate::FieldReader<Rateoverride>;
impl RateoverrideR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rateoverride {
        match self.bits {
            0 => Rateoverride::_1mbit,
            1 => Rateoverride::_2mbit,
            2 => Rateoverride::_125kbps,
            3 => Rateoverride::_500kbps,
            _ => unreachable!(),
        }
    }
    #[doc = "1 Mbps"]
    #[inline(always)]
    pub fn is_1mbit(&self) -> bool {
        *self == Rateoverride::_1mbit
    }
    #[doc = "2 Mbps"]
    #[inline(always)]
    pub fn is_2mbit(&self) -> bool {
        *self == Rateoverride::_2mbit
    }
    #[doc = "125 Kbps"]
    #[inline(always)]
    pub fn is_125kbps(&self) -> bool {
        *self == Rateoverride::_125kbps
    }
    #[doc = "500 Kbps"]
    #[inline(always)]
    pub fn is_500kbps(&self) -> bool {
        *self == Rateoverride::_500kbps
    }
}
#[doc = "Field `RATEOVERRIDE` writer - Data rate override setting."]
pub type RateoverrideW<'a, REG> = crate::FieldWriter<'a, REG, 2, Rateoverride, crate::Safe>;
impl<'a, REG> RateoverrideW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 Mbps"]
    #[inline(always)]
    pub fn _1mbit(self) -> &'a mut crate::W<REG> {
        self.variant(Rateoverride::_1mbit)
    }
    #[doc = "2 Mbps"]
    #[inline(always)]
    pub fn _2mbit(self) -> &'a mut crate::W<REG> {
        self.variant(Rateoverride::_2mbit)
    }
    #[doc = "125 Kbps"]
    #[inline(always)]
    pub fn _125kbps(self) -> &'a mut crate::W<REG> {
        self.variant(Rateoverride::_125kbps)
    }
    #[doc = "500 Kbps"]
    #[inline(always)]
    pub fn _500kbps(self) -> &'a mut crate::W<REG> {
        self.variant(Rateoverride::_500kbps)
    }
}
impl R {
    #[doc = "Bits 0:1 - Data rate override setting."]
    #[inline(always)]
    pub fn rateoverride(&self) -> RateoverrideR {
        RateoverrideR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Data rate override setting."]
    #[inline(always)]
    pub fn rateoverride(&mut self) -> RateoverrideW<'_, RateoverrideSpec> {
        RateoverrideW::new(self, 0)
    }
}
#[doc = "Data rate override setting.\n\nYou can [`read`](crate::Reg::read) this register and get [`rateoverride::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rateoverride::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RateoverrideSpec;
impl crate::RegisterSpec for RateoverrideSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rateoverride::R`](R) reader structure"]
impl crate::Readable for RateoverrideSpec {}
#[doc = "`write(|w| ..)` method takes [`rateoverride::W`](W) writer structure"]
impl crate::Writable for RateoverrideSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RATEOVERRIDE to value 0"]
impl crate::Resettable for RateoverrideSpec {}
