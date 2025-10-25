#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<IntenclrSpec>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
#[doc = "Write '1' to disable interrupt for event DATARDY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Datardy {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Datardy> for bool {
    #[inline(always)]
    fn from(variant: Datardy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATARDY` reader - Write '1' to disable interrupt for event DATARDY"]
pub type DatardyR = crate::BitReader<Datardy>;
impl DatardyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Datardy {
        match self.bits {
            false => Datardy::Disabled,
            true => Datardy::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Datardy::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Datardy::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event DATARDY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DatardyWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<DatardyWO> for bool {
    #[inline(always)]
    fn from(variant: DatardyWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATARDY` writer - Write '1' to disable interrupt for event DATARDY"]
pub type DatardyW<'a, REG> = crate::BitWriter<'a, REG, DatardyWO>;
impl<'a, REG> DatardyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(DatardyWO::Clear)
    }
}
impl R {
    #[doc = "Bit 0 - Write '1' to disable interrupt for event DATARDY"]
    #[inline(always)]
    pub fn datardy(&self) -> DatardyR {
        DatardyR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to disable interrupt for event DATARDY"]
    #[inline(always)]
    pub fn datardy(&mut self) -> DatardyW<'_, IntenclrSpec> {
        DatardyW::new(self, 0)
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
