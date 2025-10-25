#[doc = "Register `BITMODE` reader"]
pub type R = crate::R<BitmodeSpec>;
#[doc = "Register `BITMODE` writer"]
pub type W = crate::W<BitmodeSpec>;
#[doc = "Timer bit width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bitmode {
    #[doc = "0: 16 bit timer bit width"]
    _16bit = 0,
    #[doc = "1: 8 bit timer bit width"]
    _08bit = 1,
    #[doc = "2: 24 bit timer bit width"]
    _24bit = 2,
    #[doc = "3: 32 bit timer bit width"]
    _32bit = 3,
}
impl From<Bitmode> for u8 {
    #[inline(always)]
    fn from(variant: Bitmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bitmode {
    type Ux = u8;
}
impl crate::IsEnum for Bitmode {}
#[doc = "Field `BITMODE` reader - Timer bit width"]
pub type BitmodeR = crate::FieldReader<Bitmode>;
impl BitmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bitmode {
        match self.bits {
            0 => Bitmode::_16bit,
            1 => Bitmode::_08bit,
            2 => Bitmode::_24bit,
            3 => Bitmode::_32bit,
            _ => unreachable!(),
        }
    }
    #[doc = "16 bit timer bit width"]
    #[inline(always)]
    pub fn is_16bit(&self) -> bool {
        *self == Bitmode::_16bit
    }
    #[doc = "8 bit timer bit width"]
    #[inline(always)]
    pub fn is_08bit(&self) -> bool {
        *self == Bitmode::_08bit
    }
    #[doc = "24 bit timer bit width"]
    #[inline(always)]
    pub fn is_24bit(&self) -> bool {
        *self == Bitmode::_24bit
    }
    #[doc = "32 bit timer bit width"]
    #[inline(always)]
    pub fn is_32bit(&self) -> bool {
        *self == Bitmode::_32bit
    }
}
#[doc = "Field `BITMODE` writer - Timer bit width"]
pub type BitmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Bitmode, crate::Safe>;
impl<'a, REG> BitmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "16 bit timer bit width"]
    #[inline(always)]
    pub fn _16bit(self) -> &'a mut crate::W<REG> {
        self.variant(Bitmode::_16bit)
    }
    #[doc = "8 bit timer bit width"]
    #[inline(always)]
    pub fn _08bit(self) -> &'a mut crate::W<REG> {
        self.variant(Bitmode::_08bit)
    }
    #[doc = "24 bit timer bit width"]
    #[inline(always)]
    pub fn _24bit(self) -> &'a mut crate::W<REG> {
        self.variant(Bitmode::_24bit)
    }
    #[doc = "32 bit timer bit width"]
    #[inline(always)]
    pub fn _32bit(self) -> &'a mut crate::W<REG> {
        self.variant(Bitmode::_32bit)
    }
}
impl R {
    #[doc = "Bits 0:1 - Timer bit width"]
    #[inline(always)]
    pub fn bitmode(&self) -> BitmodeR {
        BitmodeR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Timer bit width"]
    #[inline(always)]
    pub fn bitmode(&mut self) -> BitmodeW<'_, BitmodeSpec> {
        BitmodeW::new(self, 0)
    }
}
#[doc = "Configure the number of bits used by the TIMER\n\nYou can [`read`](crate::Reg::read) this register and get [`bitmode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bitmode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BitmodeSpec;
impl crate::RegisterSpec for BitmodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bitmode::R`](R) reader structure"]
impl crate::Readable for BitmodeSpec {}
#[doc = "`write(|w| ..)` method takes [`bitmode::W`](W) writer structure"]
impl crate::Writable for BitmodeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BITMODE to value 0"]
impl crate::Resettable for BitmodeSpec {}
