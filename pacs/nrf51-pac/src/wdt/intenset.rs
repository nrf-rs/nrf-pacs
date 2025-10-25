#[doc = "Register `INTENSET` reader"]
pub type R = crate::R<IntensetSpec>;
#[doc = "Register `INTENSET` writer"]
pub type W = crate::W<IntensetSpec>;
#[doc = "Enable interrupt on TIMEOUT event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timeout {
    #[doc = "0: Interrupt disabled."]
    Disabled = 0,
    #[doc = "1: Interrupt enabled."]
    Enabled = 1,
}
impl From<Timeout> for bool {
    #[inline(always)]
    fn from(variant: Timeout) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMEOUT` reader - Enable interrupt on TIMEOUT event."]
pub type TimeoutR = crate::BitReader<Timeout>;
impl TimeoutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timeout {
        match self.bits {
            false => Timeout::Disabled,
            true => Timeout::Enabled,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Timeout::Disabled
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Timeout::Enabled
    }
}
#[doc = "Enable interrupt on TIMEOUT event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimeoutWO {
    #[doc = "1: Enable interrupt on write."]
    Set = 1,
}
impl From<TimeoutWO> for bool {
    #[inline(always)]
    fn from(variant: TimeoutWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMEOUT` writer - Enable interrupt on TIMEOUT event."]
pub type TimeoutW<'a, REG> = crate::BitWriter<'a, REG, TimeoutWO>;
impl<'a, REG> TimeoutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt on write."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(TimeoutWO::Set)
    }
}
impl R {
    #[doc = "Bit 0 - Enable interrupt on TIMEOUT event."]
    #[inline(always)]
    pub fn timeout(&self) -> TimeoutR {
        TimeoutR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable interrupt on TIMEOUT event."]
    #[inline(always)]
    pub fn timeout(&mut self) -> TimeoutW<'_, IntensetSpec> {
        TimeoutW::new(self, 0)
    }
}
#[doc = "Interrupt enable set register.\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntensetSpec;
impl crate::RegisterSpec for IntensetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenset::R`](R) reader structure"]
impl crate::Readable for IntensetSpec {}
#[doc = "`write(|w| ..)` method takes [`intenset::W`](W) writer structure"]
impl crate::Writable for IntensetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for IntensetSpec {}
