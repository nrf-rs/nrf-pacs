#[doc = "Register `EVENTS_MHRMATCH` reader"]
pub type R = crate::R<EventsMhrmatchSpec>;
#[doc = "Register `EVENTS_MHRMATCH` writer"]
pub type W = crate::W<EventsMhrmatchSpec>;
#[doc = "MAC header match found\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsMhrmatch {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsMhrmatch> for bool {
    #[inline(always)]
    fn from(variant: EventsMhrmatch) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_MHRMATCH` reader - MAC header match found"]
pub type EventsMhrmatchR = crate::BitReader<EventsMhrmatch>;
impl EventsMhrmatchR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsMhrmatch {
        match self.bits {
            false => EventsMhrmatch::NotGenerated,
            true => EventsMhrmatch::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsMhrmatch::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsMhrmatch::Generated
    }
}
#[doc = "Field `EVENTS_MHRMATCH` writer - MAC header match found"]
pub type EventsMhrmatchW<'a, REG> = crate::BitWriter<'a, REG, EventsMhrmatch>;
impl<'a, REG> EventsMhrmatchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsMhrmatch::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsMhrmatch::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - MAC header match found"]
    #[inline(always)]
    pub fn events_mhrmatch(&self) -> EventsMhrmatchR {
        EventsMhrmatchR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MAC header match found"]
    #[inline(always)]
    pub fn events_mhrmatch(&mut self) -> EventsMhrmatchW<'_, EventsMhrmatchSpec> {
        EventsMhrmatchW::new(self, 0)
    }
}
#[doc = "MAC header match found\n\nYou can [`read`](crate::Reg::read) this register and get [`events_mhrmatch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_mhrmatch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsMhrmatchSpec;
impl crate::RegisterSpec for EventsMhrmatchSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_mhrmatch::R`](R) reader structure"]
impl crate::Readable for EventsMhrmatchSpec {}
#[doc = "`write(|w| ..)` method takes [`events_mhrmatch::W`](W) writer structure"]
impl crate::Writable for EventsMhrmatchSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_MHRMATCH to value 0"]
impl crate::Resettable for EventsMhrmatchSpec {}
