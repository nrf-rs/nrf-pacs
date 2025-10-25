#[doc = "Register `INTEN` reader"]
pub type R = crate::R<IntenSpec>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<IntenSpec>;
#[doc = "Enable or disable interrupt for event USBDETECTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usbdetected {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Usbdetected> for bool {
    #[inline(always)]
    fn from(variant: Usbdetected) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBDETECTED` reader - Enable or disable interrupt for event USBDETECTED"]
pub type UsbdetectedR = crate::BitReader<Usbdetected>;
impl UsbdetectedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usbdetected {
        match self.bits {
            false => Usbdetected::Disabled,
            true => Usbdetected::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Usbdetected::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Usbdetected::Enabled
    }
}
#[doc = "Field `USBDETECTED` writer - Enable or disable interrupt for event USBDETECTED"]
pub type UsbdetectedW<'a, REG> = crate::BitWriter<'a, REG, Usbdetected>;
impl<'a, REG> UsbdetectedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Usbdetected::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Usbdetected::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event USBREMOVED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usbremoved {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Usbremoved> for bool {
    #[inline(always)]
    fn from(variant: Usbremoved) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBREMOVED` reader - Enable or disable interrupt for event USBREMOVED"]
pub type UsbremovedR = crate::BitReader<Usbremoved>;
impl UsbremovedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usbremoved {
        match self.bits {
            false => Usbremoved::Disabled,
            true => Usbremoved::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Usbremoved::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Usbremoved::Enabled
    }
}
#[doc = "Field `USBREMOVED` writer - Enable or disable interrupt for event USBREMOVED"]
pub type UsbremovedW<'a, REG> = crate::BitWriter<'a, REG, Usbremoved>;
impl<'a, REG> UsbremovedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Usbremoved::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Usbremoved::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event USBPWRRDY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usbpwrrdy {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Usbpwrrdy> for bool {
    #[inline(always)]
    fn from(variant: Usbpwrrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBPWRRDY` reader - Enable or disable interrupt for event USBPWRRDY"]
pub type UsbpwrrdyR = crate::BitReader<Usbpwrrdy>;
impl UsbpwrrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usbpwrrdy {
        match self.bits {
            false => Usbpwrrdy::Disabled,
            true => Usbpwrrdy::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Usbpwrrdy::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Usbpwrrdy::Enabled
    }
}
#[doc = "Field `USBPWRRDY` writer - Enable or disable interrupt for event USBPWRRDY"]
pub type UsbpwrrdyW<'a, REG> = crate::BitWriter<'a, REG, Usbpwrrdy>;
impl<'a, REG> UsbpwrrdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Usbpwrrdy::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Usbpwrrdy::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Enable or disable interrupt for event USBDETECTED"]
    #[inline(always)]
    pub fn usbdetected(&self) -> UsbdetectedR {
        UsbdetectedR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable or disable interrupt for event USBREMOVED"]
    #[inline(always)]
    pub fn usbremoved(&self) -> UsbremovedR {
        UsbremovedR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable or disable interrupt for event USBPWRRDY"]
    #[inline(always)]
    pub fn usbpwrrdy(&self) -> UsbpwrrdyR {
        UsbpwrrdyR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable interrupt for event USBDETECTED"]
    #[inline(always)]
    pub fn usbdetected(&mut self) -> UsbdetectedW<'_, IntenSpec> {
        UsbdetectedW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable or disable interrupt for event USBREMOVED"]
    #[inline(always)]
    pub fn usbremoved(&mut self) -> UsbremovedW<'_, IntenSpec> {
        UsbremovedW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable or disable interrupt for event USBPWRRDY"]
    #[inline(always)]
    pub fn usbpwrrdy(&mut self) -> UsbpwrrdyW<'_, IntenSpec> {
        UsbpwrrdyW::new(self, 2)
    }
}
#[doc = "Enable or disable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenSpec;
impl crate::RegisterSpec for IntenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for IntenSpec {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for IntenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for IntenSpec {}
