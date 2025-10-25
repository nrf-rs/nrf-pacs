#[doc = "Register `VREGHVOUT` reader"]
pub type R = crate::R<VreghvoutSpec>;
#[doc = "Register `VREGHVOUT` writer"]
pub type W = crate::W<VreghvoutSpec>;
#[doc = "VREGH regulator output voltage.\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Vreghvout {
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
impl From<Vreghvout> for u8 {
    #[inline(always)]
    fn from(variant: Vreghvout) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Vreghvout {
    type Ux = u8;
}
impl crate::IsEnum for Vreghvout {}
#[doc = "Field `VREGHVOUT` reader - VREGH regulator output voltage."]
pub type VreghvoutR = crate::FieldReader<Vreghvout>;
impl VreghvoutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Vreghvout> {
        match self.bits {
            0 => Some(Vreghvout::_1v8),
            1 => Some(Vreghvout::_2v1),
            2 => Some(Vreghvout::_2v4),
            3 => Some(Vreghvout::_2v7),
            4 => Some(Vreghvout::_3v0),
            5 => Some(Vreghvout::_3v3),
            7 => Some(Vreghvout::Default),
            _ => None,
        }
    }
    #[doc = "1.8 V"]
    #[inline(always)]
    pub fn is_1v8(&self) -> bool {
        *self == Vreghvout::_1v8
    }
    #[doc = "2.1 V"]
    #[inline(always)]
    pub fn is_2v1(&self) -> bool {
        *self == Vreghvout::_2v1
    }
    #[doc = "2.4 V"]
    #[inline(always)]
    pub fn is_2v4(&self) -> bool {
        *self == Vreghvout::_2v4
    }
    #[doc = "2.7 V"]
    #[inline(always)]
    pub fn is_2v7(&self) -> bool {
        *self == Vreghvout::_2v7
    }
    #[doc = "3.0 V"]
    #[inline(always)]
    pub fn is_3v0(&self) -> bool {
        *self == Vreghvout::_3v0
    }
    #[doc = "3.3 V"]
    #[inline(always)]
    pub fn is_3v3(&self) -> bool {
        *self == Vreghvout::_3v3
    }
    #[doc = "Default voltage: 1.8 V"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == Vreghvout::Default
    }
}
#[doc = "Field `VREGHVOUT` writer - VREGH regulator output voltage."]
pub type VreghvoutW<'a, REG> = crate::FieldWriter<'a, REG, 3, Vreghvout>;
impl<'a, REG> VreghvoutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1.8 V"]
    #[inline(always)]
    pub fn _1v8(self) -> &'a mut crate::W<REG> {
        self.variant(Vreghvout::_1v8)
    }
    #[doc = "2.1 V"]
    #[inline(always)]
    pub fn _2v1(self) -> &'a mut crate::W<REG> {
        self.variant(Vreghvout::_2v1)
    }
    #[doc = "2.4 V"]
    #[inline(always)]
    pub fn _2v4(self) -> &'a mut crate::W<REG> {
        self.variant(Vreghvout::_2v4)
    }
    #[doc = "2.7 V"]
    #[inline(always)]
    pub fn _2v7(self) -> &'a mut crate::W<REG> {
        self.variant(Vreghvout::_2v7)
    }
    #[doc = "3.0 V"]
    #[inline(always)]
    pub fn _3v0(self) -> &'a mut crate::W<REG> {
        self.variant(Vreghvout::_3v0)
    }
    #[doc = "3.3 V"]
    #[inline(always)]
    pub fn _3v3(self) -> &'a mut crate::W<REG> {
        self.variant(Vreghvout::_3v3)
    }
    #[doc = "Default voltage: 1.8 V"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(Vreghvout::Default)
    }
}
impl R {
    #[doc = "Bits 0:2 - VREGH regulator output voltage."]
    #[inline(always)]
    pub fn vreghvout(&self) -> VreghvoutR {
        VreghvoutR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - VREGH regulator output voltage."]
    #[inline(always)]
    pub fn vreghvout(&mut self) -> VreghvoutW<'_, VreghvoutSpec> {
        VreghvoutW::new(self, 0)
    }
}
#[doc = "Output voltage from the high voltage (VREGH) regulator stage. The maximum output voltage from this stage is given as VDDH - VREGHDROP.\n\nYou can [`read`](crate::Reg::read) this register and get [`vreghvout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vreghvout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VreghvoutSpec;
impl crate::RegisterSpec for VreghvoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vreghvout::R`](R) reader structure"]
impl crate::Readable for VreghvoutSpec {}
#[doc = "`write(|w| ..)` method takes [`vreghvout::W`](W) writer structure"]
impl crate::Writable for VreghvoutSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VREGHVOUT to value 0xffff_ffff"]
impl crate::Resettable for VreghvoutSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
