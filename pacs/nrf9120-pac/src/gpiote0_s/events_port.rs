#[doc = "Register `EVENTS_PORT` reader"]
pub type R = crate::R<EventsPortSpec>;
#[doc = "Register `EVENTS_PORT` writer"]
pub type W = crate::W<EventsPortSpec>;
#[doc = "Event generated from multiple input GPIO pins with SENSE mechanism enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsPort {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsPort> for bool {
    #[inline(always)]
    fn from(variant: EventsPort) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_PORT` reader - Event generated from multiple input GPIO pins with SENSE mechanism enabled"]
pub type EventsPortR = crate::BitReader<EventsPort>;
impl EventsPortR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsPort {
        match self.bits {
            false => EventsPort::NotGenerated,
            true => EventsPort::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsPort::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsPort::Generated
    }
}
#[doc = "Field `EVENTS_PORT` writer - Event generated from multiple input GPIO pins with SENSE mechanism enabled"]
pub type EventsPortW<'a, REG> = crate::BitWriter<'a, REG, EventsPort>;
impl<'a, REG> EventsPortW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsPort::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsPort::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Event generated from multiple input GPIO pins with SENSE mechanism enabled"]
    #[inline(always)]
    pub fn events_port(&self) -> EventsPortR {
        EventsPortR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Event generated from multiple input GPIO pins with SENSE mechanism enabled"]
    #[inline(always)]
    #[must_use]
    pub fn events_port(&mut self) -> EventsPortW<EventsPortSpec> {
        EventsPortW::new(self, 0)
    }
}
#[doc = "Event generated from multiple input GPIO pins with SENSE mechanism enabled\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`events_port::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`events_port::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsPortSpec;
impl crate::RegisterSpec for EventsPortSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_port::R`](R) reader structure"]
impl crate::Readable for EventsPortSpec {}
#[doc = "`write(|w| ..)` method takes [`events_port::W`](W) writer structure"]
impl crate::Writable for EventsPortSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVENTS_PORT to value 0"]
impl crate::Resettable for EventsPortSpec {
    const RESET_VALUE: u32 = 0;
}
