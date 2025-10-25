#[doc = "Register `EXTREFSEL` reader"]
pub type R = crate::R<ExtrefselSpec>;
#[doc = "Register `EXTREFSEL` writer"]
pub type W = crate::W<ExtrefselSpec>;
#[doc = "External analog reference select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Extrefsel {
    #[doc = "0: Use AIN0 as external analog reference"]
    AnalogReference0 = 0,
    #[doc = "1: Use AIN1 as external analog reference"]
    AnalogReference1 = 1,
}
impl From<Extrefsel> for bool {
    #[inline(always)]
    fn from(variant: Extrefsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTREFSEL` reader - External analog reference select"]
pub type ExtrefselR = crate::BitReader<Extrefsel>;
impl ExtrefselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Extrefsel {
        match self.bits {
            false => Extrefsel::AnalogReference0,
            true => Extrefsel::AnalogReference1,
        }
    }
    #[doc = "Use AIN0 as external analog reference"]
    #[inline(always)]
    pub fn is_analog_reference0(&self) -> bool {
        *self == Extrefsel::AnalogReference0
    }
    #[doc = "Use AIN1 as external analog reference"]
    #[inline(always)]
    pub fn is_analog_reference1(&self) -> bool {
        *self == Extrefsel::AnalogReference1
    }
}
#[doc = "Field `EXTREFSEL` writer - External analog reference select"]
pub type ExtrefselW<'a, REG> = crate::BitWriter<'a, REG, Extrefsel>;
impl<'a, REG> ExtrefselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use AIN0 as external analog reference"]
    #[inline(always)]
    pub fn analog_reference0(self) -> &'a mut crate::W<REG> {
        self.variant(Extrefsel::AnalogReference0)
    }
    #[doc = "Use AIN1 as external analog reference"]
    #[inline(always)]
    pub fn analog_reference1(self) -> &'a mut crate::W<REG> {
        self.variant(Extrefsel::AnalogReference1)
    }
}
impl R {
    #[doc = "Bit 0 - External analog reference select"]
    #[inline(always)]
    pub fn extrefsel(&self) -> ExtrefselR {
        ExtrefselR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - External analog reference select"]
    #[inline(always)]
    pub fn extrefsel(&mut self) -> ExtrefselW<'_, ExtrefselSpec> {
        ExtrefselW::new(self, 0)
    }
}
#[doc = "External reference select\n\nYou can [`read`](crate::Reg::read) this register and get [`extrefsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extrefsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExtrefselSpec;
impl crate::RegisterSpec for ExtrefselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extrefsel::R`](R) reader structure"]
impl crate::Readable for ExtrefselSpec {}
#[doc = "`write(|w| ..)` method takes [`extrefsel::W`](W) writer structure"]
impl crate::Writable for ExtrefselSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EXTREFSEL to value 0"]
impl crate::Resettable for ExtrefselSpec {}
