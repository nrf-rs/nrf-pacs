#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<IntenclrSpec>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
#[doc = "Write '1' to disable interrupt for event TIMEOUT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timeout {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Timeout> for bool {
    #[inline(always)]
    fn from(variant: Timeout) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMEOUT` reader - Write '1' to disable interrupt for event TIMEOUT"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Timeout::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Timeout::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event TIMEOUT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimeoutWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<TimeoutWO> for bool {
    #[inline(always)]
    fn from(variant: TimeoutWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMEOUT` writer - Write '1' to disable interrupt for event TIMEOUT"]
pub type TimeoutW<'a, REG> = crate::BitWriter<'a, REG, TimeoutWO>;
impl<'a, REG> TimeoutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(TimeoutWO::Clear)
    }
}
impl R {
    #[doc = "Bit 0 - Write '1' to disable interrupt for event TIMEOUT"]
    #[inline(always)]
    pub fn timeout(&self) -> TimeoutR {
        TimeoutR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to disable interrupt for event TIMEOUT"]
    #[inline(always)]
    #[must_use]
    pub fn timeout(&mut self) -> TimeoutW<IntenclrSpec> {
        TimeoutW::new(self, 0)
    }
}
#[doc = "Disable interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intenclr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intenclr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenclrSpec;
impl crate::RegisterSpec for IntenclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenclr::R`](R) reader structure"]
impl crate::Readable for IntenclrSpec {}
#[doc = "`write(|w| ..)` method takes [`intenclr::W`](W) writer structure"]
impl crate::Writable for IntenclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for IntenclrSpec {
    const RESET_VALUE: u32 = 0;
}
