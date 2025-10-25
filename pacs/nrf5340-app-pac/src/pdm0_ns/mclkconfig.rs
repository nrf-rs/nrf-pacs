#[doc = "Register `MCLKCONFIG` reader"]
pub type R = crate::R<MclkconfigSpec>;
#[doc = "Register `MCLKCONFIG` writer"]
pub type W = crate::W<MclkconfigSpec>;
#[doc = "Master clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Src {
    #[doc = "0: 32 MHz peripheral clock"]
    Pclk32m = 0,
    #[doc = "1: Audio PLL clock"]
    Aclk = 1,
}
impl From<Src> for bool {
    #[inline(always)]
    fn from(variant: Src) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRC` reader - Master clock source selection"]
pub type SrcR = crate::BitReader<Src>;
impl SrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Src {
        match self.bits {
            false => Src::Pclk32m,
            true => Src::Aclk,
        }
    }
    #[doc = "32 MHz peripheral clock"]
    #[inline(always)]
    pub fn is_pclk32m(&self) -> bool {
        *self == Src::Pclk32m
    }
    #[doc = "Audio PLL clock"]
    #[inline(always)]
    pub fn is_aclk(&self) -> bool {
        *self == Src::Aclk
    }
}
#[doc = "Field `SRC` writer - Master clock source selection"]
pub type SrcW<'a, REG> = crate::BitWriter<'a, REG, Src>;
impl<'a, REG> SrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "32 MHz peripheral clock"]
    #[inline(always)]
    pub fn pclk32m(self) -> &'a mut crate::W<REG> {
        self.variant(Src::Pclk32m)
    }
    #[doc = "Audio PLL clock"]
    #[inline(always)]
    pub fn aclk(self) -> &'a mut crate::W<REG> {
        self.variant(Src::Aclk)
    }
}
impl R {
    #[doc = "Bit 0 - Master clock source selection"]
    #[inline(always)]
    pub fn src(&self) -> SrcR {
        SrcR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Master clock source selection"]
    #[inline(always)]
    pub fn src(&mut self) -> SrcW<'_, MclkconfigSpec> {
        SrcW::new(self, 0)
    }
}
#[doc = "Master clock generator configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`mclkconfig::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mclkconfig::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MclkconfigSpec;
impl crate::RegisterSpec for MclkconfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mclkconfig::R`](R) reader structure"]
impl crate::Readable for MclkconfigSpec {}
#[doc = "`write(|w| ..)` method takes [`mclkconfig::W`](W) writer structure"]
impl crate::Writable for MclkconfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MCLKCONFIG to value 0"]
impl crate::Resettable for MclkconfigSpec {}
