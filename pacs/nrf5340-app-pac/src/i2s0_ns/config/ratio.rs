#[doc = "Register `RATIO` reader"]
pub type R = crate::R<RatioSpec>;
#[doc = "Register `RATIO` writer"]
pub type W = crate::W<RatioSpec>;
#[doc = "MCK / LRCK ratio\n\nValue on reset: 6"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ratio {
    #[doc = "0: LRCK = MCK / 32"]
    _32x = 0,
    #[doc = "1: LRCK = MCK / 48"]
    _48x = 1,
    #[doc = "2: LRCK = MCK / 64"]
    _64x = 2,
    #[doc = "3: LRCK = MCK / 96"]
    _96x = 3,
    #[doc = "4: LRCK = MCK / 128"]
    _128x = 4,
    #[doc = "5: LRCK = MCK / 192"]
    _192x = 5,
    #[doc = "6: LRCK = MCK / 256"]
    _256x = 6,
    #[doc = "7: LRCK = MCK / 384"]
    _384x = 7,
    #[doc = "8: LRCK = MCK / 512"]
    _512x = 8,
}
impl From<Ratio> for u8 {
    #[inline(always)]
    fn from(variant: Ratio) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ratio {
    type Ux = u8;
}
impl crate::IsEnum for Ratio {}
#[doc = "Field `RATIO` reader - MCK / LRCK ratio"]
pub type RatioR = crate::FieldReader<Ratio>;
impl RatioR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ratio> {
        match self.bits {
            0 => Some(Ratio::_32x),
            1 => Some(Ratio::_48x),
            2 => Some(Ratio::_64x),
            3 => Some(Ratio::_96x),
            4 => Some(Ratio::_128x),
            5 => Some(Ratio::_192x),
            6 => Some(Ratio::_256x),
            7 => Some(Ratio::_384x),
            8 => Some(Ratio::_512x),
            _ => None,
        }
    }
    #[doc = "LRCK = MCK / 32"]
    #[inline(always)]
    pub fn is_32x(&self) -> bool {
        *self == Ratio::_32x
    }
    #[doc = "LRCK = MCK / 48"]
    #[inline(always)]
    pub fn is_48x(&self) -> bool {
        *self == Ratio::_48x
    }
    #[doc = "LRCK = MCK / 64"]
    #[inline(always)]
    pub fn is_64x(&self) -> bool {
        *self == Ratio::_64x
    }
    #[doc = "LRCK = MCK / 96"]
    #[inline(always)]
    pub fn is_96x(&self) -> bool {
        *self == Ratio::_96x
    }
    #[doc = "LRCK = MCK / 128"]
    #[inline(always)]
    pub fn is_128x(&self) -> bool {
        *self == Ratio::_128x
    }
    #[doc = "LRCK = MCK / 192"]
    #[inline(always)]
    pub fn is_192x(&self) -> bool {
        *self == Ratio::_192x
    }
    #[doc = "LRCK = MCK / 256"]
    #[inline(always)]
    pub fn is_256x(&self) -> bool {
        *self == Ratio::_256x
    }
    #[doc = "LRCK = MCK / 384"]
    #[inline(always)]
    pub fn is_384x(&self) -> bool {
        *self == Ratio::_384x
    }
    #[doc = "LRCK = MCK / 512"]
    #[inline(always)]
    pub fn is_512x(&self) -> bool {
        *self == Ratio::_512x
    }
}
#[doc = "Field `RATIO` writer - MCK / LRCK ratio"]
pub type RatioW<'a, REG> = crate::FieldWriter<'a, REG, 4, Ratio>;
impl<'a, REG> RatioW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LRCK = MCK / 32"]
    #[inline(always)]
    pub fn _32x(self) -> &'a mut crate::W<REG> {
        self.variant(Ratio::_32x)
    }
    #[doc = "LRCK = MCK / 48"]
    #[inline(always)]
    pub fn _48x(self) -> &'a mut crate::W<REG> {
        self.variant(Ratio::_48x)
    }
    #[doc = "LRCK = MCK / 64"]
    #[inline(always)]
    pub fn _64x(self) -> &'a mut crate::W<REG> {
        self.variant(Ratio::_64x)
    }
    #[doc = "LRCK = MCK / 96"]
    #[inline(always)]
    pub fn _96x(self) -> &'a mut crate::W<REG> {
        self.variant(Ratio::_96x)
    }
    #[doc = "LRCK = MCK / 128"]
    #[inline(always)]
    pub fn _128x(self) -> &'a mut crate::W<REG> {
        self.variant(Ratio::_128x)
    }
    #[doc = "LRCK = MCK / 192"]
    #[inline(always)]
    pub fn _192x(self) -> &'a mut crate::W<REG> {
        self.variant(Ratio::_192x)
    }
    #[doc = "LRCK = MCK / 256"]
    #[inline(always)]
    pub fn _256x(self) -> &'a mut crate::W<REG> {
        self.variant(Ratio::_256x)
    }
    #[doc = "LRCK = MCK / 384"]
    #[inline(always)]
    pub fn _384x(self) -> &'a mut crate::W<REG> {
        self.variant(Ratio::_384x)
    }
    #[doc = "LRCK = MCK / 512"]
    #[inline(always)]
    pub fn _512x(self) -> &'a mut crate::W<REG> {
        self.variant(Ratio::_512x)
    }
}
impl R {
    #[doc = "Bits 0:3 - MCK / LRCK ratio"]
    #[inline(always)]
    pub fn ratio(&self) -> RatioR {
        RatioR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - MCK / LRCK ratio"]
    #[inline(always)]
    pub fn ratio(&mut self) -> RatioW<'_, RatioSpec> {
        RatioW::new(self, 0)
    }
}
#[doc = "MCK / LRCK ratio\n\nYou can [`read`](crate::Reg::read) this register and get [`ratio::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ratio::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RatioSpec;
impl crate::RegisterSpec for RatioSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ratio::R`](R) reader structure"]
impl crate::Readable for RatioSpec {}
#[doc = "`write(|w| ..)` method takes [`ratio::W`](W) writer structure"]
impl crate::Writable for RatioSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RATIO to value 0x06"]
impl crate::Resettable for RatioSpec {
    const RESET_VALUE: u32 = 0x06;
}
