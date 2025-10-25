#[doc = "Register `EVENTS_FIELDLOST` reader"]
pub type R = crate::R<EventsFieldlostSpec>;
#[doc = "Register `EVENTS_FIELDLOST` writer"]
pub type W = crate::W<EventsFieldlostSpec>;
#[doc = "Remote NFC field lost\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsFieldlost {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsFieldlost> for bool {
    #[inline(always)]
    fn from(variant: EventsFieldlost) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_FIELDLOST` reader - Remote NFC field lost"]
pub type EventsFieldlostR = crate::BitReader<EventsFieldlost>;
impl EventsFieldlostR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsFieldlost {
        match self.bits {
            false => EventsFieldlost::NotGenerated,
            true => EventsFieldlost::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsFieldlost::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsFieldlost::Generated
    }
}
#[doc = "Field `EVENTS_FIELDLOST` writer - Remote NFC field lost"]
pub type EventsFieldlostW<'a, REG> = crate::BitWriter<'a, REG, EventsFieldlost>;
impl<'a, REG> EventsFieldlostW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsFieldlost::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsFieldlost::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Remote NFC field lost"]
    #[inline(always)]
    pub fn events_fieldlost(&self) -> EventsFieldlostR {
        EventsFieldlostR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Remote NFC field lost"]
    #[inline(always)]
    pub fn events_fieldlost(&mut self) -> EventsFieldlostW<'_, EventsFieldlostSpec> {
        EventsFieldlostW::new(self, 0)
    }
}
#[doc = "Remote NFC field lost\n\nYou can [`read`](crate::Reg::read) this register and get [`events_fieldlost::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_fieldlost::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsFieldlostSpec;
impl crate::RegisterSpec for EventsFieldlostSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_fieldlost::R`](R) reader structure"]
impl crate::Readable for EventsFieldlostSpec {}
#[doc = "`write(|w| ..)` method takes [`events_fieldlost::W`](W) writer structure"]
impl crate::Writable for EventsFieldlostSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_FIELDLOST to value 0"]
impl crate::Resettable for EventsFieldlostSpec {}
