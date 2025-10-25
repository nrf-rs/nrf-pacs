#[doc = "Register `EVENTS_ENDTX` reader"]
pub type R = crate::R<EventsEndtxSpec>;
#[doc = "Register `EVENTS_ENDTX` writer"]
pub type W = crate::W<EventsEndtxSpec>;
#[doc = "End of TXD buffer reached\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsEndtx {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsEndtx> for bool {
    #[inline(always)]
    fn from(variant: EventsEndtx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_ENDTX` reader - End of TXD buffer reached"]
pub type EventsEndtxR = crate::BitReader<EventsEndtx>;
impl EventsEndtxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsEndtx {
        match self.bits {
            false => EventsEndtx::NotGenerated,
            true => EventsEndtx::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsEndtx::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsEndtx::Generated
    }
}
#[doc = "Field `EVENTS_ENDTX` writer - End of TXD buffer reached"]
pub type EventsEndtxW<'a, REG> = crate::BitWriter<'a, REG, EventsEndtx>;
impl<'a, REG> EventsEndtxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsEndtx::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsEndtx::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - End of TXD buffer reached"]
    #[inline(always)]
    pub fn events_endtx(&self) -> EventsEndtxR {
        EventsEndtxR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - End of TXD buffer reached"]
    #[inline(always)]
    pub fn events_endtx(&mut self) -> EventsEndtxW<'_, EventsEndtxSpec> {
        EventsEndtxW::new(self, 0)
    }
}
#[doc = "End of TXD buffer reached\n\nYou can [`read`](crate::Reg::read) this register and get [`events_endtx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_endtx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsEndtxSpec;
impl crate::RegisterSpec for EventsEndtxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_endtx::R`](R) reader structure"]
impl crate::Readable for EventsEndtxSpec {}
#[doc = "`write(|w| ..)` method takes [`events_endtx::W`](W) writer structure"]
impl crate::Writable for EventsEndtxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_ENDTX to value 0"]
impl crate::Resettable for EventsEndtxSpec {}
