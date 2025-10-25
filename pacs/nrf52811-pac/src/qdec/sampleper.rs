#[doc = "Register `SAMPLEPER` reader"]
pub type R = crate::R<SampleperSpec>;
#[doc = "Register `SAMPLEPER` writer"]
pub type W = crate::W<SampleperSpec>;
#[doc = "Sample period. The SAMPLE register will be updated for every new sample\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sampleper {
    #[doc = "0: 128 us"]
    _128us = 0,
    #[doc = "1: 256 us"]
    _256us = 1,
    #[doc = "2: 512 us"]
    _512us = 2,
    #[doc = "3: 1024 us"]
    _1024us = 3,
    #[doc = "4: 2048 us"]
    _2048us = 4,
    #[doc = "5: 4096 us"]
    _4096us = 5,
    #[doc = "6: 8192 us"]
    _8192us = 6,
    #[doc = "7: 16384 us"]
    _16384us = 7,
    #[doc = "8: 32768 us"]
    _32ms = 8,
    #[doc = "9: 65536 us"]
    _65ms = 9,
    #[doc = "10: 131072 us"]
    _131ms = 10,
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
#[doc = "Field `SAMPLEPER` reader - Sample period. The SAMPLE register will be updated for every new sample"]
pub type SampleperR = crate::FieldReader<Sampleper>;
impl SampleperR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sampleper> {
        match self.bits {
            0 => Some(Sampleper::_128us),
            1 => Some(Sampleper::_256us),
            2 => Some(Sampleper::_512us),
            3 => Some(Sampleper::_1024us),
            4 => Some(Sampleper::_2048us),
            5 => Some(Sampleper::_4096us),
            6 => Some(Sampleper::_8192us),
            7 => Some(Sampleper::_16384us),
            8 => Some(Sampleper::_32ms),
            9 => Some(Sampleper::_65ms),
            10 => Some(Sampleper::_131ms),
            _ => None,
        }
    }
    #[doc = "128 us"]
    #[inline(always)]
    pub fn is_128us(&self) -> bool {
        *self == Sampleper::_128us
    }
    #[doc = "256 us"]
    #[inline(always)]
    pub fn is_256us(&self) -> bool {
        *self == Sampleper::_256us
    }
    #[doc = "512 us"]
    #[inline(always)]
    pub fn is_512us(&self) -> bool {
        *self == Sampleper::_512us
    }
    #[doc = "1024 us"]
    #[inline(always)]
    pub fn is_1024us(&self) -> bool {
        *self == Sampleper::_1024us
    }
    #[doc = "2048 us"]
    #[inline(always)]
    pub fn is_2048us(&self) -> bool {
        *self == Sampleper::_2048us
    }
    #[doc = "4096 us"]
    #[inline(always)]
    pub fn is_4096us(&self) -> bool {
        *self == Sampleper::_4096us
    }
    #[doc = "8192 us"]
    #[inline(always)]
    pub fn is_8192us(&self) -> bool {
        *self == Sampleper::_8192us
    }
    #[doc = "16384 us"]
    #[inline(always)]
    pub fn is_16384us(&self) -> bool {
        *self == Sampleper::_16384us
    }
    #[doc = "32768 us"]
    #[inline(always)]
    pub fn is_32ms(&self) -> bool {
        *self == Sampleper::_32ms
    }
    #[doc = "65536 us"]
    #[inline(always)]
    pub fn is_65ms(&self) -> bool {
        *self == Sampleper::_65ms
    }
    #[doc = "131072 us"]
    #[inline(always)]
    pub fn is_131ms(&self) -> bool {
        *self == Sampleper::_131ms
    }
}
#[doc = "Field `SAMPLEPER` writer - Sample period. The SAMPLE register will be updated for every new sample"]
pub type SampleperW<'a, REG> = crate::FieldWriter<'a, REG, 4, Sampleper>;
impl<'a, REG> SampleperW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "128 us"]
    #[inline(always)]
    pub fn _128us(self) -> &'a mut crate::W<REG> {
        self.variant(Sampleper::_128us)
    }
    #[doc = "256 us"]
    #[inline(always)]
    pub fn _256us(self) -> &'a mut crate::W<REG> {
        self.variant(Sampleper::_256us)
    }
    #[doc = "512 us"]
    #[inline(always)]
    pub fn _512us(self) -> &'a mut crate::W<REG> {
        self.variant(Sampleper::_512us)
    }
    #[doc = "1024 us"]
    #[inline(always)]
    pub fn _1024us(self) -> &'a mut crate::W<REG> {
        self.variant(Sampleper::_1024us)
    }
    #[doc = "2048 us"]
    #[inline(always)]
    pub fn _2048us(self) -> &'a mut crate::W<REG> {
        self.variant(Sampleper::_2048us)
    }
    #[doc = "4096 us"]
    #[inline(always)]
    pub fn _4096us(self) -> &'a mut crate::W<REG> {
        self.variant(Sampleper::_4096us)
    }
    #[doc = "8192 us"]
    #[inline(always)]
    pub fn _8192us(self) -> &'a mut crate::W<REG> {
        self.variant(Sampleper::_8192us)
    }
    #[doc = "16384 us"]
    #[inline(always)]
    pub fn _16384us(self) -> &'a mut crate::W<REG> {
        self.variant(Sampleper::_16384us)
    }
    #[doc = "32768 us"]
    #[inline(always)]
    pub fn _32ms(self) -> &'a mut crate::W<REG> {
        self.variant(Sampleper::_32ms)
    }
    #[doc = "65536 us"]
    #[inline(always)]
    pub fn _65ms(self) -> &'a mut crate::W<REG> {
        self.variant(Sampleper::_65ms)
    }
    #[doc = "131072 us"]
    #[inline(always)]
    pub fn _131ms(self) -> &'a mut crate::W<REG> {
        self.variant(Sampleper::_131ms)
    }
}
impl R {
    #[doc = "Bits 0:3 - Sample period. The SAMPLE register will be updated for every new sample"]
    #[inline(always)]
    pub fn sampleper(&self) -> SampleperR {
        SampleperR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Sample period. The SAMPLE register will be updated for every new sample"]
    #[inline(always)]
    pub fn sampleper(&mut self) -> SampleperW<'_, SampleperSpec> {
        SampleperW::new(self, 0)
    }
}
#[doc = "Sample period\n\nYou can [`read`](crate::Reg::read) this register and get [`sampleper::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sampleper::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
