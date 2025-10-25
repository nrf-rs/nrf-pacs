#[doc = "Register `SAMPLEPER` reader"]
pub type R = crate::R<SampleperSpec>;
#[doc = "Register `SAMPLEPER` writer"]
pub type W = crate::W<SampleperSpec>;
#[doc = "Sample period.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sampleper {
    #[doc = "0: 128us sample period."]
    _128us = 0,
    #[doc = "1: 256us sample period."]
    _256us = 1,
    #[doc = "2: 512us sample period."]
    _512us = 2,
    #[doc = "3: 1024us sample period."]
    _1024us = 3,
    #[doc = "4: 2048us sample period."]
    _2048us = 4,
    #[doc = "5: 4096us sample period."]
    _4096us = 5,
    #[doc = "6: 8192us sample period."]
    _8192us = 6,
    #[doc = "7: 16384us sample period."]
    _16384us = 7,
}
impl From<Sampleper> for u8 {
    #[inline(always)]
    fn from(variant: Sampleper) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sampleper {
    type Ux = u8;
}
impl crate::IsEnum for Sampleper {}
#[doc = "Field `SAMPLEPER` reader - Sample period."]
pub type SampleperR = crate::FieldReader<Sampleper>;
impl SampleperR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sampleper {
        match self.bits {
            0 => Sampleper::_128us,
            1 => Sampleper::_256us,
            2 => Sampleper::_512us,
            3 => Sampleper::_1024us,
            4 => Sampleper::_2048us,
            5 => Sampleper::_4096us,
            6 => Sampleper::_8192us,
            7 => Sampleper::_16384us,
            _ => unreachable!(),
        }
    }
    #[doc = "128us sample period."]
    #[inline(always)]
    pub fn is_128us(&self) -> bool {
        *self == Sampleper::_128us
    }
    #[doc = "256us sample period."]
    #[inline(always)]
    pub fn is_256us(&self) -> bool {
        *self == Sampleper::_256us
    }
    #[doc = "512us sample period."]
    #[inline(always)]
    pub fn is_512us(&self) -> bool {
        *self == Sampleper::_512us
    }
    #[doc = "1024us sample period."]
    #[inline(always)]
    pub fn is_1024us(&self) -> bool {
        *self == Sampleper::_1024us
    }
    #[doc = "2048us sample period."]
    #[inline(always)]
    pub fn is_2048us(&self) -> bool {
        *self == Sampleper::_2048us
    }
    #[doc = "4096us sample period."]
    #[inline(always)]
    pub fn is_4096us(&self) -> bool {
        *self == Sampleper::_4096us
    }
    #[doc = "8192us sample period."]
    #[inline(always)]
    pub fn is_8192us(&self) -> bool {
        *self == Sampleper::_8192us
    }
    #[doc = "16384us sample period."]
    #[inline(always)]
    pub fn is_16384us(&self) -> bool {
        *self == Sampleper::_16384us
    }
}
#[doc = "Field `SAMPLEPER` writer - Sample period."]
pub type SampleperW<'a, REG> = crate::FieldWriter<'a, REG, 3, Sampleper, crate::Safe>;
impl<'a, REG> SampleperW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "128us sample period."]
    #[inline(always)]
    pub fn _128us(self) -> &'a mut crate::W<REG> {
        self.variant(Sampleper::_128us)
    }
    #[doc = "256us sample period."]
    #[inline(always)]
    pub fn _256us(self) -> &'a mut crate::W<REG> {
        self.variant(Sampleper::_256us)
    }
    #[doc = "512us sample period."]
    #[inline(always)]
    pub fn _512us(self) -> &'a mut crate::W<REG> {
        self.variant(Sampleper::_512us)
    }
    #[doc = "1024us sample period."]
    #[inline(always)]
    pub fn _1024us(self) -> &'a mut crate::W<REG> {
        self.variant(Sampleper::_1024us)
    }
    #[doc = "2048us sample period."]
    #[inline(always)]
    pub fn _2048us(self) -> &'a mut crate::W<REG> {
        self.variant(Sampleper::_2048us)
    }
    #[doc = "4096us sample period."]
    #[inline(always)]
    pub fn _4096us(self) -> &'a mut crate::W<REG> {
        self.variant(Sampleper::_4096us)
    }
    #[doc = "8192us sample period."]
    #[inline(always)]
    pub fn _8192us(self) -> &'a mut crate::W<REG> {
        self.variant(Sampleper::_8192us)
    }
    #[doc = "16384us sample period."]
    #[inline(always)]
    pub fn _16384us(self) -> &'a mut crate::W<REG> {
        self.variant(Sampleper::_16384us)
    }
}
impl R {
    #[doc = "Bits 0:2 - Sample period."]
    #[inline(always)]
    pub fn sampleper(&self) -> SampleperR {
        SampleperR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Sample period."]
    #[inline(always)]
    pub fn sampleper(&mut self) -> SampleperW<'_, SampleperSpec> {
        SampleperW::new(self, 0)
    }
}
#[doc = "Sample period.\n\nYou can [`read`](crate::Reg::read) this register and get [`sampleper::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sampleper::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SampleperSpec;
impl crate::RegisterSpec for SampleperSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sampleper::R`](R) reader structure"]
impl crate::Readable for SampleperSpec {}
#[doc = "`write(|w| ..)` method takes [`sampleper::W`](W) writer structure"]
impl crate::Writable for SampleperSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAMPLEPER to value 0"]
impl crate::Resettable for SampleperSpec {}
