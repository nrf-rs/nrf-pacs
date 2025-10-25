#[doc = "Register `USBREGSTATUS` reader"]
pub type R = crate::R<UsbregstatusSpec>;
#[doc = "VBUS input detection status (USBDETECTED and USBREMOVED events are derived from this information)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vbusdetect {
    #[doc = "0: VBUS voltage below valid threshold"]
    NoVbus = 0,
    #[doc = "1: VBUS voltage above valid threshold"]
    VbusPresent = 1,
}
impl From<Vbusdetect> for bool {
    #[inline(always)]
    fn from(variant: Vbusdetect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBUSDETECT` reader - VBUS input detection status (USBDETECTED and USBREMOVED events are derived from this information)"]
pub type VbusdetectR = crate::BitReader<Vbusdetect>;
impl VbusdetectR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vbusdetect {
        match self.bits {
            false => Vbusdetect::NoVbus,
            true => Vbusdetect::VbusPresent,
        }
    }
    #[doc = "VBUS voltage below valid threshold"]
    #[inline(always)]
    pub fn is_no_vbus(&self) -> bool {
        *self == Vbusdetect::NoVbus
    }
    #[doc = "VBUS voltage above valid threshold"]
    #[inline(always)]
    pub fn is_vbus_present(&self) -> bool {
        *self == Vbusdetect::VbusPresent
    }
}
#[doc = "USB supply output settling time elapsed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outputrdy {
    #[doc = "0: USBREG output settling time not elapsed"]
    NotReady = 0,
    #[doc = "1: USBREG output settling time elapsed (same information as USBPWRRDY event)"]
    Ready = 1,
}
impl From<Outputrdy> for bool {
    #[inline(always)]
    fn from(variant: Outputrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OUTPUTRDY` reader - USB supply output settling time elapsed"]
pub type OutputrdyR = crate::BitReader<Outputrdy>;
impl OutputrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Outputrdy {
        match self.bits {
            false => Outputrdy::NotReady,
            true => Outputrdy::Ready,
        }
    }
    #[doc = "USBREG output settling time not elapsed"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == Outputrdy::NotReady
    }
    #[doc = "USBREG output settling time elapsed (same information as USBPWRRDY event)"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == Outputrdy::Ready
    }
}
impl R {
    #[doc = "Bit 0 - VBUS input detection status (USBDETECTED and USBREMOVED events are derived from this information)"]
    #[inline(always)]
    pub fn vbusdetect(&self) -> VbusdetectR {
        VbusdetectR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USB supply output settling time elapsed"]
    #[inline(always)]
    pub fn outputrdy(&self) -> OutputrdyR {
        OutputrdyR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "USB supply status\n\nYou can [`read`](crate::Reg::read) this register and get [`usbregstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsbregstatusSpec;
impl crate::RegisterSpec for UsbregstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usbregstatus::R`](R) reader structure"]
impl crate::Readable for UsbregstatusSpec {}
#[doc = "`reset()` method sets USBREGSTATUS to value 0"]
impl crate::Resettable for UsbregstatusSpec {}
