#[doc = "Register `EVENTS_FRAMESTART` reader"]
pub type R = crate::R<EventsFramestartSpec>;
#[doc = "Register `EVENTS_FRAMESTART` writer"]
pub type W = crate::W<EventsFramestartSpec>;
#[doc = "Frame start event, generated on the active edge of LRCK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsFramestart {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsFramestart> for bool {
    #[inline(always)]
    fn from(variant: EventsFramestart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_FRAMESTART` reader - Frame start event, generated on the active edge of LRCK"]
pub type EventsFramestartR = crate::BitReader<EventsFramestart>;
impl EventsFramestartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsFramestart {
        match self.bits {
            false => EventsFramestart::NotGenerated,
            true => EventsFramestart::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsFramestart::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsFramestart::Generated
    }
}
#[doc = "Field `EVENTS_FRAMESTART` writer - Frame start event, generated on the active edge of LRCK"]
pub type EventsFramestartW<'a, REG> = crate::BitWriter<'a, REG, EventsFramestart>;
impl<'a, REG> EventsFramestartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsFramestart::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsFramestart::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Frame start event, generated on the active edge of LRCK"]
    #[inline(always)]
    pub fn events_framestart(&self) -> EventsFramestartR {
        EventsFramestartR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Frame start event, generated on the active edge of LRCK"]
    #[inline(always)]
    pub fn events_framestart(&mut self) -> EventsFramestartW<'_, EventsFramestartSpec> {
        EventsFramestartW::new(self, 0)
    }
}
#[doc = "Frame start event, generated on the active edge of LRCK\n\nYou can [`read`](crate::Reg::read) this register and get [`events_framestart::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_framestart::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsFramestartSpec;
impl crate::RegisterSpec for EventsFramestartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_framestart::R`](R) reader structure"]
impl crate::Readable for EventsFramestartSpec {}
#[doc = "`write(|w| ..)` method takes [`events_framestart::W`](W) writer structure"]
impl crate::Writable for EventsFramestartSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_FRAMESTART to value 0"]
impl crate::Resettable for EventsFramestartSpec {}
