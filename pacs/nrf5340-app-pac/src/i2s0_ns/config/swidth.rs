#[doc = "Register `SWIDTH` reader"]
pub type R = crate::R<SwidthSpec>;
#[doc = "Register `SWIDTH` writer"]
pub type W = crate::W<SwidthSpec>;
#[doc = "Sample and half-frame width\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Swidth {
    #[doc = "0: 8 bit sample."]
    _8bit = 0,
    #[doc = "1: 16 bit sample."]
    _16bit = 1,
    #[doc = "2: 24 bit sample."]
    _24bit = 2,
    #[doc = "3: 32 bit sample."]
    _32bit = 3,
    #[doc = "4: 8 bit sample in a 16-bit half-frame."]
    _8bitIn16 = 4,
    #[doc = "5: 8 bit sample in a 32-bit half-frame."]
    _8bitIn32 = 5,
    #[doc = "6: 16 bit sample in a 32-bit half-frame."]
    _16bitIn32 = 6,
    #[doc = "7: 24 bit sample in a 32-bit half-frame."]
    _24bitIn32 = 7,
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
#[doc = "Field `SWIDTH` reader - Sample and half-frame width"]
pub type SwidthR = crate::FieldReader<Swidth>;
impl SwidthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swidth {
        match self.bits {
            0 => Swidth::_8bit,
            1 => Swidth::_16bit,
            2 => Swidth::_24bit,
            3 => Swidth::_32bit,
            4 => Swidth::_8bitIn16,
            5 => Swidth::_8bitIn32,
            6 => Swidth::_16bitIn32,
            7 => Swidth::_24bitIn32,
            _ => unreachable!(),
        }
    }
    #[doc = "8 bit sample."]
    #[inline(always)]
    pub fn is_8bit(&self) -> bool {
        *self == Swidth::_8bit
    }
    #[doc = "16 bit sample."]
    #[inline(always)]
    pub fn is_16bit(&self) -> bool {
        *self == Swidth::_16bit
    }
    #[doc = "24 bit sample."]
    #[inline(always)]
    pub fn is_24bit(&self) -> bool {
        *self == Swidth::_24bit
    }
    #[doc = "32 bit sample."]
    #[inline(always)]
    pub fn is_32bit(&self) -> bool {
        *self == Swidth::_32bit
    }
    #[doc = "8 bit sample in a 16-bit half-frame."]
    #[inline(always)]
    pub fn is_8bit_in16(&self) -> bool {
        *self == Swidth::_8bitIn16
    }
    #[doc = "8 bit sample in a 32-bit half-frame."]
    #[inline(always)]
    pub fn is_8bit_in32(&self) -> bool {
        *self == Swidth::_8bitIn32
    }
    #[doc = "16 bit sample in a 32-bit half-frame."]
    #[inline(always)]
    pub fn is_16bit_in32(&self) -> bool {
        *self == Swidth::_16bitIn32
    }
    #[doc = "24 bit sample in a 32-bit half-frame."]
    #[inline(always)]
    pub fn is_24bit_in32(&self) -> bool {
        *self == Swidth::_24bitIn32
    }
}
#[doc = "Field `SWIDTH` writer - Sample and half-frame width"]
pub type SwidthW<'a, REG> = crate::FieldWriter<'a, REG, 3, Swidth, crate::Safe>;
impl<'a, REG> SwidthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8 bit sample."]
    #[inline(always)]
    pub fn _8bit(self) -> &'a mut crate::W<REG> {
        self.variant(Swidth::_8bit)
    }
    #[doc = "16 bit sample."]
    #[inline(always)]
    pub fn _16bit(self) -> &'a mut crate::W<REG> {
        self.variant(Swidth::_16bit)
    }
    #[doc = "24 bit sample."]
    #[inline(always)]
    pub fn _24bit(self) -> &'a mut crate::W<REG> {
        self.variant(Swidth::_24bit)
    }
    #[doc = "32 bit sample."]
    #[inline(always)]
    pub fn _32bit(self) -> &'a mut crate::W<REG> {
        self.variant(Swidth::_32bit)
    }
    #[doc = "8 bit sample in a 16-bit half-frame."]
    #[inline(always)]
    pub fn _8bit_in16(self) -> &'a mut crate::W<REG> {
        self.variant(Swidth::_8bitIn16)
    }
    #[doc = "8 bit sample in a 32-bit half-frame."]
    #[inline(always)]
    pub fn _8bit_in32(self) -> &'a mut crate::W<REG> {
        self.variant(Swidth::_8bitIn32)
    }
    #[doc = "16 bit sample in a 32-bit half-frame."]
    #[inline(always)]
    pub fn _16bit_in32(self) -> &'a mut crate::W<REG> {
        self.variant(Swidth::_16bitIn32)
    }
    #[doc = "24 bit sample in a 32-bit half-frame."]
    #[inline(always)]
    pub fn _24bit_in32(self) -> &'a mut crate::W<REG> {
        self.variant(Swidth::_24bitIn32)
    }
}
impl R {
    #[doc = "Bits 0:2 - Sample and half-frame width"]
    #[inline(always)]
    pub fn swidth(&self) -> SwidthR {
        SwidthR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Sample and half-frame width"]
    #[inline(always)]
    pub fn swidth(&mut self) -> SwidthW<'_, SwidthSpec> {
        SwidthW::new(self, 0)
    }
}
#[doc = "Sample width\n\nYou can [`read`](crate::Reg::read) this register and get [`swidth::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swidth::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwidthSpec;
impl crate::RegisterSpec for SwidthSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swidth::R`](R) reader structure"]
impl crate::Readable for SwidthSpec {}
#[doc = "`write(|w| ..)` method takes [`swidth::W`](W) writer structure"]
impl crate::Writable for SwidthSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SWIDTH to value 0x01"]
impl crate::Resettable for SwidthSpec {
    const RESET_VALUE: u32 = 0x01;
}
