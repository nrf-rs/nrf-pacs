#[doc = "Register `MODE` reader"]
pub struct R(crate::R<MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MODE` writer"]
pub struct W(crate::W<MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MODE_SPEC>;
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
impl From<crate::W<MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Radio data rate and modulation setting. Decision point: TXEN or RXEN task.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: 1Mbit/s Nordic propietary radio mode."]
    NRF_1MBIT = 0,
    #[doc = "1: 2Mbit/s Nordic propietary radio mode."]
    NRF_2MBIT = 1,
    #[doc = "2: 250kbit/s Nordic propietary radio mode."]
    NRF_250KBIT = 2,
    #[doc = "3: 1Mbit/s Bluetooth Low Energy"]
    BLE_1MBIT = 3,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MODE` reader - Radio data rate and modulation setting. Decision point: TXEN or RXEN task."]
pub struct MODE_R(crate::FieldReader<u8, MODE_A>);
impl MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            0 => MODE_A::NRF_1MBIT,
            1 => MODE_A::NRF_2MBIT,
            2 => MODE_A::NRF_250KBIT,
            3 => MODE_A::BLE_1MBIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NRF_1MBIT`"]
    #[inline(always)]
    pub fn is_nrf_1mbit(&self) -> bool {
        **self == MODE_A::NRF_1MBIT
    }
    #[doc = "Checks if the value of the field is `NRF_2MBIT`"]
    #[inline(always)]
    pub fn is_nrf_2mbit(&self) -> bool {
        **self == MODE_A::NRF_2MBIT
    }
    #[doc = "Checks if the value of the field is `NRF_250KBIT`"]
    #[inline(always)]
    pub fn is_nrf_250kbit(&self) -> bool {
        **self == MODE_A::NRF_250KBIT
    }
    #[doc = "Checks if the value of the field is `BLE_1MBIT`"]
    #[inline(always)]
    pub fn is_ble_1mbit(&self) -> bool {
        **self == MODE_A::BLE_1MBIT
    }
}
impl core::ops::Deref for MODE_R {
    type Target = crate::FieldReader<u8, MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE` writer - Radio data rate and modulation setting. Decision point: TXEN or RXEN task."]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "1Mbit/s Nordic propietary radio mode."]
    #[inline(always)]
    pub fn nrf_1mbit(self) -> &'a mut W {
        self.variant(MODE_A::NRF_1MBIT)
    }
    #[doc = "2Mbit/s Nordic propietary radio mode."]
    #[inline(always)]
    pub fn nrf_2mbit(self) -> &'a mut W {
        self.variant(MODE_A::NRF_2MBIT)
    }
    #[doc = "250kbit/s Nordic propietary radio mode."]
    #[inline(always)]
    pub fn nrf_250kbit(self) -> &'a mut W {
        self.variant(MODE_A::NRF_250KBIT)
    }
    #[doc = "1Mbit/s Bluetooth Low Energy"]
    #[inline(always)]
    pub fn ble_1mbit(self) -> &'a mut W {
        self.variant(MODE_A::BLE_1MBIT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Radio data rate and modulation setting. Decision point: TXEN or RXEN task."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Radio data rate and modulation setting. Decision point: TXEN or RXEN task."]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data rate and modulation.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mode](index.html) module"]
pub struct MODE_SPEC;
impl crate::RegisterSpec for MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mode::R](R) reader structure"]
impl crate::Readable for MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mode::W](W) writer structure"]
impl crate::Writable for MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MODE to value 0"]
impl crate::Resettable for MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
