#[doc = "Register `EVENTS_CROSS` reader"]
pub type R = crate::R<EventsCrossSpec>;
#[doc = "Register `EVENTS_CROSS` writer"]
pub type W = crate::W<EventsCrossSpec>;
#[doc = "Downward or upward crossing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsCross {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsCross> for bool {
    #[inline(always)]
    fn from(variant: EventsCross) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_CROSS` reader - Downward or upward crossing"]
pub type EventsCrossR = crate::BitReader<EventsCross>;
impl EventsCrossR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsCross {
        match self.bits {
            false => EventsCross::NotGenerated,
            true => EventsCross::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsCross::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsCross::Generated
    }
}
#[doc = "Field `EVENTS_CROSS` writer - Downward or upward crossing"]
pub type EventsCrossW<'a, REG> = crate::BitWriter<'a, REG, EventsCross>;
impl<'a, REG> EventsCrossW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsCross::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsCross::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Downward or upward crossing"]
    #[inline(always)]
    pub fn events_cross(&self) -> EventsCrossR {
        EventsCrossR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Downward or upward crossing"]
    #[inline(always)]
    pub fn events_cross(&mut self) -> EventsCrossW<'_, EventsCrossSpec> {
        EventsCrossW::new(self, 0)
    }
}
#[doc = "Downward or upward crossing\n\nYou can [`read`](crate::Reg::read) this register and get [`events_cross::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_cross::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsCrossSpec;
impl crate::RegisterSpec for EventsCrossSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_cross::R`](R) reader structure"]
impl crate::Readable for EventsCrossSpec {}
#[doc = "`write(|w| ..)` method takes [`events_cross::W`](W) writer structure"]
impl crate::Writable for EventsCrossSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_CROSS to value 0"]
impl crate::Resettable for EventsCrossSpec {}
