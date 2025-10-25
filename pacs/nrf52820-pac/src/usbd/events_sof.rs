#[doc = "Register `EVENTS_SOF` reader"]
pub type R = crate::R<EventsSofSpec>;
#[doc = "Register `EVENTS_SOF` writer"]
pub type W = crate::W<EventsSofSpec>;
#[doc = "Signals that a SOF (start of frame) condition has been detected on USB lines\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsSof {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsSof> for bool {
    #[inline(always)]
    fn from(variant: EventsSof) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_SOF` reader - Signals that a SOF (start of frame) condition has been detected on USB lines"]
pub type EventsSofR = crate::BitReader<EventsSof>;
impl EventsSofR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsSof {
        match self.bits {
            false => EventsSof::NotGenerated,
            true => EventsSof::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsSof::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsSof::Generated
    }
}
#[doc = "Field `EVENTS_SOF` writer - Signals that a SOF (start of frame) condition has been detected on USB lines"]
pub type EventsSofW<'a, REG> = crate::BitWriter<'a, REG, EventsSof>;
impl<'a, REG> EventsSofW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsSof::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsSof::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Signals that a SOF (start of frame) condition has been detected on USB lines"]
    #[inline(always)]
    pub fn events_sof(&self) -> EventsSofR {
        EventsSofR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Signals that a SOF (start of frame) condition has been detected on USB lines"]
    #[inline(always)]
    pub fn events_sof(&mut self) -> EventsSofW<'_, EventsSofSpec> {
        EventsSofW::new(self, 0)
    }
}
#[doc = "Signals that a SOF (start of frame) condition has been detected on USB lines\n\nYou can [`read`](crate::Reg::read) this register and get [`events_sof::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_sof::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsSofSpec;
impl crate::RegisterSpec for EventsSofSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_sof::R`](R) reader structure"]
impl crate::Readable for EventsSofSpec {}
#[doc = "`write(|w| ..)` method takes [`events_sof::W`](W) writer structure"]
impl crate::Writable for EventsSofSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_SOF to value 0"]
impl crate::Resettable for EventsSofSpec {}
