#[doc = "Register `TRACECONFIG` reader"]
pub struct R(crate::R<TRACECONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRACECONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRACECONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRACECONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRACECONFIG` writer"]
pub struct W(crate::W<TRACECONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRACECONFIG_SPEC>;
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
impl From<crate::W<TRACECONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRACECONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Speed of trace port clock. Note that the TRACECLK pin will output this clock divided by two.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRACEPORTSPEED_A {
    #[doc = "0: 32 MHz trace port clock (TRACECLK = 16 MHz)"]
    _32MHZ = 0,
    #[doc = "1: 16 MHz trace port clock (TRACECLK = 8 MHz)"]
    _16MHZ = 1,
    #[doc = "2: 8 MHz trace port clock (TRACECLK = 4 MHz)"]
    _8MHZ = 2,
    #[doc = "3: 4 MHz trace port clock (TRACECLK = 2 MHz)"]
    _4MHZ = 3,
}
impl From<TRACEPORTSPEED_A> for u8 {
    #[inline(always)]
    fn from(variant: TRACEPORTSPEED_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TRACEPORTSPEED` reader - Speed of trace port clock. Note that the TRACECLK pin will output this clock divided by two."]
pub struct TRACEPORTSPEED_R(crate::FieldReader<u8, TRACEPORTSPEED_A>);
impl TRACEPORTSPEED_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TRACEPORTSPEED_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRACEPORTSPEED_A {
        match self.bits {
            0 => TRACEPORTSPEED_A::_32MHZ,
            1 => TRACEPORTSPEED_A::_16MHZ,
            2 => TRACEPORTSPEED_A::_8MHZ,
            3 => TRACEPORTSPEED_A::_4MHZ,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_32MHZ`"]
    #[inline(always)]
    pub fn is_32mhz(&self) -> bool {
        **self == TRACEPORTSPEED_A::_32MHZ
    }
    #[doc = "Checks if the value of the field is `_16MHZ`"]
    #[inline(always)]
    pub fn is_16mhz(&self) -> bool {
        **self == TRACEPORTSPEED_A::_16MHZ
    }
    #[doc = "Checks if the value of the field is `_8MHZ`"]
    #[inline(always)]
    pub fn is_8mhz(&self) -> bool {
        **self == TRACEPORTSPEED_A::_8MHZ
    }
    #[doc = "Checks if the value of the field is `_4MHZ`"]
    #[inline(always)]
    pub fn is_4mhz(&self) -> bool {
        **self == TRACEPORTSPEED_A::_4MHZ
    }
}
impl core::ops::Deref for TRACEPORTSPEED_R {
    type Target = crate::FieldReader<u8, TRACEPORTSPEED_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRACEPORTSPEED` writer - Speed of trace port clock. Note that the TRACECLK pin will output this clock divided by two."]
pub struct TRACEPORTSPEED_W<'a> {
    w: &'a mut W,
}
impl<'a> TRACEPORTSPEED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRACEPORTSPEED_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "32 MHz trace port clock (TRACECLK = 16 MHz)"]
    #[inline(always)]
    pub fn _32mhz(self) -> &'a mut W {
        self.variant(TRACEPORTSPEED_A::_32MHZ)
    }
    #[doc = "16 MHz trace port clock (TRACECLK = 8 MHz)"]
    #[inline(always)]
    pub fn _16mhz(self) -> &'a mut W {
        self.variant(TRACEPORTSPEED_A::_16MHZ)
    }
    #[doc = "8 MHz trace port clock (TRACECLK = 4 MHz)"]
    #[inline(always)]
    pub fn _8mhz(self) -> &'a mut W {
        self.variant(TRACEPORTSPEED_A::_8MHZ)
    }
    #[doc = "4 MHz trace port clock (TRACECLK = 2 MHz)"]
    #[inline(always)]
    pub fn _4mhz(self) -> &'a mut W {
        self.variant(TRACEPORTSPEED_A::_4MHZ)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Pin multiplexing of trace signals. See pin assignment chapter for more details.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRACEMUX_A {
    #[doc = "0: No trace signals routed to pins. All pins can be used as regular GPIOs."]
    GPIO = 0,
    #[doc = "1: SWO trace signal routed to pin. Remaining pins can be used as regular GPIOs."]
    SERIAL = 1,
    #[doc = "2: All trace signals (TRACECLK and TRACEDATA\\[n\\]) routed to pins."]
    PARALLEL = 2,
}
impl From<TRACEMUX_A> for u8 {
    #[inline(always)]
    fn from(variant: TRACEMUX_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TRACEMUX` reader - Pin multiplexing of trace signals. See pin assignment chapter for more details."]
pub struct TRACEMUX_R(crate::FieldReader<u8, TRACEMUX_A>);
impl TRACEMUX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TRACEMUX_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TRACEMUX_A> {
        match self.bits {
            0 => Some(TRACEMUX_A::GPIO),
            1 => Some(TRACEMUX_A::SERIAL),
            2 => Some(TRACEMUX_A::PARALLEL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `GPIO`"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        **self == TRACEMUX_A::GPIO
    }
    #[doc = "Checks if the value of the field is `SERIAL`"]
    #[inline(always)]
    pub fn is_serial(&self) -> bool {
        **self == TRACEMUX_A::SERIAL
    }
    #[doc = "Checks if the value of the field is `PARALLEL`"]
    #[inline(always)]
    pub fn is_parallel(&self) -> bool {
        **self == TRACEMUX_A::PARALLEL
    }
}
impl core::ops::Deref for TRACEMUX_R {
    type Target = crate::FieldReader<u8, TRACEMUX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRACEMUX` writer - Pin multiplexing of trace signals. See pin assignment chapter for more details."]
pub struct TRACEMUX_W<'a> {
    w: &'a mut W,
}
impl<'a> TRACEMUX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRACEMUX_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No trace signals routed to pins. All pins can be used as regular GPIOs."]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut W {
        self.variant(TRACEMUX_A::GPIO)
    }
    #[doc = "SWO trace signal routed to pin. Remaining pins can be used as regular GPIOs."]
    #[inline(always)]
    pub fn serial(self) -> &'a mut W {
        self.variant(TRACEMUX_A::SERIAL)
    }
    #[doc = "All trace signals (TRACECLK and TRACEDATA\\[n\\]) routed to pins."]
    #[inline(always)]
    pub fn parallel(self) -> &'a mut W {
        self.variant(TRACEMUX_A::PARALLEL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Speed of trace port clock. Note that the TRACECLK pin will output this clock divided by two."]
    #[inline(always)]
    pub fn traceportspeed(&self) -> TRACEPORTSPEED_R {
        TRACEPORTSPEED_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Pin multiplexing of trace signals. See pin assignment chapter for more details."]
    #[inline(always)]
    pub fn tracemux(&self) -> TRACEMUX_R {
        TRACEMUX_R::new(((self.bits >> 16) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Speed of trace port clock. Note that the TRACECLK pin will output this clock divided by two."]
    #[inline(always)]
    pub fn traceportspeed(&mut self) -> TRACEPORTSPEED_W {
        TRACEPORTSPEED_W { w: self }
    }
    #[doc = "Bits 16:17 - Pin multiplexing of trace signals. See pin assignment chapter for more details."]
    #[inline(always)]
    pub fn tracemux(&mut self) -> TRACEMUX_W {
        TRACEMUX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clocking options for the trace port debug interface\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [traceconfig](index.html) module"]
pub struct TRACECONFIG_SPEC;
impl crate::RegisterSpec for TRACECONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [traceconfig::R](R) reader structure"]
impl crate::Readable for TRACECONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [traceconfig::W](W) writer structure"]
impl crate::Writable for TRACECONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TRACECONFIG to value 0"]
impl crate::Resettable for TRACECONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
