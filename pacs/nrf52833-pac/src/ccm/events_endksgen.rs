#[doc = "Register `EVENTS_ENDKSGEN` reader"]
pub type R = crate::R<EventsEndksgenSpec>;
#[doc = "Register `EVENTS_ENDKSGEN` writer"]
pub type W = crate::W<EventsEndksgenSpec>;
#[doc = "Keystream generation complete\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsEndksgen {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsEndksgen> for bool {
    #[inline(always)]
    fn from(variant: EventsEndksgen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_ENDKSGEN` reader - Keystream generation complete"]
pub type EventsEndksgenR = crate::BitReader<EventsEndksgen>;
impl EventsEndksgenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsEndksgen {
        match self.bits {
            false => EventsEndksgen::NotGenerated,
            true => EventsEndksgen::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsEndksgen::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsEndksgen::Generated
    }
}
#[doc = "Field `EVENTS_ENDKSGEN` writer - Keystream generation complete"]
pub type EventsEndksgenW<'a, REG> = crate::BitWriter<'a, REG, EventsEndksgen>;
impl<'a, REG> EventsEndksgenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsEndksgen::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsEndksgen::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Keystream generation complete"]
    #[inline(always)]
    pub fn events_endksgen(&self) -> EventsEndksgenR {
        EventsEndksgenR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Keystream generation complete"]
    #[inline(always)]
    pub fn events_endksgen(&mut self) -> EventsEndksgenW<'_, EventsEndksgenSpec> {
        EventsEndksgenW::new(self, 0)
    }
}
#[doc = "Keystream generation complete\n\nYou can [`read`](crate::Reg::read) this register and get [`events_endksgen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_endksgen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsEndksgenSpec;
impl crate::RegisterSpec for EventsEndksgenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_endksgen::R`](R) reader structure"]
impl crate::Readable for EventsEndksgenSpec {}
#[doc = "`write(|w| ..)` method takes [`events_endksgen::W`](W) writer structure"]
impl crate::Writable for EventsEndksgenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_ENDKSGEN to value 0"]
impl crate::Resettable for EventsEndksgenSpec {}
