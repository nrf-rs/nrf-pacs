#[doc = "Register `EVENTS_USBRESET` reader"]
pub type R = crate::R<EventsUsbresetSpec>;
#[doc = "Register `EVENTS_USBRESET` writer"]
pub type W = crate::W<EventsUsbresetSpec>;
#[doc = "Signals that a USB reset condition has been detected on USB lines\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsUsbreset {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsUsbreset> for bool {
    #[inline(always)]
    fn from(variant: EventsUsbreset) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_USBRESET` reader - Signals that a USB reset condition has been detected on USB lines"]
pub type EventsUsbresetR = crate::BitReader<EventsUsbreset>;
impl EventsUsbresetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsUsbreset {
        match self.bits {
            false => EventsUsbreset::NotGenerated,
            true => EventsUsbreset::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsUsbreset::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsUsbreset::Generated
    }
}
#[doc = "Field `EVENTS_USBRESET` writer - Signals that a USB reset condition has been detected on USB lines"]
pub type EventsUsbresetW<'a, REG> = crate::BitWriter<'a, REG, EventsUsbreset>;
impl<'a, REG> EventsUsbresetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsUsbreset::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsUsbreset::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Signals that a USB reset condition has been detected on USB lines"]
    #[inline(always)]
    pub fn events_usbreset(&self) -> EventsUsbresetR {
        EventsUsbresetR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Signals that a USB reset condition has been detected on USB lines"]
    #[inline(always)]
    pub fn events_usbreset(&mut self) -> EventsUsbresetW<'_, EventsUsbresetSpec> {
        EventsUsbresetW::new(self, 0)
    }
}
#[doc = "Signals that a USB reset condition has been detected on USB lines\n\nYou can [`read`](crate::Reg::read) this register and get [`events_usbreset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_usbreset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsUsbresetSpec;
impl crate::RegisterSpec for EventsUsbresetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_usbreset::R`](R) reader structure"]
impl crate::Readable for EventsUsbresetSpec {}
#[doc = "`write(|w| ..)` method takes [`events_usbreset::W`](W) writer structure"]
impl crate::Writable for EventsUsbresetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_USBRESET to value 0"]
impl crate::Resettable for EventsUsbresetSpec {}
