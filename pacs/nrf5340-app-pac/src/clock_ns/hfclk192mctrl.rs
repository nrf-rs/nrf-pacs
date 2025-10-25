#[doc = "Register `HFCLK192MCTRL` reader"]
pub type R = crate::R<Hfclk192mctrlSpec>;
#[doc = "Register `HFCLK192MCTRL` writer"]
pub type W = crate::W<Hfclk192mctrlSpec>;
#[doc = "High frequency clock HCLK192M\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hclk192m {
    #[doc = "0: Divide HFCLK192M by 1"]
    Div1 = 0,
    #[doc = "1: Divide HFCLK192M by 2"]
    Div2 = 1,
    #[doc = "2: Divide HFCLK192M by 4"]
    Div4 = 2,
}
impl From<Hclk192m> for u8 {
    #[inline(always)]
    fn from(variant: Hclk192m) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hclk192m {
    type Ux = u8;
}
impl crate::IsEnum for Hclk192m {}
#[doc = "Field `HCLK192M` reader - High frequency clock HCLK192M"]
pub type Hclk192mR = crate::FieldReader<Hclk192m>;
impl Hclk192mR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Hclk192m> {
        match self.bits {
            0 => Some(Hclk192m::Div1),
            1 => Some(Hclk192m::Div2),
            2 => Some(Hclk192m::Div4),
            _ => None,
        }
    }
    #[doc = "Divide HFCLK192M by 1"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Hclk192m::Div1
    }
    #[doc = "Divide HFCLK192M by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Hclk192m::Div2
    }
    #[doc = "Divide HFCLK192M by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Hclk192m::Div4
    }
}
#[doc = "Field `HCLK192M` writer - High frequency clock HCLK192M"]
pub type Hclk192mW<'a, REG> = crate::FieldWriter<'a, REG, 2, Hclk192m>;
impl<'a, REG> Hclk192mW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divide HFCLK192M by 1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Hclk192m::Div1)
    }
    #[doc = "Divide HFCLK192M by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Hclk192m::Div2)
    }
    #[doc = "Divide HFCLK192M by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Hclk192m::Div4)
    }
}
impl R {
    #[doc = "Bits 0:1 - High frequency clock HCLK192M"]
    #[inline(always)]
    pub fn hclk192m(&self) -> Hclk192mR {
        Hclk192mR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - High frequency clock HCLK192M"]
    #[inline(always)]
    pub fn hclk192m(&mut self) -> Hclk192mW<'_, Hfclk192mctrlSpec> {
        Hclk192mW::new(self, 0)
    }
}
#[doc = "HFCLK192M frequency configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`hfclk192mctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfclk192mctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hfclk192mctrlSpec;
impl crate::RegisterSpec for Hfclk192mctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfclk192mctrl::R`](R) reader structure"]
impl crate::Readable for Hfclk192mctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`hfclk192mctrl::W`](W) writer structure"]
impl crate::Writable for Hfclk192mctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HFCLK192MCTRL to value 0x02"]
impl crate::Resettable for Hfclk192mctrlSpec {
    const RESET_VALUE: u32 = 0x02;
}
