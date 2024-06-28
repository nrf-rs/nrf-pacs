#[doc = "Register `EVENTS_KEYSLOT_PUSHED` reader"]
pub type R = crate::R<EventsKeyslotPushedSpec>;
#[doc = "Register `EVENTS_KEYSLOT_PUSHED` writer"]
pub type W = crate::W<EventsKeyslotPushedSpec>;
#[doc = "Key slot successfully pushed over secure APB\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsKeyslotPushed {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsKeyslotPushed> for bool {
    #[inline(always)]
    fn from(variant: EventsKeyslotPushed) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_KEYSLOT_PUSHED` reader - Key slot successfully pushed over secure APB"]
pub type EventsKeyslotPushedR = crate::BitReader<EventsKeyslotPushed>;
impl EventsKeyslotPushedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsKeyslotPushed {
        match self.bits {
            false => EventsKeyslotPushed::NotGenerated,
            true => EventsKeyslotPushed::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsKeyslotPushed::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsKeyslotPushed::Generated
    }
}
#[doc = "Field `EVENTS_KEYSLOT_PUSHED` writer - Key slot successfully pushed over secure APB"]
pub type EventsKeyslotPushedW<'a, REG> = crate::BitWriter<'a, REG, EventsKeyslotPushed>;
impl<'a, REG> EventsKeyslotPushedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsKeyslotPushed::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsKeyslotPushed::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Key slot successfully pushed over secure APB"]
    #[inline(always)]
    pub fn events_keyslot_pushed(&self) -> EventsKeyslotPushedR {
        EventsKeyslotPushedR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Key slot successfully pushed over secure APB"]
    #[inline(always)]
    #[must_use]
    pub fn events_keyslot_pushed(&mut self) -> EventsKeyslotPushedW<EventsKeyslotPushedSpec> {
        EventsKeyslotPushedW::new(self, 0)
    }
}
#[doc = "Key slot successfully pushed over secure APB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`events_keyslot_pushed::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`events_keyslot_pushed::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsKeyslotPushedSpec;
impl crate::RegisterSpec for EventsKeyslotPushedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_keyslot_pushed::R`](R) reader structure"]
impl crate::Readable for EventsKeyslotPushedSpec {}
#[doc = "`write(|w| ..)` method takes [`events_keyslot_pushed::W`](W) writer structure"]
impl crate::Writable for EventsKeyslotPushedSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVENTS_KEYSLOT_PUSHED to value 0"]
impl crate::Resettable for EventsKeyslotPushedSpec {
    const RESET_VALUE: u32 = 0;
}
