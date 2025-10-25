#[doc = "Register `ERRORSRC` reader"]
pub type R = crate::R<ErrorsrcSpec>;
#[doc = "Register `ERRORSRC` writer"]
pub type W = crate::W<ErrorsrcSpec>;
#[doc = "RX buffer overflow detected, and prevented\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Overflow {
    #[doc = "0: Error did not occur"]
    NotDetected = 0,
    #[doc = "1: Error occurred"]
    Detected = 1,
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
            false => Overflow::NotDetected,
            true => Overflow::Detected,
        }
    }
    #[doc = "Error did not occur"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == Overflow::NotDetected
    }
    #[doc = "Error occurred"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == Overflow::Detected
    }
}
#[doc = "Field `OVERFLOW` writer - RX buffer overflow detected, and prevented"]
pub type OverflowW<'a, REG> = crate::BitWriter1C<'a, REG, Overflow>;
impl<'a, REG> OverflowW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Error did not occur"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut crate::W<REG> {
        self.variant(Overflow::NotDetected)
    }
    #[doc = "Error occurred"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut crate::W<REG> {
        self.variant(Overflow::Detected)
    }
}
#[doc = "NACK sent after receiving a data byte\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dnack {
    #[doc = "0: Error did not occur"]
    NotReceived = 0,
    #[doc = "1: Error occurred"]
    Received = 1,
}
impl From<Dnack> for bool {
    #[inline(always)]
    fn from(variant: Dnack) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DNACK` reader - NACK sent after receiving a data byte"]
pub type DnackR = crate::BitReader<Dnack>;
impl DnackR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dnack {
        match self.bits {
            false => Dnack::NotReceived,
            true => Dnack::Received,
        }
    }
    #[doc = "Error did not occur"]
    #[inline(always)]
    pub fn is_not_received(&self) -> bool {
        *self == Dnack::NotReceived
    }
    #[doc = "Error occurred"]
    #[inline(always)]
    pub fn is_received(&self) -> bool {
        *self == Dnack::Received
    }
}
#[doc = "Field `DNACK` writer - NACK sent after receiving a data byte"]
pub type DnackW<'a, REG> = crate::BitWriter1C<'a, REG, Dnack>;
impl<'a, REG> DnackW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Error did not occur"]
    #[inline(always)]
    pub fn not_received(self) -> &'a mut crate::W<REG> {
        self.variant(Dnack::NotReceived)
    }
    #[doc = "Error occurred"]
    #[inline(always)]
    pub fn received(self) -> &'a mut crate::W<REG> {
        self.variant(Dnack::Received)
    }
}
#[doc = "TX buffer over-read detected, and prevented\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Overread {
    #[doc = "0: Error did not occur"]
    NotDetected = 0,
    #[doc = "1: Error occurred"]
    Detected = 1,
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
            false => Overread::NotDetected,
            true => Overread::Detected,
        }
    }
    #[doc = "Error did not occur"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == Overread::NotDetected
    }
    #[doc = "Error occurred"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == Overread::Detected
    }
}
#[doc = "Field `OVERREAD` writer - TX buffer over-read detected, and prevented"]
pub type OverreadW<'a, REG> = crate::BitWriter1C<'a, REG, Overread>;
impl<'a, REG> OverreadW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Error did not occur"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut crate::W<REG> {
        self.variant(Overread::NotDetected)
    }
    #[doc = "Error occurred"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut crate::W<REG> {
        self.variant(Overread::Detected)
    }
}
impl R {
    #[doc = "Bit 0 - RX buffer overflow detected, and prevented"]
    #[inline(always)]
    pub fn overflow(&self) -> OverflowR {
        OverflowR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - NACK sent after receiving a data byte"]
    #[inline(always)]
    pub fn dnack(&self) -> DnackR {
        DnackR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TX buffer over-read detected, and prevented"]
    #[inline(always)]
    pub fn overread(&self) -> OverreadR {
        OverreadR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RX buffer overflow detected, and prevented"]
    #[inline(always)]
    pub fn overflow(&mut self) -> OverflowW<'_, ErrorsrcSpec> {
        OverflowW::new(self, 0)
    }
    #[doc = "Bit 2 - NACK sent after receiving a data byte"]
    #[inline(always)]
    pub fn dnack(&mut self) -> DnackW<'_, ErrorsrcSpec> {
        DnackW::new(self, 2)
    }
    #[doc = "Bit 3 - TX buffer over-read detected, and prevented"]
    #[inline(always)]
    pub fn overread(&mut self) -> OverreadW<'_, ErrorsrcSpec> {
        OverreadW::new(self, 3)
    }
}
#[doc = "Error source\n\nYou can [`read`](crate::Reg::read) this register and get [`errorsrc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`errorsrc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrorsrcSpec;
impl crate::RegisterSpec for ErrorsrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`errorsrc::R`](R) reader structure"]
impl crate::Readable for ErrorsrcSpec {}
#[doc = "`write(|w| ..)` method takes [`errorsrc::W`](W) writer structure"]
impl crate::Writable for ErrorsrcSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0d;
}
#[doc = "`reset()` method sets ERRORSRC to value 0"]
impl crate::Resettable for ErrorsrcSpec {}
