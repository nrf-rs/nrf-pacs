#[doc = "Register `MODECNF0` reader"]
pub type R = crate::R<Modecnf0Spec>;
#[doc = "Register `MODECNF0` writer"]
pub type W = crate::W<Modecnf0Spec>;
#[doc = "Radio ramp-up time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ru {
    #[doc = "0: Default ramp-up time (tRXEN and tTXEN), compatible with firmware written for nRF51"]
    Default = 0,
    #[doc = "1: Fast ramp-up (tRXEN,FAST and tTXEN,FAST), see electrical specification for more information"]
    Fast = 1,
}
impl From<Ru> for bool {
    #[inline(always)]
    fn from(variant: Ru) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RU` reader - Radio ramp-up time"]
pub type RuR = crate::BitReader<Ru>;
impl RuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ru {
        match self.bits {
            false => Ru::Default,
            true => Ru::Fast,
        }
    }
    #[doc = "Default ramp-up time (tRXEN and tTXEN), compatible with firmware written for nRF51"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == Ru::Default
    }
    #[doc = "Fast ramp-up (tRXEN,FAST and tTXEN,FAST), see electrical specification for more information"]
    #[inline(always)]
    pub fn is_fast(&self) -> bool {
        *self == Ru::Fast
    }
}
#[doc = "Field `RU` writer - Radio ramp-up time"]
pub type RuW<'a, REG> = crate::BitWriter<'a, REG, Ru>;
impl<'a, REG> RuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Default ramp-up time (tRXEN and tTXEN), compatible with firmware written for nRF51"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(Ru::Default)
    }
    #[doc = "Fast ramp-up (tRXEN,FAST and tTXEN,FAST), see electrical specification for more information"]
    #[inline(always)]
    pub fn fast(self) -> &'a mut crate::W<REG> {
        self.variant(Ru::Fast)
    }
}
#[doc = "Default TX value\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dtx {
    #[doc = "0: Transmit '1'"]
    B1 = 0,
    #[doc = "1: Transmit '0'"]
    B0 = 1,
    #[doc = "2: Transmit center frequency"]
    Center = 2,
}
impl From<Dtx> for u8 {
    #[inline(always)]
    fn from(variant: Dtx) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dtx {
    type Ux = u8;
}
impl crate::IsEnum for Dtx {}
#[doc = "Field `DTX` reader - Default TX value"]
pub type DtxR = crate::FieldReader<Dtx>;
impl DtxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dtx> {
        match self.bits {
            0 => Some(Dtx::B1),
            1 => Some(Dtx::B0),
            2 => Some(Dtx::Center),
            _ => None,
        }
    }
    #[doc = "Transmit '1'"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Dtx::B1
    }
    #[doc = "Transmit '0'"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Dtx::B0
    }
    #[doc = "Transmit center frequency"]
    #[inline(always)]
    pub fn is_center(&self) -> bool {
        *self == Dtx::Center
    }
}
#[doc = "Field `DTX` writer - Default TX value"]
pub type DtxW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dtx>;
impl<'a, REG> DtxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Transmit '1'"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Dtx::B1)
    }
    #[doc = "Transmit '0'"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Dtx::B0)
    }
    #[doc = "Transmit center frequency"]
    #[inline(always)]
    pub fn center(self) -> &'a mut crate::W<REG> {
        self.variant(Dtx::Center)
    }
}
impl R {
    #[doc = "Bit 0 - Radio ramp-up time"]
    #[inline(always)]
    pub fn ru(&self) -> RuR {
        RuR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:9 - Default TX value"]
    #[inline(always)]
    pub fn dtx(&self) -> DtxR {
        DtxR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Radio ramp-up time"]
    #[inline(always)]
    pub fn ru(&mut self) -> RuW<'_, Modecnf0Spec> {
        RuW::new(self, 0)
    }
    #[doc = "Bits 8:9 - Default TX value"]
    #[inline(always)]
    pub fn dtx(&mut self) -> DtxW<'_, Modecnf0Spec> {
        DtxW::new(self, 8)
    }
}
#[doc = "Radio mode configuration register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`modecnf0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modecnf0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Modecnf0Spec;
impl crate::RegisterSpec for Modecnf0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`modecnf0::R`](R) reader structure"]
impl crate::Readable for Modecnf0Spec {}
#[doc = "`write(|w| ..)` method takes [`modecnf0::W`](W) writer structure"]
impl crate::Writable for Modecnf0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MODECNF0 to value 0x0200"]
impl crate::Resettable for Modecnf0Spec {
    const RESET_VALUE: u32 = 0x0200;
}
