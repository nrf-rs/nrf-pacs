#[doc = "Register `EVENTS_CCABUSY` reader"]
pub type R = crate::R<EventsCcabusySpec>;
#[doc = "Register `EVENTS_CCABUSY` writer"]
pub type W = crate::W<EventsCcabusySpec>;
#[doc = "Wireless medium busy - do not send\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsCcabusy {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsCcabusy> for bool {
    #[inline(always)]
    fn from(variant: EventsCcabusy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_CCABUSY` reader - Wireless medium busy - do not send"]
pub type EventsCcabusyR = crate::BitReader<EventsCcabusy>;
impl EventsCcabusyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsCcabusy {
        match self.bits {
            false => EventsCcabusy::NotGenerated,
            true => EventsCcabusy::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsCcabusy::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsCcabusy::Generated
    }
}
#[doc = "Field `EVENTS_CCABUSY` writer - Wireless medium busy - do not send"]
pub type EventsCcabusyW<'a, REG> = crate::BitWriter<'a, REG, EventsCcabusy>;
impl<'a, REG> EventsCcabusyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsCcabusy::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsCcabusy::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Wireless medium busy - do not send"]
    #[inline(always)]
    pub fn events_ccabusy(&self) -> EventsCcabusyR {
        EventsCcabusyR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wireless medium busy - do not send"]
    #[inline(always)]
    pub fn events_ccabusy(&mut self) -> EventsCcabusyW<'_, EventsCcabusySpec> {
        EventsCcabusyW::new(self, 0)
    }
}
#[doc = "Wireless medium busy - do not send\n\nYou can [`read`](crate::Reg::read) this register and get [`events_ccabusy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_ccabusy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsCcabusySpec;
impl crate::RegisterSpec for EventsCcabusySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_ccabusy::R`](R) reader structure"]
impl crate::Readable for EventsCcabusySpec {}
#[doc = "`write(|w| ..)` method takes [`events_ccabusy::W`](W) writer structure"]
impl crate::Writable for EventsCcabusySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_CCABUSY to value 0"]
impl crate::Resettable for EventsCcabusySpec {}
