#[doc = "Register `EVENTS_INEXACT` reader"]
pub type R = crate::R<EventsInexactSpec>;
#[doc = "Register `EVENTS_INEXACT` writer"]
pub type W = crate::W<EventsInexactSpec>;
#[doc = "An FPUIXC exception triggered by an inexact floating-point operation has occurred in the FPU\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsInexact {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsInexact> for bool {
    #[inline(always)]
    fn from(variant: EventsInexact) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_INEXACT` reader - An FPUIXC exception triggered by an inexact floating-point operation has occurred in the FPU"]
pub type EventsInexactR = crate::BitReader<EventsInexact>;
impl EventsInexactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsInexact {
        match self.bits {
            false => EventsInexact::NotGenerated,
            true => EventsInexact::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsInexact::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsInexact::Generated
    }
}
#[doc = "Field `EVENTS_INEXACT` writer - An FPUIXC exception triggered by an inexact floating-point operation has occurred in the FPU"]
pub type EventsInexactW<'a, REG> = crate::BitWriter<'a, REG, EventsInexact>;
impl<'a, REG> EventsInexactW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsInexact::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsInexact::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - An FPUIXC exception triggered by an inexact floating-point operation has occurred in the FPU"]
    #[inline(always)]
    pub fn events_inexact(&self) -> EventsInexactR {
        EventsInexactR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - An FPUIXC exception triggered by an inexact floating-point operation has occurred in the FPU"]
    #[inline(always)]
    pub fn events_inexact(&mut self) -> EventsInexactW<'_, EventsInexactSpec> {
        EventsInexactW::new(self, 0)
    }
}
#[doc = "An FPUIXC exception triggered by an inexact floating-point operation has occurred in the FPU\n\nYou can [`read`](crate::Reg::read) this register and get [`events_inexact::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_inexact::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsInexactSpec;
impl crate::RegisterSpec for EventsInexactSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_inexact::R`](R) reader structure"]
impl crate::Readable for EventsInexactSpec {}
#[doc = "`write(|w| ..)` method takes [`events_inexact::W`](W) writer structure"]
impl crate::Writable for EventsInexactSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_INEXACT to value 0"]
impl crate::Resettable for EventsInexactSpec {}
