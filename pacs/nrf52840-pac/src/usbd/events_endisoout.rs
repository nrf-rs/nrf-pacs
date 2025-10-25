#[doc = "Register `EVENTS_ENDISOOUT` reader"]
pub type R = crate::R<EventsEndisooutSpec>;
#[doc = "Register `EVENTS_ENDISOOUT` writer"]
pub type W = crate::W<EventsEndisooutSpec>;
#[doc = "The whole ISOOUT buffer has been consumed. The buffer can be accessed safely by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsEndisoout {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsEndisoout> for bool {
    #[inline(always)]
    fn from(variant: EventsEndisoout) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_ENDISOOUT` reader - The whole ISOOUT buffer has been consumed. The buffer can be accessed safely by software."]
pub type EventsEndisooutR = crate::BitReader<EventsEndisoout>;
impl EventsEndisooutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsEndisoout {
        match self.bits {
            false => EventsEndisoout::NotGenerated,
            true => EventsEndisoout::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsEndisoout::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsEndisoout::Generated
    }
}
#[doc = "Field `EVENTS_ENDISOOUT` writer - The whole ISOOUT buffer has been consumed. The buffer can be accessed safely by software."]
pub type EventsEndisooutW<'a, REG> = crate::BitWriter<'a, REG, EventsEndisoout>;
impl<'a, REG> EventsEndisooutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsEndisoout::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsEndisoout::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - The whole ISOOUT buffer has been consumed. The buffer can be accessed safely by software."]
    #[inline(always)]
    pub fn events_endisoout(&self) -> EventsEndisooutR {
        EventsEndisooutR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The whole ISOOUT buffer has been consumed. The buffer can be accessed safely by software."]
    #[inline(always)]
    pub fn events_endisoout(&mut self) -> EventsEndisooutW<'_, EventsEndisooutSpec> {
        EventsEndisooutW::new(self, 0)
    }
}
#[doc = "The whole ISOOUT buffer has been consumed. The buffer can be accessed safely by software.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_endisoout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_endisoout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsEndisooutSpec;
impl crate::RegisterSpec for EventsEndisooutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_endisoout::R`](R) reader structure"]
impl crate::Readable for EventsEndisooutSpec {}
#[doc = "`write(|w| ..)` method takes [`events_endisoout::W`](W) writer structure"]
impl crate::Writable for EventsEndisooutSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_ENDISOOUT to value 0"]
impl crate::Resettable for EventsEndisooutSpec {}
