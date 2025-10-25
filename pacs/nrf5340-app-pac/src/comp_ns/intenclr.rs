#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<IntenclrSpec>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
#[doc = "Write '1' to disable interrupt for event READY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ready {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Ready> for bool {
    #[inline(always)]
    fn from(variant: Ready) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READY` reader - Write '1' to disable interrupt for event READY"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ready::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ready::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event READY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReadyWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<ReadyWO> for bool {
    #[inline(always)]
    fn from(variant: ReadyWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READY` writer - Write '1' to disable interrupt for event READY"]
pub type ReadyW<'a, REG> = crate::BitWriter<'a, REG, ReadyWO>;
impl<'a, REG> ReadyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ReadyWO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event DOWN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Down {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Down> for bool {
    #[inline(always)]
    fn from(variant: Down) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOWN` reader - Write '1' to disable interrupt for event DOWN"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Down::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Down::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event DOWN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DownWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<DownWO> for bool {
    #[inline(always)]
    fn from(variant: DownWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOWN` writer - Write '1' to disable interrupt for event DOWN"]
pub type DownW<'a, REG> = crate::BitWriter<'a, REG, DownWO>;
impl<'a, REG> DownW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(DownWO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event UP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Up {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Up> for bool {
    #[inline(always)]
    fn from(variant: Up) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UP` reader - Write '1' to disable interrupt for event UP"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Up::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Up::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event UP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UpWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<UpWO> for bool {
    #[inline(always)]
    fn from(variant: UpWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UP` writer - Write '1' to disable interrupt for event UP"]
pub type UpW<'a, REG> = crate::BitWriter<'a, REG, UpWO>;
impl<'a, REG> UpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(UpWO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event CROSS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cross {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Cross> for bool {
    #[inline(always)]
    fn from(variant: Cross) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CROSS` reader - Write '1' to disable interrupt for event CROSS"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Cross::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Cross::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event CROSS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CrossWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<CrossWO> for bool {
    #[inline(always)]
    fn from(variant: CrossWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CROSS` writer - Write '1' to disable interrupt for event CROSS"]
pub type CrossW<'a, REG> = crate::BitWriter<'a, REG, CrossWO>;
impl<'a, REG> CrossW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CrossWO::Clear)
    }
}
impl R {
    #[doc = "Bit 0 - Write '1' to disable interrupt for event READY"]
    #[inline(always)]
    pub fn ready(&self) -> ReadyR {
        ReadyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write '1' to disable interrupt for event DOWN"]
    #[inline(always)]
    pub fn down(&self) -> DownR {
        DownR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write '1' to disable interrupt for event UP"]
    #[inline(always)]
    pub fn up(&self) -> UpR {
        UpR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write '1' to disable interrupt for event CROSS"]
    #[inline(always)]
    pub fn cross(&self) -> CrossR {
        CrossR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to disable interrupt for event READY"]
    #[inline(always)]
    pub fn ready(&mut self) -> ReadyW<'_, IntenclrSpec> {
        ReadyW::new(self, 0)
    }
    #[doc = "Bit 1 - Write '1' to disable interrupt for event DOWN"]
    #[inline(always)]
    pub fn down(&mut self) -> DownW<'_, IntenclrSpec> {
        DownW::new(self, 1)
    }
    #[doc = "Bit 2 - Write '1' to disable interrupt for event UP"]
    #[inline(always)]
    pub fn up(&mut self) -> UpW<'_, IntenclrSpec> {
        UpW::new(self, 2)
    }
    #[doc = "Bit 3 - Write '1' to disable interrupt for event CROSS"]
    #[inline(always)]
    pub fn cross(&mut self) -> CrossW<'_, IntenclrSpec> {
        CrossW::new(self, 3)
    }
}
#[doc = "Disable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenclrSpec;
impl crate::RegisterSpec for IntenclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenclr::R`](R) reader structure"]
impl crate::Readable for IntenclrSpec {}
#[doc = "`write(|w| ..)` method takes [`intenclr::W`](W) writer structure"]
impl crate::Writable for IntenclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for IntenclrSpec {}
