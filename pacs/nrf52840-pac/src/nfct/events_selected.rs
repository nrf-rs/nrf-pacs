#[doc = "Register `EVENTS_SELECTED` reader"]
pub type R = crate::R<EventsSelectedSpec>;
#[doc = "Register `EVENTS_SELECTED` writer"]
pub type W = crate::W<EventsSelectedSpec>;
#[doc = "NFC auto collision resolution successfully completed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsSelected {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsSelected> for bool {
    #[inline(always)]
    fn from(variant: EventsSelected) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_SELECTED` reader - NFC auto collision resolution successfully completed"]
pub type EventsSelectedR = crate::BitReader<EventsSelected>;
impl EventsSelectedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsSelected {
        match self.bits {
            false => EventsSelected::NotGenerated,
            true => EventsSelected::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsSelected::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsSelected::Generated
    }
}
#[doc = "Field `EVENTS_SELECTED` writer - NFC auto collision resolution successfully completed"]
pub type EventsSelectedW<'a, REG> = crate::BitWriter<'a, REG, EventsSelected>;
impl<'a, REG> EventsSelectedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsSelected::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsSelected::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - NFC auto collision resolution successfully completed"]
    #[inline(always)]
    pub fn events_selected(&self) -> EventsSelectedR {
        EventsSelectedR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NFC auto collision resolution successfully completed"]
    #[inline(always)]
    pub fn events_selected(&mut self) -> EventsSelectedW<'_, EventsSelectedSpec> {
        EventsSelectedW::new(self, 0)
    }
}
#[doc = "NFC auto collision resolution successfully completed\n\nYou can [`read`](crate::Reg::read) this register and get [`events_selected::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_selected::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsSelectedSpec;
impl crate::RegisterSpec for EventsSelectedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_selected::R`](R) reader structure"]
impl crate::Readable for EventsSelectedSpec {}
#[doc = "`write(|w| ..)` method takes [`events_selected::W`](W) writer structure"]
impl crate::Writable for EventsSelectedSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_SELECTED to value 0"]
impl crate::Resettable for EventsSelectedSpec {}
