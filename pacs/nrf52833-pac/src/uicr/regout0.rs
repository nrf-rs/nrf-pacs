#[doc = "Register `REGOUT0` reader"]
pub type R = crate::R<Regout0Spec>;
#[doc = "Register `REGOUT0` writer"]
pub type W = crate::W<Regout0Spec>;
#[doc = "Output voltage from REG0 regulator stage.\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Vout {
    #[doc = "0: 1.8 V"]
    _1v8 = 0,
    #[doc = "1: 2.1 V"]
    _2v1 = 1,
    #[doc = "2: 2.4 V"]
    _2v4 = 2,
    #[doc = "3: 2.7 V"]
    _2v7 = 3,
    #[doc = "4: 3.0 V"]
    _3v0 = 4,
    #[doc = "5: 3.3 V"]
    _3v3 = 5,
    #[doc = "7: Default voltage: 1.8 V"]
    Default = 7,
}
impl From<Vout> for u8 {
    #[inline(always)]
    fn from(variant: Vout) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Vout {
    type Ux = u8;
}
impl crate::IsEnum for Vout {}
#[doc = "Field `VOUT` reader - Output voltage from REG0 regulator stage."]
pub type VoutR = crate::FieldReader<Vout>;
impl VoutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Vout> {
        match self.bits {
            0 => Some(Vout::_1v8),
            1 => Some(Vout::_2v1),
            2 => Some(Vout::_2v4),
            3 => Some(Vout::_2v7),
            4 => Some(Vout::_3v0),
            5 => Some(Vout::_3v3),
            7 => Some(Vout::Default),
            _ => None,
        }
    }
    #[doc = "1.8 V"]
    #[inline(always)]
    pub fn is_1v8(&self) -> bool {
        *self == Vout::_1v8
    }
    #[doc = "2.1 V"]
    #[inline(always)]
    pub fn is_2v1(&self) -> bool {
        *self == Vout::_2v1
    }
    #[doc = "2.4 V"]
    #[inline(always)]
    pub fn is_2v4(&self) -> bool {
        *self == Vout::_2v4
    }
    #[doc = "2.7 V"]
    #[inline(always)]
    pub fn is_2v7(&self) -> bool {
        *self == Vout::_2v7
    }
    #[doc = "3.0 V"]
    #[inline(always)]
    pub fn is_3v0(&self) -> bool {
        *self == Vout::_3v0
    }
    #[doc = "3.3 V"]
    #[inline(always)]
    pub fn is_3v3(&self) -> bool {
        *self == Vout::_3v3
    }
    #[doc = "Default voltage: 1.8 V"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == Vout::Default
    }
}
#[doc = "Field `VOUT` writer - Output voltage from REG0 regulator stage."]
pub type VoutW<'a, REG> = crate::FieldWriter<'a, REG, 3, Vout>;
impl<'a, REG> VoutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1.8 V"]
    #[inline(always)]
    pub fn _1v8(self) -> &'a mut crate::W<REG> {
        self.variant(Vout::_1v8)
    }
    #[doc = "2.1 V"]
    #[inline(always)]
    pub fn _2v1(self) -> &'a mut crate::W<REG> {
        self.variant(Vout::_2v1)
    }
    #[doc = "2.4 V"]
    #[inline(always)]
    pub fn _2v4(self) -> &'a mut crate::W<REG> {
        self.variant(Vout::_2v4)
    }
    #[doc = "2.7 V"]
    #[inline(always)]
    pub fn _2v7(self) -> &'a mut crate::W<REG> {
        self.variant(Vout::_2v7)
    }
    #[doc = "3.0 V"]
    #[inline(always)]
    pub fn _3v0(self) -> &'a mut crate::W<REG> {
        self.variant(Vout::_3v0)
    }
    #[doc = "3.3 V"]
    #[inline(always)]
    pub fn _3v3(self) -> &'a mut crate::W<REG> {
        self.variant(Vout::_3v3)
    }
    #[doc = "Default voltage: 1.8 V"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(Vout::Default)
    }
}
impl R {
    #[doc = "Bits 0:2 - Output voltage from REG0 regulator stage."]
    #[inline(always)]
    pub fn vout(&self) -> VoutR {
        VoutR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Output voltage from REG0 regulator stage."]
    #[inline(always)]
    pub fn vout(&mut self) -> VoutW<'_, Regout0Spec> {
        VoutW::new(self, 0)
    }
}
#[doc = "Output voltage from REG0 regulator stage. The maximum output voltage from this stage is given as VDDH - V_VDDH-VDD.\n\nYou can [`read`](crate::Reg::read) this register and get [`regout0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`regout0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Regout0Spec;
impl crate::RegisterSpec for Regout0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regout0::R`](R) reader structure"]
impl crate::Readable for Regout0Spec {}
#[doc = "`write(|w| ..)` method takes [`regout0::W`](W) writer structure"]
impl crate::Writable for Regout0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGOUT0 to value 0xffff_ffff"]
impl crate::Resettable for Regout0Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
