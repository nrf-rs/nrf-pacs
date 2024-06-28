#[doc = "Register `RATIO` reader"]
pub type R = crate::R<RatioSpec>;
#[doc = "Register `RATIO` writer"]
pub type W = crate::W<RatioSpec>;
#[doc = "Selects the ratio between PDM_CLK and output sample rate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ratio {
    #[doc = "0: Ratio of 64"]
    Ratio64 = 0,
    #[doc = "1: Ratio of 80"]
    Ratio80 = 1,
}
impl From<Ratio> for bool {
    #[inline(always)]
    fn from(variant: Ratio) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RATIO` reader - Selects the ratio between PDM_CLK and output sample rate"]
pub type RatioR = crate::BitReader<Ratio>;
impl RatioR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ratio {
        match self.bits {
            false => Ratio::Ratio64,
            true => Ratio::Ratio80,
        }
    }
    #[doc = "Ratio of 64"]
    #[inline(always)]
    pub fn is_ratio64(&self) -> bool {
        *self == Ratio::Ratio64
    }
    #[doc = "Ratio of 80"]
    #[inline(always)]
    pub fn is_ratio80(&self) -> bool {
        *self == Ratio::Ratio80
    }
}
#[doc = "Field `RATIO` writer - Selects the ratio between PDM_CLK and output sample rate"]
pub type RatioW<'a, REG> = crate::BitWriter<'a, REG, Ratio>;
impl<'a, REG> RatioW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ratio of 64"]
    #[inline(always)]
    pub fn ratio64(self) -> &'a mut crate::W<REG> {
        self.variant(Ratio::Ratio64)
    }
    #[doc = "Ratio of 80"]
    #[inline(always)]
    pub fn ratio80(self) -> &'a mut crate::W<REG> {
        self.variant(Ratio::Ratio80)
    }
}
impl R {
    #[doc = "Bit 0 - Selects the ratio between PDM_CLK and output sample rate"]
    #[inline(always)]
    pub fn ratio(&self) -> RatioR {
        RatioR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selects the ratio between PDM_CLK and output sample rate"]
    #[inline(always)]
    #[must_use]
    pub fn ratio(&mut self) -> RatioW<RatioSpec> {
        RatioW::new(self, 0)
    }
}
#[doc = "Selects the ratio between PDM_CLK and output sample rate. Change PDMCLKCTRL accordingly.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ratio::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ratio::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RatioSpec;
impl crate::RegisterSpec for RatioSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ratio::R`](R) reader structure"]
impl crate::Readable for RatioSpec {}
#[doc = "`write(|w| ..)` method takes [`ratio::W`](W) writer structure"]
impl crate::Writable for RatioSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RATIO to value 0"]
impl crate::Resettable for RatioSpec {
    const RESET_VALUE: u32 = 0;
}
