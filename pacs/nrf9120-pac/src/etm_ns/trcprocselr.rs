#[doc = "Register `TRCPROCSELR` reader"]
pub type R = crate::R<TrcprocselrSpec>;
#[doc = "Register `TRCPROCSELR` writer"]
pub type W = crate::W<TrcprocselrSpec>;
#[doc = "Field `PROCSEL` reader - PE select bits that select the PE to trace."]
pub type ProcselR = crate::FieldReader;
#[doc = "Field `PROCSEL` writer - PE select bits that select the PE to trace."]
pub type ProcselW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - PE select bits that select the PE to trace."]
    #[inline(always)]
    pub fn procsel(&self) -> ProcselR {
        ProcselR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - PE select bits that select the PE to trace."]
    #[inline(always)]
    pub fn procsel(&mut self) -> ProcselW<'_, TrcprocselrSpec> {
        ProcselW::new(self, 0)
    }
}
#[doc = "Controls which PE to trace. Might ignore writes when the trace unit is enabled or not idle. Before writing to this register, ensure that TRCSTATR.IDLE == 1 so that the trace unit can synchronize with the chosen PE. Implemented if TRCIDR3.NUMPROC is greater than zero.\n\nYou can [`read`](crate::Reg::read) this register and get [`trcprocselr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcprocselr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrcprocselrSpec;
impl crate::RegisterSpec for TrcprocselrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcprocselr::R`](R) reader structure"]
impl crate::Readable for TrcprocselrSpec {}
#[doc = "`write(|w| ..)` method takes [`trcprocselr::W`](W) writer structure"]
impl crate::Writable for TrcprocselrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRCPROCSELR to value 0"]
impl crate::Resettable for TrcprocselrSpec {}
