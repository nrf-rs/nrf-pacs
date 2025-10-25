#[doc = "Register `HFCLKCTRL` reader"]
pub type R = crate::R<HfclkctrlSpec>;
#[doc = "Register `HFCLKCTRL` writer"]
pub type W = crate::W<HfclkctrlSpec>;
#[doc = "High frequency clock HCLK\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hclk {
    #[doc = "0: Divide HFCLK by 1"]
    Div1 = 0,
    #[doc = "1: Divide HFCLK by 2"]
    Div2 = 1,
}
impl From<Hclk> for u8 {
    #[inline(always)]
    fn from(variant: Hclk) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hclk {
    type Ux = u8;
}
impl crate::IsEnum for Hclk {}
#[doc = "Field `HCLK` reader - High frequency clock HCLK"]
pub type HclkR = crate::FieldReader<Hclk>;
impl HclkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Hclk> {
        match self.bits {
            0 => Some(Hclk::Div1),
            1 => Some(Hclk::Div2),
            _ => None,
        }
    }
    #[doc = "Divide HFCLK by 1"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Hclk::Div1
    }
    #[doc = "Divide HFCLK by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Hclk::Div2
    }
}
#[doc = "Field `HCLK` writer - High frequency clock HCLK"]
pub type HclkW<'a, REG> = crate::FieldWriter<'a, REG, 2, Hclk>;
impl<'a, REG> HclkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divide HFCLK by 1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Hclk::Div1)
    }
    #[doc = "Divide HFCLK by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Hclk::Div2)
    }
}
impl R {
    #[doc = "Bits 0:1 - High frequency clock HCLK"]
    #[inline(always)]
    pub fn hclk(&self) -> HclkR {
        HclkR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - High frequency clock HCLK"]
    #[inline(always)]
    pub fn hclk(&mut self) -> HclkW<'_, HfclkctrlSpec> {
        HclkW::new(self, 0)
    }
}
#[doc = "HFCLK128M frequency configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`hfclkctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfclkctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HfclkctrlSpec;
impl crate::RegisterSpec for HfclkctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfclkctrl::R`](R) reader structure"]
impl crate::Readable for HfclkctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`hfclkctrl::W`](W) writer structure"]
impl crate::Writable for HfclkctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HFCLKCTRL to value 0x01"]
impl crate::Resettable for HfclkctrlSpec {
    const RESET_VALUE: u32 = 0x01;
}
