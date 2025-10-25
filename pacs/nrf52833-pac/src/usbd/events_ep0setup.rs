#[doc = "Register `EVENTS_EP0SETUP` reader"]
pub type R = crate::R<EventsEp0setupSpec>;
#[doc = "Register `EVENTS_EP0SETUP` writer"]
pub type W = crate::W<EventsEp0setupSpec>;
#[doc = "A valid SETUP token has been received (and acknowledged) on the control endpoint\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsEp0setup {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsEp0setup> for bool {
    #[inline(always)]
    fn from(variant: EventsEp0setup) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_EP0SETUP` reader - A valid SETUP token has been received (and acknowledged) on the control endpoint"]
pub type EventsEp0setupR = crate::BitReader<EventsEp0setup>;
impl EventsEp0setupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsEp0setup {
        match self.bits {
            false => EventsEp0setup::NotGenerated,
            true => EventsEp0setup::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsEp0setup::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsEp0setup::Generated
    }
}
#[doc = "Field `EVENTS_EP0SETUP` writer - A valid SETUP token has been received (and acknowledged) on the control endpoint"]
pub type EventsEp0setupW<'a, REG> = crate::BitWriter<'a, REG, EventsEp0setup>;
impl<'a, REG> EventsEp0setupW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsEp0setup::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsEp0setup::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - A valid SETUP token has been received (and acknowledged) on the control endpoint"]
    #[inline(always)]
    pub fn events_ep0setup(&self) -> EventsEp0setupR {
        EventsEp0setupR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - A valid SETUP token has been received (and acknowledged) on the control endpoint"]
    #[inline(always)]
    pub fn events_ep0setup(&mut self) -> EventsEp0setupW<'_, EventsEp0setupSpec> {
        EventsEp0setupW::new(self, 0)
    }
}
#[doc = "A valid SETUP token has been received (and acknowledged) on the control endpoint\n\nYou can [`read`](crate::Reg::read) this register and get [`events_ep0setup::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_ep0setup::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsEp0setupSpec;
impl crate::RegisterSpec for EventsEp0setupSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_ep0setup::R`](R) reader structure"]
impl crate::Readable for EventsEp0setupSpec {}
#[doc = "`write(|w| ..)` method takes [`events_ep0setup::W`](W) writer structure"]
impl crate::Writable for EventsEp0setupSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_EP0SETUP to value 0"]
impl crate::Resettable for EventsEp0setupSpec {}
