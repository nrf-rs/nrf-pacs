#[doc = "Register `EVENTS_TXFRAMESTART` reader"]
pub type R = crate::R<EventsTxframestartSpec>;
#[doc = "Register `EVENTS_TXFRAMESTART` writer"]
pub type W = crate::W<EventsTxframestartSpec>;
#[doc = "Marks the start of the first symbol of a transmitted frame\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsTxframestart {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsTxframestart> for bool {
    #[inline(always)]
    fn from(variant: EventsTxframestart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_TXFRAMESTART` reader - Marks the start of the first symbol of a transmitted frame"]
pub type EventsTxframestartR = crate::BitReader<EventsTxframestart>;
impl EventsTxframestartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsTxframestart {
        match self.bits {
            false => EventsTxframestart::NotGenerated,
            true => EventsTxframestart::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsTxframestart::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsTxframestart::Generated
    }
}
#[doc = "Field `EVENTS_TXFRAMESTART` writer - Marks the start of the first symbol of a transmitted frame"]
pub type EventsTxframestartW<'a, REG> = crate::BitWriter<'a, REG, EventsTxframestart>;
impl<'a, REG> EventsTxframestartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsTxframestart::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsTxframestart::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Marks the start of the first symbol of a transmitted frame"]
    #[inline(always)]
    pub fn events_txframestart(&self) -> EventsTxframestartR {
        EventsTxframestartR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Marks the start of the first symbol of a transmitted frame"]
    #[inline(always)]
    pub fn events_txframestart(&mut self) -> EventsTxframestartW<'_, EventsTxframestartSpec> {
        EventsTxframestartW::new(self, 0)
    }
}
#[doc = "Marks the start of the first symbol of a transmitted frame\n\nYou can [`read`](crate::Reg::read) this register and get [`events_txframestart::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_txframestart::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsTxframestartSpec;
impl crate::RegisterSpec for EventsTxframestartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_txframestart::R`](R) reader structure"]
impl crate::Readable for EventsTxframestartSpec {}
#[doc = "`write(|w| ..)` method takes [`events_txframestart::W`](W) writer structure"]
impl crate::Writable for EventsTxframestartSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_TXFRAMESTART to value 0"]
impl crate::Resettable for EventsTxframestartSpec {}
