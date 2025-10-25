#[doc = "Register `EVENTS_OVRFLW` reader"]
pub type R = crate::R<EventsOvrflwSpec>;
#[doc = "Register `EVENTS_OVRFLW` writer"]
pub type W = crate::W<EventsOvrflwSpec>;
#[doc = "Event on counter overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsOvrflw {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsOvrflw> for bool {
    #[inline(always)]
    fn from(variant: EventsOvrflw) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_OVRFLW` reader - Event on counter overflow"]
pub type EventsOvrflwR = crate::BitReader<EventsOvrflw>;
impl EventsOvrflwR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsOvrflw {
        match self.bits {
            false => EventsOvrflw::NotGenerated,
            true => EventsOvrflw::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsOvrflw::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsOvrflw::Generated
    }
}
#[doc = "Field `EVENTS_OVRFLW` writer - Event on counter overflow"]
pub type EventsOvrflwW<'a, REG> = crate::BitWriter<'a, REG, EventsOvrflw>;
impl<'a, REG> EventsOvrflwW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsOvrflw::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsOvrflw::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Event on counter overflow"]
    #[inline(always)]
    pub fn events_ovrflw(&self) -> EventsOvrflwR {
        EventsOvrflwR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Event on counter overflow"]
    #[inline(always)]
    pub fn events_ovrflw(&mut self) -> EventsOvrflwW<'_, EventsOvrflwSpec> {
        EventsOvrflwW::new(self, 0)
    }
}
#[doc = "Event on counter overflow\n\nYou can [`read`](crate::Reg::read) this register and get [`events_ovrflw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_ovrflw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsOvrflwSpec;
impl crate::RegisterSpec for EventsOvrflwSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_ovrflw::R`](R) reader structure"]
impl crate::Readable for EventsOvrflwSpec {}
#[doc = "`write(|w| ..)` method takes [`events_ovrflw::W`](W) writer structure"]
impl crate::Writable for EventsOvrflwSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_OVRFLW to value 0"]
impl crate::Resettable for EventsOvrflwSpec {}
