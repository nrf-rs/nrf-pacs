#[doc = "Register `EVENTCAUSE` reader"]
pub type R = crate::R<EventcauseSpec>;
#[doc = "Register `EVENTCAUSE` writer"]
pub type W = crate::W<EventcauseSpec>;
#[doc = "CRC error was detected on isochronous OUT endpoint 8. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Isooutcrc {
    #[doc = "0: No error detected"]
    NotDetected = 0,
    #[doc = "1: Error detected"]
    Detected = 1,
}
impl From<Isooutcrc> for bool {
    #[inline(always)]
    fn from(variant: Isooutcrc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISOOUTCRC` reader - CRC error was detected on isochronous OUT endpoint 8. Write '1' to clear."]
pub type IsooutcrcR = crate::BitReader<Isooutcrc>;
impl IsooutcrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Isooutcrc {
        match self.bits {
            false => Isooutcrc::NotDetected,
            true => Isooutcrc::Detected,
        }
    }
    #[doc = "No error detected"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == Isooutcrc::NotDetected
    }
    #[doc = "Error detected"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == Isooutcrc::Detected
    }
}
#[doc = "Field `ISOOUTCRC` writer - CRC error was detected on isochronous OUT endpoint 8. Write '1' to clear."]
pub type IsooutcrcW<'a, REG> = crate::BitWriter1C<'a, REG, Isooutcrc>;
impl<'a, REG> IsooutcrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No error detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut crate::W<REG> {
        self.variant(Isooutcrc::NotDetected)
    }
    #[doc = "Error detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut crate::W<REG> {
        self.variant(Isooutcrc::Detected)
    }
}
#[doc = "Signals that USB lines have been idle long enough for the device to enter suspend. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Suspend {
    #[doc = "0: Suspend not detected"]
    NotDetected = 0,
    #[doc = "1: Suspend detected"]
    Detected = 1,
}
impl From<Suspend> for bool {
    #[inline(always)]
    fn from(variant: Suspend) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUSPEND` reader - Signals that USB lines have been idle long enough for the device to enter suspend. Write '1' to clear."]
pub type SuspendR = crate::BitReader<Suspend>;
impl SuspendR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Suspend {
        match self.bits {
            false => Suspend::NotDetected,
            true => Suspend::Detected,
        }
    }
    #[doc = "Suspend not detected"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == Suspend::NotDetected
    }
    #[doc = "Suspend detected"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == Suspend::Detected
    }
}
#[doc = "Field `SUSPEND` writer - Signals that USB lines have been idle long enough for the device to enter suspend. Write '1' to clear."]
pub type SuspendW<'a, REG> = crate::BitWriter1C<'a, REG, Suspend>;
impl<'a, REG> SuspendW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Suspend not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut crate::W<REG> {
        self.variant(Suspend::NotDetected)
    }
    #[doc = "Suspend detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut crate::W<REG> {
        self.variant(Suspend::Detected)
    }
}
#[doc = "Signals that a RESUME condition (K state or activity restart) has been detected on USB lines. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Resume {
    #[doc = "0: Resume not detected"]
    NotDetected = 0,
    #[doc = "1: Resume detected"]
    Detected = 1,
}
impl From<Resume> for bool {
    #[inline(always)]
    fn from(variant: Resume) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESUME` reader - Signals that a RESUME condition (K state or activity restart) has been detected on USB lines. Write '1' to clear."]
pub type ResumeR = crate::BitReader<Resume>;
impl ResumeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Resume {
        match self.bits {
            false => Resume::NotDetected,
            true => Resume::Detected,
        }
    }
    #[doc = "Resume not detected"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == Resume::NotDetected
    }
    #[doc = "Resume detected"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == Resume::Detected
    }
}
#[doc = "Field `RESUME` writer - Signals that a RESUME condition (K state or activity restart) has been detected on USB lines. Write '1' to clear."]
pub type ResumeW<'a, REG> = crate::BitWriter1C<'a, REG, Resume>;
impl<'a, REG> ResumeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Resume not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut crate::W<REG> {
        self.variant(Resume::NotDetected)
    }
    #[doc = "Resume detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut crate::W<REG> {
        self.variant(Resume::Detected)
    }
}
#[doc = "USB MAC has been woken up and operational. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usbwuallowed {
    #[doc = "0: Wake up not allowed"]
    NotAllowed = 0,
    #[doc = "1: Wake up allowed"]
    Allowed = 1,
}
impl From<Usbwuallowed> for bool {
    #[inline(always)]
    fn from(variant: Usbwuallowed) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBWUALLOWED` reader - USB MAC has been woken up and operational. Write '1' to clear."]
pub type UsbwuallowedR = crate::BitReader<Usbwuallowed>;
impl UsbwuallowedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usbwuallowed {
        match self.bits {
            false => Usbwuallowed::NotAllowed,
            true => Usbwuallowed::Allowed,
        }
    }
    #[doc = "Wake up not allowed"]
    #[inline(always)]
    pub fn is_not_allowed(&self) -> bool {
        *self == Usbwuallowed::NotAllowed
    }
    #[doc = "Wake up allowed"]
    #[inline(always)]
    pub fn is_allowed(&self) -> bool {
        *self == Usbwuallowed::Allowed
    }
}
#[doc = "Field `USBWUALLOWED` writer - USB MAC has been woken up and operational. Write '1' to clear."]
pub type UsbwuallowedW<'a, REG> = crate::BitWriter1C<'a, REG, Usbwuallowed>;
impl<'a, REG> UsbwuallowedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wake up not allowed"]
    #[inline(always)]
    pub fn not_allowed(self) -> &'a mut crate::W<REG> {
        self.variant(Usbwuallowed::NotAllowed)
    }
    #[doc = "Wake up allowed"]
    #[inline(always)]
    pub fn allowed(self) -> &'a mut crate::W<REG> {
        self.variant(Usbwuallowed::Allowed)
    }
}
#[doc = "USB device is ready for normal operation. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ready {
    #[doc = "0: USBEVENT was not issued due to USBD peripheral ready"]
    NotDetected = 0,
    #[doc = "1: USBD peripheral is ready"]
    Ready = 1,
}
impl From<Ready> for bool {
    #[inline(always)]
    fn from(variant: Ready) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READY` reader - USB device is ready for normal operation. Write '1' to clear."]
pub type ReadyR = crate::BitReader<Ready>;
impl ReadyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ready {
        match self.bits {
            false => Ready::NotDetected,
            true => Ready::Ready,
        }
    }
    #[doc = "USBEVENT was not issued due to USBD peripheral ready"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == Ready::NotDetected
    }
    #[doc = "USBD peripheral is ready"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == Ready::Ready
    }
}
#[doc = "Field `READY` writer - USB device is ready for normal operation. Write '1' to clear."]
pub type ReadyW<'a, REG> = crate::BitWriter1C<'a, REG, Ready>;
impl<'a, REG> ReadyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "USBEVENT was not issued due to USBD peripheral ready"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut crate::W<REG> {
        self.variant(Ready::NotDetected)
    }
    #[doc = "USBD peripheral is ready"]
    #[inline(always)]
    pub fn ready(self) -> &'a mut crate::W<REG> {
        self.variant(Ready::Ready)
    }
}
impl R {
    #[doc = "Bit 0 - CRC error was detected on isochronous OUT endpoint 8. Write '1' to clear."]
    #[inline(always)]
    pub fn isooutcrc(&self) -> IsooutcrcR {
        IsooutcrcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Signals that USB lines have been idle long enough for the device to enter suspend. Write '1' to clear."]
    #[inline(always)]
    pub fn suspend(&self) -> SuspendR {
        SuspendR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Signals that a RESUME condition (K state or activity restart) has been detected on USB lines. Write '1' to clear."]
    #[inline(always)]
    pub fn resume(&self) -> ResumeR {
        ResumeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - USB MAC has been woken up and operational. Write '1' to clear."]
    #[inline(always)]
    pub fn usbwuallowed(&self) -> UsbwuallowedR {
        UsbwuallowedR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - USB device is ready for normal operation. Write '1' to clear."]
    #[inline(always)]
    pub fn ready(&self) -> ReadyR {
        ReadyR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CRC error was detected on isochronous OUT endpoint 8. Write '1' to clear."]
    #[inline(always)]
    pub fn isooutcrc(&mut self) -> IsooutcrcW<'_, EventcauseSpec> {
        IsooutcrcW::new(self, 0)
    }
    #[doc = "Bit 8 - Signals that USB lines have been idle long enough for the device to enter suspend. Write '1' to clear."]
    #[inline(always)]
    pub fn suspend(&mut self) -> SuspendW<'_, EventcauseSpec> {
        SuspendW::new(self, 8)
    }
    #[doc = "Bit 9 - Signals that a RESUME condition (K state or activity restart) has been detected on USB lines. Write '1' to clear."]
    #[inline(always)]
    pub fn resume(&mut self) -> ResumeW<'_, EventcauseSpec> {
        ResumeW::new(self, 9)
    }
    #[doc = "Bit 10 - USB MAC has been woken up and operational. Write '1' to clear."]
    #[inline(always)]
    pub fn usbwuallowed(&mut self) -> UsbwuallowedW<'_, EventcauseSpec> {
        UsbwuallowedW::new(self, 10)
    }
    #[doc = "Bit 11 - USB device is ready for normal operation. Write '1' to clear."]
    #[inline(always)]
    pub fn ready(&mut self) -> ReadyW<'_, EventcauseSpec> {
        ReadyW::new(self, 11)
    }
}
#[doc = "Details on what caused the USBEVENT event\n\nYou can [`read`](crate::Reg::read) this register and get [`eventcause::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eventcause::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventcauseSpec;
impl crate::RegisterSpec for EventcauseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eventcause::R`](R) reader structure"]
impl crate::Readable for EventcauseSpec {}
#[doc = "`write(|w| ..)` method takes [`eventcause::W`](W) writer structure"]
impl crate::Writable for EventcauseSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0f01;
}
#[doc = "`reset()` method sets EVENTCAUSE to value 0"]
impl crate::Resettable for EventcauseSpec {}
