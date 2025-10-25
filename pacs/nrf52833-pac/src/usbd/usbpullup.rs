#[doc = "Register `USBPULLUP` reader"]
pub type R = crate::R<UsbpullupSpec>;
#[doc = "Register `USBPULLUP` writer"]
pub type W = crate::W<UsbpullupSpec>;
#[doc = "Control of the USB pull-up on the D+ line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Connect {
    #[doc = "0: Pull-up is disconnected"]
    Disabled = 0,
    #[doc = "1: Pull-up is connected to D+"]
    Enabled = 1,
}
impl From<Connect> for bool {
    #[inline(always)]
    fn from(variant: Connect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CONNECT` reader - Control of the USB pull-up on the D+ line"]
pub type ConnectR = crate::BitReader<Connect>;
impl ConnectR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Connect {
        match self.bits {
            false => Connect::Disabled,
            true => Connect::Enabled,
        }
    }
    #[doc = "Pull-up is disconnected"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Connect::Disabled
    }
    #[doc = "Pull-up is connected to D+"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Connect::Enabled
    }
}
#[doc = "Field `CONNECT` writer - Control of the USB pull-up on the D+ line"]
pub type ConnectW<'a, REG> = crate::BitWriter<'a, REG, Connect>;
impl<'a, REG> ConnectW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pull-up is disconnected"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Connect::Disabled)
    }
    #[doc = "Pull-up is connected to D+"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Connect::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Control of the USB pull-up on the D+ line"]
    #[inline(always)]
    pub fn connect(&self) -> ConnectR {
        ConnectR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Control of the USB pull-up on the D+ line"]
    #[inline(always)]
    pub fn connect(&mut self) -> ConnectW<'_, UsbpullupSpec> {
        ConnectW::new(self, 0)
    }
}
#[doc = "Control of the USB pull-up\n\nYou can [`read`](crate::Reg::read) this register and get [`usbpullup::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbpullup::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsbpullupSpec;
impl crate::RegisterSpec for UsbpullupSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usbpullup::R`](R) reader structure"]
impl crate::Readable for UsbpullupSpec {}
#[doc = "`write(|w| ..)` method takes [`usbpullup::W`](W) writer structure"]
impl crate::Writable for UsbpullupSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets USBPULLUP to value 0"]
impl crate::Resettable for UsbpullupSpec {}
