#[doc = "Register `TRCSYNCPR` reader"]
pub type R = crate::R<TrcsyncprSpec>;
#[doc = "Register `TRCSYNCPR` writer"]
pub type W = crate::W<TrcsyncprSpec>;
#[doc = "Controls how many bytes of trace, the sum of instruction and data, that a trace unit can generate before a trace synchronization request occurs. The number of bytes is always a power of two, calculated by 2^PERIOD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Period {
    #[doc = "0: Trace synchronization requests are disabled. This setting does not disable other types of trace synchronization request."]
    Disabled = 0,
}
impl From<Period> for u8 {
    #[inline(always)]
    fn from(variant: Period) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Period {
    type Ux = u8;
}
impl crate::IsEnum for Period {}
#[doc = "Field `PERIOD` reader - Controls how many bytes of trace, the sum of instruction and data, that a trace unit can generate before a trace synchronization request occurs. The number of bytes is always a power of two, calculated by 2^PERIOD"]
pub type PeriodR = crate::FieldReader<Period>;
impl PeriodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Period> {
        match self.bits {
            0 => Some(Period::Disabled),
            _ => None,
        }
    }
    #[doc = "Trace synchronization requests are disabled. This setting does not disable other types of trace synchronization request."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Period::Disabled
    }
}
#[doc = "Field `PERIOD` writer - Controls how many bytes of trace, the sum of instruction and data, that a trace unit can generate before a trace synchronization request occurs. The number of bytes is always a power of two, calculated by 2^PERIOD"]
pub type PeriodW<'a, REG> = crate::FieldWriter<'a, REG, 5, Period>;
impl<'a, REG> PeriodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Trace synchronization requests are disabled. This setting does not disable other types of trace synchronization request."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Period::Disabled)
    }
}
impl R {
    #[doc = "Bits 0:4 - Controls how many bytes of trace, the sum of instruction and data, that a trace unit can generate before a trace synchronization request occurs. The number of bytes is always a power of two, calculated by 2^PERIOD"]
    #[inline(always)]
    pub fn period(&self) -> PeriodR {
        PeriodR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Controls how many bytes of trace, the sum of instruction and data, that a trace unit can generate before a trace synchronization request occurs. The number of bytes is always a power of two, calculated by 2^PERIOD"]
    #[inline(always)]
    pub fn period(&mut self) -> PeriodW<'_, TrcsyncprSpec> {
        PeriodW::new(self, 0)
    }
}
#[doc = "Controls how often trace synchronization requests occur. Might ignore writes when the trace unit is enabled or not idle. If writes are permitted then the register must be programmed.\n\nYou can [`read`](crate::Reg::read) this register and get [`trcsyncpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcsyncpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrcsyncprSpec;
impl crate::RegisterSpec for TrcsyncprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcsyncpr::R`](R) reader structure"]
impl crate::Readable for TrcsyncprSpec {}
#[doc = "`write(|w| ..)` method takes [`trcsyncpr::W`](W) writer structure"]
impl crate::Writable for TrcsyncprSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRCSYNCPR to value 0"]
impl crate::Resettable for TrcsyncprSpec {}
