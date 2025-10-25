#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<StatusSpec>;
#[doc = "TX buffer over-read detected, and prevented\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Overread {
    #[doc = "0: Read: error not present"]
    NotPresent = 0,
    #[doc = "1: Read: error present"]
    Present = 1,
}
impl From<Overread> for bool {
    #[inline(always)]
    fn from(variant: Overread) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVERREAD` reader - TX buffer over-read detected, and prevented"]
pub type OverreadR = crate::BitReader<Overread>;
impl OverreadR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Overread {
        match self.bits {
            false => Overread::NotPresent,
            true => Overread::Present,
        }
    }
    #[doc = "Read: error not present"]
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        *self == Overread::NotPresent
    }
    #[doc = "Read: error present"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == Overread::Present
    }
}
#[doc = "TX buffer over-read detected, and prevented\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OverreadWO {
    #[doc = "1: Write: clear error on writing '1'"]
    Clear = 1,
}
impl From<OverreadWO> for bool {
    #[inline(always)]
    fn from(variant: OverreadWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVERREAD` writer - TX buffer over-read detected, and prevented"]
pub type OverreadW<'a, REG> = crate::BitWriter<'a, REG, OverreadWO>;
impl<'a, REG> OverreadW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write: clear error on writing '1'"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(OverreadWO::Clear)
    }
}
#[doc = "RX buffer overflow detected, and prevented\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Overflow {
    #[doc = "0: Read: error not present"]
    NotPresent = 0,
    #[doc = "1: Read: error present"]
    Present = 1,
}
impl From<Overflow> for bool {
    #[inline(always)]
    fn from(variant: Overflow) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVERFLOW` reader - RX buffer overflow detected, and prevented"]
pub type OverflowR = crate::BitReader<Overflow>;
impl OverflowR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Overflow {
        match self.bits {
            false => Overflow::NotPresent,
            true => Overflow::Present,
        }
    }
    #[doc = "Read: error not present"]
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        *self == Overflow::NotPresent
    }
    #[doc = "Read: error present"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == Overflow::Present
    }
}
#[doc = "RX buffer overflow detected, and prevented\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OverflowWO {
    #[doc = "1: Write: clear error on writing '1'"]
    Clear = 1,
}
impl From<OverflowWO> for bool {
    #[inline(always)]
    fn from(variant: OverflowWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVERFLOW` writer - RX buffer overflow detected, and prevented"]
pub type OverflowW<'a, REG> = crate::BitWriter<'a, REG, OverflowWO>;
impl<'a, REG> OverflowW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write: clear error on writing '1'"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(OverflowWO::Clear)
    }
}
impl R {
    #[doc = "Bit 0 - TX buffer over-read detected, and prevented"]
    #[inline(always)]
    pub fn overread(&self) -> OverreadR {
        OverreadR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RX buffer overflow detected, and prevented"]
    #[inline(always)]
    pub fn overflow(&self) -> OverflowR {
        OverflowR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TX buffer over-read detected, and prevented"]
    #[inline(always)]
    pub fn overread(&mut self) -> OverreadW<'_, StatusSpec> {
        OverreadW::new(self, 0)
    }
    #[doc = "Bit 1 - RX buffer overflow detected, and prevented"]
    #[inline(always)]
    pub fn overflow(&mut self) -> OverflowW<'_, StatusSpec> {
        OverflowW::new(self, 1)
    }
}
#[doc = "Status from last transaction\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for StatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {}
