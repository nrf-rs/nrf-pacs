#[doc = "Register `EVENTS_UNDERFLOW` reader"]
pub type R = crate::R<EventsUnderflowSpec>;
#[doc = "Register `EVENTS_UNDERFLOW` writer"]
pub type W = crate::W<EventsUnderflowSpec>;
#[doc = "An FPUUFC exception triggered by a floating-point underflow has occurred in the FPU\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsUnderflow {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsUnderflow> for bool {
    #[inline(always)]
    fn from(variant: EventsUnderflow) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_UNDERFLOW` reader - An FPUUFC exception triggered by a floating-point underflow has occurred in the FPU"]
pub type EventsUnderflowR = crate::BitReader<EventsUnderflow>;
impl EventsUnderflowR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsUnderflow {
        match self.bits {
            false => EventsUnderflow::NotGenerated,
            true => EventsUnderflow::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsUnderflow::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsUnderflow::Generated
    }
}
#[doc = "Field `EVENTS_UNDERFLOW` writer - An FPUUFC exception triggered by a floating-point underflow has occurred in the FPU"]
pub type EventsUnderflowW<'a, REG> = crate::BitWriter<'a, REG, EventsUnderflow>;
impl<'a, REG> EventsUnderflowW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsUnderflow::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsUnderflow::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - An FPUUFC exception triggered by a floating-point underflow has occurred in the FPU"]
    #[inline(always)]
    pub fn events_underflow(&self) -> EventsUnderflowR {
        EventsUnderflowR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - An FPUUFC exception triggered by a floating-point underflow has occurred in the FPU"]
    #[inline(always)]
    pub fn events_underflow(&mut self) -> EventsUnderflowW<'_, EventsUnderflowSpec> {
        EventsUnderflowW::new(self, 0)
    }
}
#[doc = "An FPUUFC exception triggered by a floating-point underflow has occurred in the FPU\n\nYou can [`read`](crate::Reg::read) this register and get [`events_underflow::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_underflow::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsUnderflowSpec;
impl crate::RegisterSpec for EventsUnderflowSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_underflow::R`](R) reader structure"]
impl crate::Readable for EventsUnderflowSpec {}
#[doc = "`write(|w| ..)` method takes [`events_underflow::W`](W) writer structure"]
impl crate::Writable for EventsUnderflowSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_UNDERFLOW to value 0"]
impl crate::Resettable for EventsUnderflowSpec {}
