#[doc = "Register `EVENTS_TXFRAMEEND` reader"]
pub type R = crate::R<EventsTxframeendSpec>;
#[doc = "Register `EVENTS_TXFRAMEEND` writer"]
pub type W = crate::W<EventsTxframeendSpec>;
#[doc = "Marks the end of the last transmitted on-air symbol of a frame\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsTxframeend {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsTxframeend> for bool {
    #[inline(always)]
    fn from(variant: EventsTxframeend) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_TXFRAMEEND` reader - Marks the end of the last transmitted on-air symbol of a frame"]
pub type EventsTxframeendR = crate::BitReader<EventsTxframeend>;
impl EventsTxframeendR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsTxframeend {
        match self.bits {
            false => EventsTxframeend::NotGenerated,
            true => EventsTxframeend::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsTxframeend::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsTxframeend::Generated
    }
}
#[doc = "Field `EVENTS_TXFRAMEEND` writer - Marks the end of the last transmitted on-air symbol of a frame"]
pub type EventsTxframeendW<'a, REG> = crate::BitWriter<'a, REG, EventsTxframeend>;
impl<'a, REG> EventsTxframeendW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsTxframeend::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsTxframeend::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Marks the end of the last transmitted on-air symbol of a frame"]
    #[inline(always)]
    pub fn events_txframeend(&self) -> EventsTxframeendR {
        EventsTxframeendR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Marks the end of the last transmitted on-air symbol of a frame"]
    #[inline(always)]
    pub fn events_txframeend(&mut self) -> EventsTxframeendW<'_, EventsTxframeendSpec> {
        EventsTxframeendW::new(self, 0)
    }
}
#[doc = "Marks the end of the last transmitted on-air symbol of a frame\n\nYou can [`read`](crate::Reg::read) this register and get [`events_txframeend::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_txframeend::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsTxframeendSpec;
impl crate::RegisterSpec for EventsTxframeendSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_txframeend::R`](R) reader structure"]
impl crate::Readable for EventsTxframeendSpec {}
#[doc = "`write(|w| ..)` method takes [`events_txframeend::W`](W) writer structure"]
impl crate::Writable for EventsTxframeendSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_TXFRAMEEND to value 0"]
impl crate::Resettable for EventsTxframeendSpec {}
