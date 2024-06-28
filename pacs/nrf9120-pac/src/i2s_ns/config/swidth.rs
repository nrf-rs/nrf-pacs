#[doc = "Register `SWIDTH` reader"]
pub type R = crate::R<SwidthSpec>;
#[doc = "Register `SWIDTH` writer"]
pub type W = crate::W<SwidthSpec>;
#[doc = "Sample width.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Swidth {
    #[doc = "0: 8 bit."]
    _8bit = 0,
    #[doc = "1: 16 bit."]
    _16bit = 1,
    #[doc = "2: 24 bit."]
    _24bit = 2,
}
impl From<Swidth> for u8 {
    #[inline(always)]
    fn from(variant: Swidth) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Swidth {
    type Ux = u8;
}
impl crate::IsEnum for Swidth {}
#[doc = "Field `SWIDTH` reader - Sample width."]
pub type SwidthR = crate::FieldReader<Swidth>;
impl SwidthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Swidth> {
        match self.bits {
            0 => Some(Swidth::_8bit),
            1 => Some(Swidth::_16bit),
            2 => Some(Swidth::_24bit),
            _ => None,
        }
    }
    #[doc = "8 bit."]
    #[inline(always)]
    pub fn is_8bit(&self) -> bool {
        *self == Swidth::_8bit
    }
    #[doc = "16 bit."]
    #[inline(always)]
    pub fn is_16bit(&self) -> bool {
        *self == Swidth::_16bit
    }
    #[doc = "24 bit."]
    #[inline(always)]
    pub fn is_24bit(&self) -> bool {
        *self == Swidth::_24bit
    }
}
#[doc = "Field `SWIDTH` writer - Sample width."]
pub type SwidthW<'a, REG> = crate::FieldWriter<'a, REG, 2, Swidth>;
impl<'a, REG> SwidthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8 bit."]
    #[inline(always)]
    pub fn _8bit(self) -> &'a mut crate::W<REG> {
        self.variant(Swidth::_8bit)
    }
    #[doc = "16 bit."]
    #[inline(always)]
    pub fn _16bit(self) -> &'a mut crate::W<REG> {
        self.variant(Swidth::_16bit)
    }
    #[doc = "24 bit."]
    #[inline(always)]
    pub fn _24bit(self) -> &'a mut crate::W<REG> {
        self.variant(Swidth::_24bit)
    }
}
impl R {
    #[doc = "Bits 0:1 - Sample width."]
    #[inline(always)]
    pub fn swidth(&self) -> SwidthR {
        SwidthR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Sample width."]
    #[inline(always)]
    #[must_use]
    pub fn swidth(&mut self) -> SwidthW<SwidthSpec> {
        SwidthW::new(self, 0)
    }
}
#[doc = "Sample width.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swidth::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swidth::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwidthSpec;
impl crate::RegisterSpec for SwidthSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swidth::R`](R) reader structure"]
impl crate::Readable for SwidthSpec {}
#[doc = "`write(|w| ..)` method takes [`swidth::W`](W) writer structure"]
impl crate::Writable for SwidthSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWIDTH to value 0x01"]
impl crate::Resettable for SwidthSpec {
    const RESET_VALUE: u32 = 0x01;
}
