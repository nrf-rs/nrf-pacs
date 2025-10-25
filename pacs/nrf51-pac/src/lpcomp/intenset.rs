#[doc = "Register `INTENSET` reader"]
pub type R = crate::R<IntensetSpec>;
#[doc = "Register `INTENSET` writer"]
pub type W = crate::W<IntensetSpec>;
#[doc = "Enable interrupt on READY event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ready {
    #[doc = "0: Interrupt disabled."]
    Disabled = 0,
    #[doc = "1: Interrupt enabled."]
    Enabled = 1,
}
impl From<Ready> for bool {
    #[inline(always)]
    fn from(variant: Ready) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READY` reader - Enable interrupt on READY event."]
pub type ReadyR = crate::BitReader<Ready>;
impl ReadyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ready {
        match self.bits {
            false => Ready::Disabled,
            true => Ready::Enabled,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ready::Disabled
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ready::Enabled
    }
}
#[doc = "Enable interrupt on READY event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReadyWO {
    #[doc = "1: Enable interrupt on write."]
    Set = 1,
}
impl From<ReadyWO> for bool {
    #[inline(always)]
    fn from(variant: ReadyWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READY` writer - Enable interrupt on READY event."]
pub type ReadyW<'a, REG> = crate::BitWriter<'a, REG, ReadyWO>;
impl<'a, REG> ReadyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt on write."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(ReadyWO::Set)
    }
}
#[doc = "Enable interrupt on DOWN event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Down {
    #[doc = "0: Interrupt disabled."]
    Disabled = 0,
    #[doc = "1: Interrupt enabled."]
    Enabled = 1,
}
impl From<Down> for bool {
    #[inline(always)]
    fn from(variant: Down) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOWN` reader - Enable interrupt on DOWN event."]
pub type DownR = crate::BitReader<Down>;
impl DownR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Down {
        match self.bits {
            false => Down::Disabled,
            true => Down::Enabled,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Down::Disabled
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Down::Enabled
    }
}
#[doc = "Enable interrupt on DOWN event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DownWO {
    #[doc = "1: Enable interrupt on write."]
    Set = 1,
}
impl From<DownWO> for bool {
    #[inline(always)]
    fn from(variant: DownWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOWN` writer - Enable interrupt on DOWN event."]
pub type DownW<'a, REG> = crate::BitWriter<'a, REG, DownWO>;
impl<'a, REG> DownW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt on write."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(DownWO::Set)
    }
}
#[doc = "Enable interrupt on UP event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Up {
    #[doc = "0: Interrupt disabled."]
    Disabled = 0,
    #[doc = "1: Interrupt enabled."]
    Enabled = 1,
}
impl From<Up> for bool {
    #[inline(always)]
    fn from(variant: Up) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UP` reader - Enable interrupt on UP event."]
pub type UpR = crate::BitReader<Up>;
impl UpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Up {
        match self.bits {
            false => Up::Disabled,
            true => Up::Enabled,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Up::Disabled
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Up::Enabled
    }
}
#[doc = "Enable interrupt on UP event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UpWO {
    #[doc = "1: Enable interrupt on write."]
    Set = 1,
}
impl From<UpWO> for bool {
    #[inline(always)]
    fn from(variant: UpWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UP` writer - Enable interrupt on UP event."]
pub type UpW<'a, REG> = crate::BitWriter<'a, REG, UpWO>;
impl<'a, REG> UpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt on write."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(UpWO::Set)
    }
}
#[doc = "Enable interrupt on CROSS event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cross {
    #[doc = "0: Interrupt disabled."]
    Disabled = 0,
    #[doc = "1: Interrupt enabled."]
    Enabled = 1,
}
impl From<Cross> for bool {
    #[inline(always)]
    fn from(variant: Cross) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CROSS` reader - Enable interrupt on CROSS event."]
pub type CrossR = crate::BitReader<Cross>;
impl CrossR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cross {
        match self.bits {
            false => Cross::Disabled,
            true => Cross::Enabled,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Cross::Disabled
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Cross::Enabled
    }
}
#[doc = "Enable interrupt on CROSS event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CrossWO {
    #[doc = "1: Enable interrupt on write."]
    Set = 1,
}
impl From<CrossWO> for bool {
    #[inline(always)]
    fn from(variant: CrossWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CROSS` writer - Enable interrupt on CROSS event."]
pub type CrossW<'a, REG> = crate::BitWriter<'a, REG, CrossWO>;
impl<'a, REG> CrossW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt on write."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(CrossWO::Set)
    }
}
impl R {
    #[doc = "Bit 0 - Enable interrupt on READY event."]
    #[inline(always)]
    pub fn ready(&self) -> ReadyR {
        ReadyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable interrupt on DOWN event."]
    #[inline(always)]
    pub fn down(&self) -> DownR {
        DownR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable interrupt on UP event."]
    #[inline(always)]
    pub fn up(&self) -> UpR {
        UpR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable interrupt on CROSS event."]
    #[inline(always)]
    pub fn cross(&self) -> CrossR {
        CrossR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable interrupt on READY event."]
    #[inline(always)]
    pub fn ready(&mut self) -> ReadyW<'_, IntensetSpec> {
        ReadyW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable interrupt on DOWN event."]
    #[inline(always)]
    pub fn down(&mut self) -> DownW<'_, IntensetSpec> {
        DownW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable interrupt on UP event."]
    #[inline(always)]
    pub fn up(&mut self) -> UpW<'_, IntensetSpec> {
        UpW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable interrupt on CROSS event."]
    #[inline(always)]
    pub fn cross(&mut self) -> CrossW<'_, IntensetSpec> {
        CrossW::new(self, 3)
    }
}
#[doc = "Interrupt enable set register.\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
