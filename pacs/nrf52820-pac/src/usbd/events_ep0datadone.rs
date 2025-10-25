#[doc = "Register `EVENTS_EP0DATADONE` reader"]
pub type R = crate::R<EventsEp0datadoneSpec>;
#[doc = "Register `EVENTS_EP0DATADONE` writer"]
pub type W = crate::W<EventsEp0datadoneSpec>;
#[doc = "An acknowledged data transfer has taken place on the control endpoint\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsEp0datadone {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsEp0datadone> for bool {
    #[inline(always)]
    fn from(variant: EventsEp0datadone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_EP0DATADONE` reader - An acknowledged data transfer has taken place on the control endpoint"]
pub type EventsEp0datadoneR = crate::BitReader<EventsEp0datadone>;
impl EventsEp0datadoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsEp0datadone {
        match self.bits {
            false => EventsEp0datadone::NotGenerated,
            true => EventsEp0datadone::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsEp0datadone::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsEp0datadone::Generated
    }
}
#[doc = "Field `EVENTS_EP0DATADONE` writer - An acknowledged data transfer has taken place on the control endpoint"]
pub type EventsEp0datadoneW<'a, REG> = crate::BitWriter<'a, REG, EventsEp0datadone>;
impl<'a, REG> EventsEp0datadoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsEp0datadone::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsEp0datadone::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - An acknowledged data transfer has taken place on the control endpoint"]
    #[inline(always)]
    pub fn events_ep0datadone(&self) -> EventsEp0datadoneR {
        EventsEp0datadoneR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - An acknowledged data transfer has taken place on the control endpoint"]
    #[inline(always)]
    pub fn events_ep0datadone(&mut self) -> EventsEp0datadoneW<'_, EventsEp0datadoneSpec> {
        EventsEp0datadoneW::new(self, 0)
    }
}
#[doc = "An acknowledged data transfer has taken place on the control endpoint\n\nYou can [`read`](crate::Reg::read) this register and get [`events_ep0datadone::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_ep0datadone::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsEp0datadoneSpec;
impl crate::RegisterSpec for EventsEp0datadoneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_ep0datadone::R`](R) reader structure"]
impl crate::Readable for EventsEp0datadoneSpec {}
#[doc = "`write(|w| ..)` method takes [`events_ep0datadone::W`](W) writer structure"]
impl crate::Writable for EventsEp0datadoneSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_EP0DATADONE to value 0"]
impl crate::Resettable for EventsEp0datadoneSpec {}
