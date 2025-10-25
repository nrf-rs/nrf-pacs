#[doc = "Register `EVENTS_FIELDDETECTED` reader"]
pub type R = crate::R<EventsFielddetectedSpec>;
#[doc = "Register `EVENTS_FIELDDETECTED` writer"]
pub type W = crate::W<EventsFielddetectedSpec>;
#[doc = "Remote NFC field detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsFielddetected {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsFielddetected> for bool {
    #[inline(always)]
    fn from(variant: EventsFielddetected) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_FIELDDETECTED` reader - Remote NFC field detected"]
pub type EventsFielddetectedR = crate::BitReader<EventsFielddetected>;
impl EventsFielddetectedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsFielddetected {
        match self.bits {
            false => EventsFielddetected::NotGenerated,
            true => EventsFielddetected::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsFielddetected::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsFielddetected::Generated
    }
}
#[doc = "Field `EVENTS_FIELDDETECTED` writer - Remote NFC field detected"]
pub type EventsFielddetectedW<'a, REG> = crate::BitWriter<'a, REG, EventsFielddetected>;
impl<'a, REG> EventsFielddetectedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsFielddetected::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsFielddetected::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Remote NFC field detected"]
    #[inline(always)]
    pub fn events_fielddetected(&self) -> EventsFielddetectedR {
        EventsFielddetectedR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Remote NFC field detected"]
    #[inline(always)]
    pub fn events_fielddetected(&mut self) -> EventsFielddetectedW<'_, EventsFielddetectedSpec> {
        EventsFielddetectedW::new(self, 0)
    }
}
#[doc = "Remote NFC field detected\n\nYou can [`read`](crate::Reg::read) this register and get [`events_fielddetected::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_fielddetected::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsFielddetectedSpec;
impl crate::RegisterSpec for EventsFielddetectedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_fielddetected::R`](R) reader structure"]
impl crate::Readable for EventsFielddetectedSpec {}
#[doc = "`write(|w| ..)` method takes [`events_fielddetected::W`](W) writer structure"]
impl crate::Writable for EventsFielddetectedSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_FIELDDETECTED to value 0"]
impl crate::Resettable for EventsFielddetectedSpec {}
