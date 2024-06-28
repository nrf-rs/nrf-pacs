#[doc = "Register `TRACEPORTSPEED` reader"]
pub type R = crate::R<TraceportspeedSpec>;
#[doc = "Register `TRACEPORTSPEED` writer"]
pub type W = crate::W<TraceportspeedSpec>;
#[doc = "Speed of Trace Port clock. Note that the TRACECLK pin output will be divided again by two from the Trace Port clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Traceportspeed {
    #[doc = "0: Trace Port clock is: 32MHz"]
    _32mhz = 0,
    #[doc = "1: Trace Port clock is: 16MHz"]
    _16mhz = 1,
    #[doc = "2: Trace Port clock is: 8MHz"]
    _8mhz = 2,
    #[doc = "3: Trace Port clock is: 4MHz"]
    _4mhz = 3,
}
impl From<Traceportspeed> for u8 {
    #[inline(always)]
    fn from(variant: Traceportspeed) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Traceportspeed {
    type Ux = u8;
}
impl crate::IsEnum for Traceportspeed {}
#[doc = "Field `TRACEPORTSPEED` reader - Speed of Trace Port clock. Note that the TRACECLK pin output will be divided again by two from the Trace Port clock."]
pub type TraceportspeedR = crate::FieldReader<Traceportspeed>;
impl TraceportspeedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Traceportspeed {
        match self.bits {
            0 => Traceportspeed::_32mhz,
            1 => Traceportspeed::_16mhz,
            2 => Traceportspeed::_8mhz,
            3 => Traceportspeed::_4mhz,
            _ => unreachable!(),
        }
    }
    #[doc = "Trace Port clock is: 32MHz"]
    #[inline(always)]
    pub fn is_32mhz(&self) -> bool {
        *self == Traceportspeed::_32mhz
    }
    #[doc = "Trace Port clock is: 16MHz"]
    #[inline(always)]
    pub fn is_16mhz(&self) -> bool {
        *self == Traceportspeed::_16mhz
    }
    #[doc = "Trace Port clock is: 8MHz"]
    #[inline(always)]
    pub fn is_8mhz(&self) -> bool {
        *self == Traceportspeed::_8mhz
    }
    #[doc = "Trace Port clock is: 4MHz"]
    #[inline(always)]
    pub fn is_4mhz(&self) -> bool {
        *self == Traceportspeed::_4mhz
    }
}
#[doc = "Field `TRACEPORTSPEED` writer - Speed of Trace Port clock. Note that the TRACECLK pin output will be divided again by two from the Trace Port clock."]
pub type TraceportspeedW<'a, REG> = crate::FieldWriter<'a, REG, 2, Traceportspeed, crate::Safe>;
impl<'a, REG> TraceportspeedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Trace Port clock is: 32MHz"]
    #[inline(always)]
    pub fn _32mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Traceportspeed::_32mhz)
    }
    #[doc = "Trace Port clock is: 16MHz"]
    #[inline(always)]
    pub fn _16mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Traceportspeed::_16mhz)
    }
    #[doc = "Trace Port clock is: 8MHz"]
    #[inline(always)]
    pub fn _8mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Traceportspeed::_8mhz)
    }
    #[doc = "Trace Port clock is: 4MHz"]
    #[inline(always)]
    pub fn _4mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Traceportspeed::_4mhz)
    }
}
impl R {
    #[doc = "Bits 0:1 - Speed of Trace Port clock. Note that the TRACECLK pin output will be divided again by two from the Trace Port clock."]
    #[inline(always)]
    pub fn traceportspeed(&self) -> TraceportspeedR {
        TraceportspeedR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Speed of Trace Port clock. Note that the TRACECLK pin output will be divided again by two from the Trace Port clock."]
    #[inline(always)]
    #[must_use]
    pub fn traceportspeed(&mut self) -> TraceportspeedW<TraceportspeedSpec> {
        TraceportspeedW::new(self, 0)
    }
}
#[doc = "Clocking options for the Trace Port debug interface Reset behavior is the same as debug components\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`traceportspeed::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`traceportspeed::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TraceportspeedSpec;
impl crate::RegisterSpec for TraceportspeedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`traceportspeed::R`](R) reader structure"]
impl crate::Readable for TraceportspeedSpec {}
#[doc = "`write(|w| ..)` method takes [`traceportspeed::W`](W) writer structure"]
impl crate::Writable for TraceportspeedSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRACEPORTSPEED to value 0"]
impl crate::Resettable for TraceportspeedSpec {
    const RESET_VALUE: u32 = 0;
}
