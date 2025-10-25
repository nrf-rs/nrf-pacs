#[doc = "Register `INTENSET` reader"]
pub type R = crate::R<IntensetSpec>;
#[doc = "Register `INTENSET` writer"]
pub type W = crate::W<IntensetSpec>;
#[doc = "Write '1' to enable interrupt for event USBDETECTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usbdetected {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Usbdetected> for bool {
    #[inline(always)]
    fn from(variant: Usbdetected) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBDETECTED` reader - Write '1' to enable interrupt for event USBDETECTED"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Usbdetected::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Usbdetected::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event USBDETECTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsbdetectedWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<UsbdetectedWO> for bool {
    #[inline(always)]
    fn from(variant: UsbdetectedWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBDETECTED` writer - Write '1' to enable interrupt for event USBDETECTED"]
pub type UsbdetectedW<'a, REG> = crate::BitWriter<'a, REG, UsbdetectedWO>;
impl<'a, REG> UsbdetectedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(UsbdetectedWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event USBREMOVED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usbremoved {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Usbremoved> for bool {
    #[inline(always)]
    fn from(variant: Usbremoved) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBREMOVED` reader - Write '1' to enable interrupt for event USBREMOVED"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Usbremoved::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Usbremoved::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event USBREMOVED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsbremovedWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<UsbremovedWO> for bool {
    #[inline(always)]
    fn from(variant: UsbremovedWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBREMOVED` writer - Write '1' to enable interrupt for event USBREMOVED"]
pub type UsbremovedW<'a, REG> = crate::BitWriter<'a, REG, UsbremovedWO>;
impl<'a, REG> UsbremovedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(UsbremovedWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event USBPWRRDY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usbpwrrdy {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Usbpwrrdy> for bool {
    #[inline(always)]
    fn from(variant: Usbpwrrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBPWRRDY` reader - Write '1' to enable interrupt for event USBPWRRDY"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Usbpwrrdy::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Usbpwrrdy::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event USBPWRRDY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsbpwrrdyWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<UsbpwrrdyWO> for bool {
    #[inline(always)]
    fn from(variant: UsbpwrrdyWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBPWRRDY` writer - Write '1' to enable interrupt for event USBPWRRDY"]
pub type UsbpwrrdyW<'a, REG> = crate::BitWriter<'a, REG, UsbpwrrdyWO>;
impl<'a, REG> UsbpwrrdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(UsbpwrrdyWO::Set)
    }
}
impl R {
    #[doc = "Bit 0 - Write '1' to enable interrupt for event USBDETECTED"]
    #[inline(always)]
    pub fn usbdetected(&self) -> UsbdetectedR {
        UsbdetectedR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write '1' to enable interrupt for event USBREMOVED"]
    #[inline(always)]
    pub fn usbremoved(&self) -> UsbremovedR {
        UsbremovedR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write '1' to enable interrupt for event USBPWRRDY"]
    #[inline(always)]
    pub fn usbpwrrdy(&self) -> UsbpwrrdyR {
        UsbpwrrdyR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to enable interrupt for event USBDETECTED"]
    #[inline(always)]
    pub fn usbdetected(&mut self) -> UsbdetectedW<'_, IntensetSpec> {
        UsbdetectedW::new(self, 0)
    }
    #[doc = "Bit 1 - Write '1' to enable interrupt for event USBREMOVED"]
    #[inline(always)]
    pub fn usbremoved(&mut self) -> UsbremovedW<'_, IntensetSpec> {
        UsbremovedW::new(self, 1)
    }
    #[doc = "Bit 2 - Write '1' to enable interrupt for event USBPWRRDY"]
    #[inline(always)]
    pub fn usbpwrrdy(&mut self) -> UsbpwrrdyW<'_, IntensetSpec> {
        UsbpwrrdyW::new(self, 2)
    }
}
#[doc = "Enable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntensetSpec;
impl crate::RegisterSpec for IntensetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenset::R`](R) reader structure"]
impl crate::Readable for IntensetSpec {}
#[doc = "`write(|w| ..)` method takes [`intenset::W`](W) writer structure"]
impl crate::Writable for IntensetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for IntensetSpec {}
