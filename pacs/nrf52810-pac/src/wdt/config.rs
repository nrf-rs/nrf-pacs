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
#[doc = "Configure the watchdog to either be paused, or kept running, while the CPU is sleeping\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEEP_A {
    #[doc = "0: Pause watchdog while the CPU is sleeping"]
    PAUSE = 0,
    #[doc = "1: Keep the watchdog running while the CPU is sleeping"]
    RUN = 1,
}
impl From<SLEEP_A> for bool {
    #[inline(always)]
    fn from(variant: SLEEP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLEEP` reader - Configure the watchdog to either be paused, or kept running, while the CPU is sleeping"]
pub struct SLEEP_R(crate::FieldReader<bool, SLEEP_A>);
impl SLEEP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLEEP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == SLEEP_A::PAUSE
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        **self == SLEEP_A::RUN
    }
}
impl core::ops::Deref for SLEEP_R {
    type Target = crate::FieldReader<bool, SLEEP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLEEP` writer - Configure the watchdog to either be paused, or kept running, while the CPU is sleeping"]
pub struct SLEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEEP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLEEP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pause watchdog while the CPU is sleeping"]
    #[inline(always)]
    pub fn pause(self) -> &'a mut W {
        self.variant(SLEEP_A::PAUSE)
    }
    #[doc = "Keep the watchdog running while the CPU is sleeping"]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(SLEEP_A::RUN)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Configure the watchdog to either be paused, or kept running, while the CPU is halted by the debugger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HALT_A {
    #[doc = "0: Pause watchdog while the CPU is halted by the debugger"]
    PAUSE = 0,
    #[doc = "1: Keep the watchdog running while the CPU is halted by the debugger"]
    RUN = 1,
}
impl From<HALT_A> for bool {
    #[inline(always)]
    fn from(variant: HALT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HALT` reader - Configure the watchdog to either be paused, or kept running, while the CPU is halted by the debugger"]
pub struct HALT_R(crate::FieldReader<bool, HALT_A>);
impl HALT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HALT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == HALT_A::PAUSE
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        **self == HALT_A::RUN
    }
}
impl core::ops::Deref for HALT_R {
    type Target = crate::FieldReader<bool, HALT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HALT` writer - Configure the watchdog to either be paused, or kept running, while the CPU is halted by the debugger"]
pub struct HALT_W<'a> {
    w: &'a mut W,
}
impl<'a> HALT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HALT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pause watchdog while the CPU is halted by the debugger"]
    #[inline(always)]
    pub fn pause(self) -> &'a mut W {
        self.variant(HALT_A::PAUSE)
    }
    #[doc = "Keep the watchdog running while the CPU is halted by the debugger"]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(HALT_A::RUN)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Configure the watchdog to either be paused, or kept running, while the CPU is sleeping"]
    #[inline(always)]
    pub fn sleep(&self) -> SLEEP_R {
        SLEEP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 3 - Configure the watchdog to either be paused, or kept running, while the CPU is halted by the debugger"]
    #[inline(always)]
    pub fn halt(&self) -> HALT_R {
        HALT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Configure the watchdog to either be paused, or kept running, while the CPU is sleeping"]
    #[inline(always)]
    pub fn sleep(&mut self) -> SLEEP_W {
        SLEEP_W { w: self }
    }
    #[doc = "Bit 3 - Configure the watchdog to either be paused, or kept running, while the CPU is halted by the debugger"]
    #[inline(always)]
    pub fn halt(&mut self) -> HALT_W {
        HALT_W { w: self }
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
