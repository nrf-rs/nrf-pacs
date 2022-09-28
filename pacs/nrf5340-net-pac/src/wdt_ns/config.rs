#[doc = "Register `CONFIG` reader"]
pub struct R(crate::R<CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONFIG` writer"]
pub struct W(crate::W<CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLEEP` reader - Configure WDT to either be paused, or kept running, while the CPU is sleeping"]
pub type SLEEP_R = crate::BitReader<SLEEP_A>;
#[doc = "Configure WDT to either be paused, or kept running, while the CPU is sleeping\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEEP_A {
    #[doc = "0: Pause WDT while the CPU is sleeping"]
    PAUSE = 0,
    #[doc = "1: Keep WDT running while the CPU is sleeping"]
    RUN = 1,
}
impl From<SLEEP_A> for bool {
    #[inline(always)]
    fn from(variant: SLEEP_A) -> Self {
        variant as u8 != 0
    }
}
impl SLEEP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLEEP_A {
        match self.bits {
            false => SLEEP_A::PAUSE,
            true => SLEEP_A::RUN,
        }
    }
    #[doc = "Checks if the value of the field is `PAUSE`"]
    #[inline(always)]
    pub fn is_pause(&self) -> bool {
        *self == SLEEP_A::PAUSE
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == SLEEP_A::RUN
    }
}
#[doc = "Field `SLEEP` writer - Configure WDT to either be paused, or kept running, while the CPU is sleeping"]
pub type SLEEP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, SLEEP_A, O>;
impl<'a, const O: u8> SLEEP_W<'a, O> {
    #[doc = "Pause WDT while the CPU is sleeping"]
    #[inline(always)]
    pub fn pause(self) -> &'a mut W {
        self.variant(SLEEP_A::PAUSE)
    }
    #[doc = "Keep WDT running while the CPU is sleeping"]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(SLEEP_A::RUN)
    }
}
#[doc = "Field `HALT` reader - Configure WDT to either be paused, or kept running, while the CPU is halted by the debugger"]
pub type HALT_R = crate::BitReader<HALT_A>;
#[doc = "Configure WDT to either be paused, or kept running, while the CPU is halted by the debugger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HALT_A {
    #[doc = "0: Pause WDT while the CPU is halted by the debugger"]
    PAUSE = 0,
    #[doc = "1: Keep WDT running while the CPU is halted by the debugger"]
    RUN = 1,
}
impl From<HALT_A> for bool {
    #[inline(always)]
    fn from(variant: HALT_A) -> Self {
        variant as u8 != 0
    }
}
impl HALT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HALT_A {
        match self.bits {
            false => HALT_A::PAUSE,
            true => HALT_A::RUN,
        }
    }
    #[doc = "Checks if the value of the field is `PAUSE`"]
    #[inline(always)]
    pub fn is_pause(&self) -> bool {
        *self == HALT_A::PAUSE
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == HALT_A::RUN
    }
}
#[doc = "Field `HALT` writer - Configure WDT to either be paused, or kept running, while the CPU is halted by the debugger"]
pub type HALT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, HALT_A, O>;
impl<'a, const O: u8> HALT_W<'a, O> {
    #[doc = "Pause WDT while the CPU is halted by the debugger"]
    #[inline(always)]
    pub fn pause(self) -> &'a mut W {
        self.variant(HALT_A::PAUSE)
    }
    #[doc = "Keep WDT running while the CPU is halted by the debugger"]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(HALT_A::RUN)
    }
}
#[doc = "Field `STOPEN` reader - Allow stopping WDT"]
pub type STOPEN_R = crate::BitReader<STOPEN_A>;
#[doc = "Allow stopping WDT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPEN_A {
    #[doc = "0: Do not allow stopping WDT"]
    DISABLE = 0,
    #[doc = "1: Allow stopping WDT"]
    ENABLE = 1,
}
impl From<STOPEN_A> for bool {
    #[inline(always)]
    fn from(variant: STOPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl STOPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOPEN_A {
        match self.bits {
            false => STOPEN_A::DISABLE,
            true => STOPEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == STOPEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == STOPEN_A::ENABLE
    }
}
#[doc = "Field `STOPEN` writer - Allow stopping WDT"]
pub type STOPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, STOPEN_A, O>;
impl<'a, const O: u8> STOPEN_W<'a, O> {
    #[doc = "Do not allow stopping WDT"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(STOPEN_A::DISABLE)
    }
    #[doc = "Allow stopping WDT"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(STOPEN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - Configure WDT to either be paused, or kept running, while the CPU is sleeping"]
    #[inline(always)]
    pub fn sleep(&self) -> SLEEP_R {
        SLEEP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - Configure WDT to either be paused, or kept running, while the CPU is halted by the debugger"]
    #[inline(always)]
    pub fn halt(&self) -> HALT_R {
        HALT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - Allow stopping WDT"]
    #[inline(always)]
    pub fn stopen(&self) -> STOPEN_R {
        STOPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Configure WDT to either be paused, or kept running, while the CPU is sleeping"]
    #[inline(always)]
    pub fn sleep(&mut self) -> SLEEP_W<0> {
        SLEEP_W::new(self)
    }
    #[doc = "Bit 3 - Configure WDT to either be paused, or kept running, while the CPU is halted by the debugger"]
    #[inline(always)]
    pub fn halt(&mut self) -> HALT_W<3> {
        HALT_W::new(self)
    }
    #[doc = "Bit 6 - Allow stopping WDT"]
    #[inline(always)]
    pub fn stopen(&mut self) -> STOPEN_W<6> {
        STOPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config](index.html) module"]
pub struct CONFIG_SPEC;
impl crate::RegisterSpec for CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [config::R](R) reader structure"]
impl crate::Readable for CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [config::W](W) writer structure"]
impl crate::Writable for CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONFIG to value 0x01"]
impl crate::Resettable for CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
