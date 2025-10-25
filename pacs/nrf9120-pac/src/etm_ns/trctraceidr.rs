#[doc = "Register `TRCTRACEIDR` reader"]
pub type R = crate::R<TrctraceidrSpec>;
#[doc = "Register `TRCTRACEIDR` writer"]
pub type W = crate::W<TrctraceidrSpec>;
#[doc = "Field `TRACEID` reader - Trace ID field. Sets the trace ID value for instruction trace. Bit\\[0\\] must be zero if data trace is enabled. If data trace is enabled then a trace unit sets the trace ID for data trace, to TRACEID+1."]
pub type TraceidR = crate::FieldReader;
#[doc = "Field `TRACEID` writer - Trace ID field. Sets the trace ID value for instruction trace. Bit\\[0\\] must be zero if data trace is enabled. If data trace is enabled then a trace unit sets the trace ID for data trace, to TRACEID+1."]
pub type TraceidW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Trace ID field. Sets the trace ID value for instruction trace. Bit\\[0\\] must be zero if data trace is enabled. If data trace is enabled then a trace unit sets the trace ID for data trace, to TRACEID+1."]
    #[inline(always)]
    pub fn traceid(&self) -> TraceidR {
        TraceidR::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Trace ID field. Sets the trace ID value for instruction trace. Bit\\[0\\] must be zero if data trace is enabled. If data trace is enabled then a trace unit sets the trace ID for data trace, to TRACEID+1."]
    #[inline(always)]
    pub fn traceid(&mut self) -> TraceidW<'_, TrctraceidrSpec> {
        TraceidW::new(self, 0)
    }
}
#[doc = "Sets the trace ID for instruction trace. If data trace is enabled then it also sets the trace ID for data trace, to (trace ID for instruction trace) + 1. This register must always be programmed as part of trace unit initialization. Might ignore writes when the trace unit is enabled or not idle.\n\nYou can [`read`](crate::Reg::read) this register and get [`trctraceidr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trctraceidr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrctraceidrSpec;
impl crate::RegisterSpec for TrctraceidrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trctraceidr::R`](R) reader structure"]
impl crate::Readable for TrctraceidrSpec {}
#[doc = "`write(|w| ..)` method takes [`trctraceidr::W`](W) writer structure"]
impl crate::Writable for TrctraceidrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRCTRACEIDR to value 0"]
impl crate::Resettable for TrctraceidrSpec {}
