#[doc = "Register `EVENTS_DBLRDY` reader"]
pub type R = crate::R<EventsDblrdySpec>;
#[doc = "Register `EVENTS_DBLRDY` writer"]
pub type W = crate::W<EventsDblrdySpec>;
#[doc = "Double displacement(s) detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsDblrdy {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsDblrdy> for bool {
    #[inline(always)]
    fn from(variant: EventsDblrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_DBLRDY` reader - Double displacement(s) detected"]
pub type EventsDblrdyR = crate::BitReader<EventsDblrdy>;
impl EventsDblrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsDblrdy {
        match self.bits {
            false => EventsDblrdy::NotGenerated,
            true => EventsDblrdy::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsDblrdy::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsDblrdy::Generated
    }
}
#[doc = "Field `EVENTS_DBLRDY` writer - Double displacement(s) detected"]
pub type EventsDblrdyW<'a, REG> = crate::BitWriter<'a, REG, EventsDblrdy>;
impl<'a, REG> EventsDblrdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsDblrdy::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsDblrdy::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Double displacement(s) detected"]
    #[inline(always)]
    pub fn events_dblrdy(&self) -> EventsDblrdyR {
        EventsDblrdyR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Double displacement(s) detected"]
    #[inline(always)]
    pub fn events_dblrdy(&mut self) -> EventsDblrdyW<'_, EventsDblrdySpec> {
        EventsDblrdyW::new(self, 0)
    }
}
#[doc = "Double displacement(s) detected\n\nYou can [`read`](crate::Reg::read) this register and get [`events_dblrdy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_dblrdy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsDblrdySpec;
impl crate::RegisterSpec for EventsDblrdySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_dblrdy::R`](R) reader structure"]
impl crate::Readable for EventsDblrdySpec {}
#[doc = "`write(|w| ..)` method takes [`events_dblrdy::W`](W) writer structure"]
impl crate::Writable for EventsDblrdySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_DBLRDY to value 0"]
impl crate::Resettable for EventsDblrdySpec {}
