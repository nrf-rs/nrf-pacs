#[doc = "Register `EVENTS_CTEPRESENT` reader"]
pub type R = crate::R<EventsCtepresentSpec>;
#[doc = "Register `EVENTS_CTEPRESENT` writer"]
pub type W = crate::W<EventsCtepresentSpec>;
#[doc = "CTE is present (early warning right after receiving CTEInfo byte)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsCtepresent {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsCtepresent> for bool {
    #[inline(always)]
    fn from(variant: EventsCtepresent) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_CTEPRESENT` reader - CTE is present (early warning right after receiving CTEInfo byte)"]
pub type EventsCtepresentR = crate::BitReader<EventsCtepresent>;
impl EventsCtepresentR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsCtepresent {
        match self.bits {
            false => EventsCtepresent::NotGenerated,
            true => EventsCtepresent::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsCtepresent::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsCtepresent::Generated
    }
}
#[doc = "Field `EVENTS_CTEPRESENT` writer - CTE is present (early warning right after receiving CTEInfo byte)"]
pub type EventsCtepresentW<'a, REG> = crate::BitWriter<'a, REG, EventsCtepresent>;
impl<'a, REG> EventsCtepresentW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsCtepresent::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsCtepresent::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - CTE is present (early warning right after receiving CTEInfo byte)"]
    #[inline(always)]
    pub fn events_ctepresent(&self) -> EventsCtepresentR {
        EventsCtepresentR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CTE is present (early warning right after receiving CTEInfo byte)"]
    #[inline(always)]
    pub fn events_ctepresent(&mut self) -> EventsCtepresentW<'_, EventsCtepresentSpec> {
        EventsCtepresentW::new(self, 0)
    }
}
#[doc = "CTE is present (early warning right after receiving CTEInfo byte)\n\nYou can [`read`](crate::Reg::read) this register and get [`events_ctepresent::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_ctepresent::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsCtepresentSpec;
impl crate::RegisterSpec for EventsCtepresentSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_ctepresent::R`](R) reader structure"]
impl crate::Readable for EventsCtepresentSpec {}
#[doc = "`write(|w| ..)` method takes [`events_ctepresent::W`](W) writer structure"]
impl crate::Writable for EventsCtepresentSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_CTEPRESENT to value 0"]
impl crate::Resettable for EventsCtepresentSpec {}
