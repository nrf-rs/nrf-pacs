#[doc = "Register `EVENTS_PAYLOAD` reader"]
pub type R = crate::R<EventsPayloadSpec>;
#[doc = "Register `EVENTS_PAYLOAD` writer"]
pub type W = crate::W<EventsPayloadSpec>;
#[doc = "Packet payload sent or received\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsPayload {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsPayload> for bool {
    #[inline(always)]
    fn from(variant: EventsPayload) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_PAYLOAD` reader - Packet payload sent or received"]
pub type EventsPayloadR = crate::BitReader<EventsPayload>;
impl EventsPayloadR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsPayload {
        match self.bits {
            false => EventsPayload::NotGenerated,
            true => EventsPayload::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsPayload::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsPayload::Generated
    }
}
#[doc = "Field `EVENTS_PAYLOAD` writer - Packet payload sent or received"]
pub type EventsPayloadW<'a, REG> = crate::BitWriter<'a, REG, EventsPayload>;
impl<'a, REG> EventsPayloadW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsPayload::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsPayload::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Packet payload sent or received"]
    #[inline(always)]
    pub fn events_payload(&self) -> EventsPayloadR {
        EventsPayloadR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Packet payload sent or received"]
    #[inline(always)]
    pub fn events_payload(&mut self) -> EventsPayloadW<'_, EventsPayloadSpec> {
        EventsPayloadW::new(self, 0)
    }
}
#[doc = "Packet payload sent or received\n\nYou can [`read`](crate::Reg::read) this register and get [`events_payload::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_payload::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsPayloadSpec;
impl crate::RegisterSpec for EventsPayloadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_payload::R`](R) reader structure"]
impl crate::Readable for EventsPayloadSpec {}
#[doc = "`write(|w| ..)` method takes [`events_payload::W`](W) writer structure"]
impl crate::Writable for EventsPayloadSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_PAYLOAD to value 0"]
impl crate::Resettable for EventsPayloadSpec {}
