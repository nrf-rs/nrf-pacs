#[doc = "Register `EVENTS_PWMPERIODEND` reader"]
pub type R = crate::R<EventsPwmperiodendSpec>;
#[doc = "Register `EVENTS_PWMPERIODEND` writer"]
pub type W = crate::W<EventsPwmperiodendSpec>;
#[doc = "Emitted at the end of each PWM period\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsPwmperiodend {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsPwmperiodend> for bool {
    #[inline(always)]
    fn from(variant: EventsPwmperiodend) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_PWMPERIODEND` reader - Emitted at the end of each PWM period"]
pub type EventsPwmperiodendR = crate::BitReader<EventsPwmperiodend>;
impl EventsPwmperiodendR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsPwmperiodend {
        match self.bits {
            false => EventsPwmperiodend::NotGenerated,
            true => EventsPwmperiodend::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsPwmperiodend::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsPwmperiodend::Generated
    }
}
#[doc = "Field `EVENTS_PWMPERIODEND` writer - Emitted at the end of each PWM period"]
pub type EventsPwmperiodendW<'a, REG> = crate::BitWriter<'a, REG, EventsPwmperiodend>;
impl<'a, REG> EventsPwmperiodendW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsPwmperiodend::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsPwmperiodend::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Emitted at the end of each PWM period"]
    #[inline(always)]
    pub fn events_pwmperiodend(&self) -> EventsPwmperiodendR {
        EventsPwmperiodendR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Emitted at the end of each PWM period"]
    #[inline(always)]
    pub fn events_pwmperiodend(&mut self) -> EventsPwmperiodendW<'_, EventsPwmperiodendSpec> {
        EventsPwmperiodendW::new(self, 0)
    }
}
#[doc = "Emitted at the end of each PWM period\n\nYou can [`read`](crate::Reg::read) this register and get [`events_pwmperiodend::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_pwmperiodend::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsPwmperiodendSpec;
impl crate::RegisterSpec for EventsPwmperiodendSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_pwmperiodend::R`](R) reader structure"]
impl crate::Readable for EventsPwmperiodendSpec {}
#[doc = "`write(|w| ..)` method takes [`events_pwmperiodend::W`](W) writer structure"]
impl crate::Writable for EventsPwmperiodendSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_PWMPERIODEND to value 0"]
impl crate::Resettable for EventsPwmperiodendSpec {}
