#[doc = "Register `RESOLUTION` reader"]
pub type R = crate::R<ResolutionSpec>;
#[doc = "Register `RESOLUTION` writer"]
pub type W = crate::W<ResolutionSpec>;
#[doc = "Set the resolution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Val {
    #[doc = "0: 8 bit"]
    _8bit = 0,
    #[doc = "1: 10 bit"]
    _10bit = 1,
    #[doc = "2: 12 bit"]
    _12bit = 2,
    #[doc = "3: 14 bit"]
    _14bit = 3,
}
impl From<Val> for u8 {
    #[inline(always)]
    fn from(variant: Val) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Val {
    type Ux = u8;
}
impl crate::IsEnum for Val {}
#[doc = "Field `VAL` reader - Set the resolution"]
pub type ValR = crate::FieldReader<Val>;
impl ValR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Val> {
        match self.bits {
            0 => Some(Val::_8bit),
            1 => Some(Val::_10bit),
            2 => Some(Val::_12bit),
            3 => Some(Val::_14bit),
            _ => None,
        }
    }
    #[doc = "8 bit"]
    #[inline(always)]
    pub fn is_8bit(&self) -> bool {
        *self == Val::_8bit
    }
    #[doc = "10 bit"]
    #[inline(always)]
    pub fn is_10bit(&self) -> bool {
        *self == Val::_10bit
    }
    #[doc = "12 bit"]
    #[inline(always)]
    pub fn is_12bit(&self) -> bool {
        *self == Val::_12bit
    }
    #[doc = "14 bit"]
    #[inline(always)]
    pub fn is_14bit(&self) -> bool {
        *self == Val::_14bit
    }
}
#[doc = "Field `VAL` writer - Set the resolution"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 3, Val>;
impl<'a, REG> ValW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8 bit"]
    #[inline(always)]
    pub fn _8bit(self) -> &'a mut crate::W<REG> {
        self.variant(Val::_8bit)
    }
    #[doc = "10 bit"]
    #[inline(always)]
    pub fn _10bit(self) -> &'a mut crate::W<REG> {
        self.variant(Val::_10bit)
    }
    #[doc = "12 bit"]
    #[inline(always)]
    pub fn _12bit(self) -> &'a mut crate::W<REG> {
        self.variant(Val::_12bit)
    }
    #[doc = "14 bit"]
    #[inline(always)]
    pub fn _14bit(self) -> &'a mut crate::W<REG> {
        self.variant(Val::_14bit)
    }
}
impl R {
    #[doc = "Bits 0:2 - Set the resolution"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Set the resolution"]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<'_, ResolutionSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Resolution configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`resolution::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`resolution::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ResolutionSpec;
impl crate::RegisterSpec for ResolutionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resolution::R`](R) reader structure"]
impl crate::Readable for ResolutionSpec {}
#[doc = "`write(|w| ..)` method takes [`resolution::W`](W) writer structure"]
impl crate::Writable for ResolutionSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RESOLUTION to value 0x01"]
impl crate::Resettable for ResolutionSpec {
    const RESET_VALUE: u32 = 0x01;
}
