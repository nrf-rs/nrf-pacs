#[doc = "Register `EVENTS_OVERFLOW` reader"]
pub type R = crate::R<EventsOverflowSpec>;
#[doc = "Register `EVENTS_OVERFLOW` writer"]
pub type W = crate::W<EventsOverflowSpec>;
#[doc = "An FPUOFC exception triggered by a floating-point overflow has occurred in the FPU\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsOverflow {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsOverflow> for bool {
    #[inline(always)]
    fn from(variant: EventsOverflow) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_OVERFLOW` reader - An FPUOFC exception triggered by a floating-point overflow has occurred in the FPU"]
pub type EventsOverflowR = crate::BitReader<EventsOverflow>;
impl EventsOverflowR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsOverflow {
        match self.bits {
            false => EventsOverflow::NotGenerated,
            true => EventsOverflow::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsOverflow::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsOverflow::Generated
    }
}
#[doc = "Field `EVENTS_OVERFLOW` writer - An FPUOFC exception triggered by a floating-point overflow has occurred in the FPU"]
pub type EventsOverflowW<'a, REG> = crate::BitWriter<'a, REG, EventsOverflow>;
impl<'a, REG> EventsOverflowW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsOverflow::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsOverflow::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - An FPUOFC exception triggered by a floating-point overflow has occurred in the FPU"]
    #[inline(always)]
    pub fn events_overflow(&self) -> EventsOverflowR {
        EventsOverflowR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - An FPUOFC exception triggered by a floating-point overflow has occurred in the FPU"]
    #[inline(always)]
    pub fn events_overflow(&mut self) -> EventsOverflowW<'_, EventsOverflowSpec> {
        EventsOverflowW::new(self, 0)
    }
}
#[doc = "An FPUOFC exception triggered by a floating-point overflow has occurred in the FPU\n\nYou can [`read`](crate::Reg::read) this register and get [`events_overflow::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_overflow::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsOverflowSpec;
impl crate::RegisterSpec for EventsOverflowSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_overflow::R`](R) reader structure"]
impl crate::Readable for EventsOverflowSpec {}
#[doc = "`write(|w| ..)` method takes [`events_overflow::W`](W) writer structure"]
impl crate::Writable for EventsOverflowSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_OVERFLOW to value 0"]
impl crate::Resettable for EventsOverflowSpec {}
