#[doc = "Register `TRCEXTINSELR` reader"]
pub type R = crate::R<TrcextinselrSpec>;
#[doc = "Register `TRCEXTINSELR` writer"]
pub type W = crate::W<TrcextinselrSpec>;
#[doc = "Field `SEL_0` reader - Each field in this collection selects an external input as a resource for the trace unit."]
pub type Sel0R = crate::FieldReader;
#[doc = "Field `SEL_0` writer - Each field in this collection selects an external input as a resource for the trace unit."]
pub type Sel0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SEL_1` reader - Each field in this collection selects an external input as a resource for the trace unit."]
pub type Sel1R = crate::FieldReader;
#[doc = "Field `SEL_1` writer - Each field in this collection selects an external input as a resource for the trace unit."]
pub type Sel1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SEL_2` reader - Each field in this collection selects an external input as a resource for the trace unit."]
pub type Sel2R = crate::FieldReader;
#[doc = "Field `SEL_2` writer - Each field in this collection selects an external input as a resource for the trace unit."]
pub type Sel2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SEL_3` reader - Each field in this collection selects an external input as a resource for the trace unit."]
pub type Sel3R = crate::FieldReader;
#[doc = "Field `SEL_3` writer - Each field in this collection selects an external input as a resource for the trace unit."]
pub type Sel3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Each field in this collection selects an external input as a resource for the trace unit."]
    #[inline(always)]
    pub fn sel_0(&self) -> Sel0R {
        Sel0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Each field in this collection selects an external input as a resource for the trace unit."]
    #[inline(always)]
    pub fn sel_1(&self) -> Sel1R {
        Sel1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Each field in this collection selects an external input as a resource for the trace unit."]
    #[inline(always)]
    pub fn sel_2(&self) -> Sel2R {
        Sel2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Each field in this collection selects an external input as a resource for the trace unit."]
    #[inline(always)]
    pub fn sel_3(&self) -> Sel3R {
        Sel3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Each field in this collection selects an external input as a resource for the trace unit."]
    #[inline(always)]
    pub fn sel_0(&mut self) -> Sel0W<'_, TrcextinselrSpec> {
        Sel0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Each field in this collection selects an external input as a resource for the trace unit."]
    #[inline(always)]
    pub fn sel_1(&mut self) -> Sel1W<'_, TrcextinselrSpec> {
        Sel1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Each field in this collection selects an external input as a resource for the trace unit."]
    #[inline(always)]
    pub fn sel_2(&mut self) -> Sel2W<'_, TrcextinselrSpec> {
        Sel2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Each field in this collection selects an external input as a resource for the trace unit."]
    #[inline(always)]
    pub fn sel_3(&mut self) -> Sel3W<'_, TrcextinselrSpec> {
        Sel3W::new(self, 24)
    }
}
#[doc = "Use this to set, or read, which external inputs are resources to the trace unit. Might ignore writes when the trace unit is enabled or not idle. Only returns stable data when TRCSTATR.PMSTABLE == 1. When the sequencer is used, all sequencer state transitions must be programmed with a valid event.\n\nYou can [`read`](crate::Reg::read) this register and get [`trcextinselr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcextinselr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrcextinselrSpec;
impl crate::RegisterSpec for TrcextinselrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcextinselr::R`](R) reader structure"]
impl crate::Readable for TrcextinselrSpec {}
#[doc = "`write(|w| ..)` method takes [`trcextinselr::W`](W) writer structure"]
impl crate::Writable for TrcextinselrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRCEXTINSELR to value 0"]
impl crate::Resettable for TrcextinselrSpec {}
