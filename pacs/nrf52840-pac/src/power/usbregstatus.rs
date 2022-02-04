#[doc = "Register `USBREGSTATUS` reader"]
pub struct R(crate::R<USBREGSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBREGSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBREGSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBREGSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "VBUS input detection status (USBDETECTED and USBREMOVED events are derived from this information)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBUSDETECT_A {
    #[doc = "0: VBUS voltage below valid threshold"]
    NOVBUS = 0,
    #[doc = "1: VBUS voltage above valid threshold"]
    VBUSPRESENT = 1,
}
impl From<VBUSDETECT_A> for bool {
    #[inline(always)]
    fn from(variant: VBUSDETECT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBUSDETECT` reader - VBUS input detection status (USBDETECTED and USBREMOVED events are derived from this information)"]
pub struct VBUSDETECT_R(crate::FieldReader<bool, VBUSDETECT_A>);
impl VBUSDETECT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VBUSDETECT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBUSDETECT_A {
        match self.bits {
            false => VBUSDETECT_A::NOVBUS,
            true => VBUSDETECT_A::VBUSPRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOVBUS`"]
    #[inline(always)]
    pub fn is_no_vbus(&self) -> bool {
        **self == VBUSDETECT_A::NOVBUS
    }
    #[doc = "Checks if the value of the field is `VBUSPRESENT`"]
    #[inline(always)]
    pub fn is_vbus_present(&self) -> bool {
        **self == VBUSDETECT_A::VBUSPRESENT
    }
}
impl core::ops::Deref for VBUSDETECT_R {
    type Target = crate::FieldReader<bool, VBUSDETECT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "USB supply output settling time elapsed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTPUTRDY_A {
    #[doc = "0: USBREG output settling time not elapsed"]
    NOTREADY = 0,
    #[doc = "1: USBREG output settling time elapsed (same information as USBPWRRDY event)"]
    READY = 1,
}
impl From<OUTPUTRDY_A> for bool {
    #[inline(always)]
    fn from(variant: OUTPUTRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OUTPUTRDY` reader - USB supply output settling time elapsed"]
pub struct OUTPUTRDY_R(crate::FieldReader<bool, OUTPUTRDY_A>);
impl OUTPUTRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUTPUTRDY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTPUTRDY_A {
        match self.bits {
            false => OUTPUTRDY_A::NOTREADY,
            true => OUTPUTRDY_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOTREADY`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        **self == OUTPUTRDY_A::NOTREADY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        **self == OUTPUTRDY_A::READY
    }
}
impl core::ops::Deref for OUTPUTRDY_R {
    type Target = crate::FieldReader<bool, OUTPUTRDY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - VBUS input detection status (USBDETECTED and USBREMOVED events are derived from this information)"]
    #[inline(always)]
    pub fn vbusdetect(&self) -> VBUSDETECT_R {
        VBUSDETECT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - USB supply output settling time elapsed"]
    #[inline(always)]
    pub fn outputrdy(&self) -> OUTPUTRDY_R {
        OUTPUTRDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
#[doc = "USB supply status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbregstatus](index.html) module"]
pub struct USBREGSTATUS_SPEC;
impl crate::RegisterSpec for USBREGSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbregstatus::R](R) reader structure"]
impl crate::Readable for USBREGSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets USBREGSTATUS to value 0"]
impl crate::Resettable for USBREGSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
