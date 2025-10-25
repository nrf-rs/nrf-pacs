#[doc = "Register `CTIINTACK` writer"]
pub type W = crate::W<CtiintackSpec>;
#[doc = "Processor debug request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Debugreq {
    #[doc = "1: Clears the ctitrigout."]
    Acknowledge = 1,
}
impl From<Debugreq> for bool {
    #[inline(always)]
    fn from(variant: Debugreq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEBUGREQ` writer - Processor debug request"]
pub type DebugreqW<'a, REG> = crate::BitWriter<'a, REG, Debugreq>;
impl<'a, REG> DebugreqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the ctitrigout."]
    #[inline(always)]
    pub fn acknowledge(self) -> &'a mut crate::W<REG> {
        self.variant(Debugreq::Acknowledge)
    }
}
#[doc = "Processor Restart\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpurestart {
    #[doc = "1: Clears the ctitrigout."]
    Acknowledge = 1,
}
impl From<Cpurestart> for bool {
    #[inline(always)]
    fn from(variant: Cpurestart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPURESTART` writer - Processor Restart"]
pub type CpurestartW<'a, REG> = crate::BitWriter<'a, REG, Cpurestart>;
impl<'a, REG> CpurestartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the ctitrigout."]
    #[inline(always)]
    pub fn acknowledge(self) -> &'a mut crate::W<REG> {
        self.variant(Cpurestart::Acknowledge)
    }
}
#[doc = "N/A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unused0 {
    #[doc = "1: Clears the ctitrigout."]
    Acknowledge = 1,
}
impl From<Unused0> for bool {
    #[inline(always)]
    fn from(variant: Unused0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UNUSED0` writer - N/A"]
pub type Unused0W<'a, REG> = crate::BitWriter<'a, REG, Unused0>;
impl<'a, REG> Unused0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the ctitrigout."]
    #[inline(always)]
    pub fn acknowledge(self) -> &'a mut crate::W<REG> {
        self.variant(Unused0::Acknowledge)
    }
}
#[doc = "N/A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unused1 {
    #[doc = "1: Clears the ctitrigout."]
    Acknowledge = 1,
}
impl From<Unused1> for bool {
    #[inline(always)]
    fn from(variant: Unused1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UNUSED1` writer - N/A"]
pub type Unused1W<'a, REG> = crate::BitWriter<'a, REG, Unused1>;
impl<'a, REG> Unused1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the ctitrigout."]
    #[inline(always)]
    pub fn acknowledge(self) -> &'a mut crate::W<REG> {
        self.variant(Unused1::Acknowledge)
    }
}
#[doc = "N/A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unused2 {
    #[doc = "1: Clears the ctitrigout."]
    Acknowledge = 1,
}
impl From<Unused2> for bool {
    #[inline(always)]
    fn from(variant: Unused2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UNUSED2` writer - N/A"]
pub type Unused2W<'a, REG> = crate::BitWriter<'a, REG, Unused2>;
impl<'a, REG> Unused2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the ctitrigout."]
    #[inline(always)]
    pub fn acknowledge(self) -> &'a mut crate::W<REG> {
        self.variant(Unused2::Acknowledge)
    }
}
#[doc = "N/A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unused3 {
    #[doc = "1: Clears the ctitrigout."]
    Acknowledge = 1,
}
impl From<Unused3> for bool {
    #[inline(always)]
    fn from(variant: Unused3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UNUSED3` writer - N/A"]
pub type Unused3W<'a, REG> = crate::BitWriter<'a, REG, Unused3>;
impl<'a, REG> Unused3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the ctitrigout."]
    #[inline(always)]
    pub fn acknowledge(self) -> &'a mut crate::W<REG> {
        self.variant(Unused3::Acknowledge)
    }
}
#[doc = "N/A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unused4 {
    #[doc = "1: Clears the ctitrigout."]
    Acknowledge = 1,
}
impl From<Unused4> for bool {
    #[inline(always)]
    fn from(variant: Unused4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UNUSED4` writer - N/A"]
pub type Unused4W<'a, REG> = crate::BitWriter<'a, REG, Unused4>;
impl<'a, REG> Unused4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the ctitrigout."]
    #[inline(always)]
    pub fn acknowledge(self) -> &'a mut crate::W<REG> {
        self.variant(Unused4::Acknowledge)
    }
}
#[doc = "N/A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unused5 {
    #[doc = "1: Clears the ctitrigout."]
    Acknowledge = 1,
}
impl From<Unused5> for bool {
    #[inline(always)]
    fn from(variant: Unused5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UNUSED5` writer - N/A"]
pub type Unused5W<'a, REG> = crate::BitWriter<'a, REG, Unused5>;
impl<'a, REG> Unused5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the ctitrigout."]
    #[inline(always)]
    pub fn acknowledge(self) -> &'a mut crate::W<REG> {
        self.variant(Unused5::Acknowledge)
    }
}
impl W {
    #[doc = "Bit 0 - Processor debug request"]
    #[inline(always)]
    pub fn debugreq(&mut self) -> DebugreqW<'_, CtiintackSpec> {
        DebugreqW::new(self, 0)
    }
    #[doc = "Bit 1 - Processor Restart"]
    #[inline(always)]
    pub fn cpurestart(&mut self) -> CpurestartW<'_, CtiintackSpec> {
        CpurestartW::new(self, 1)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    pub fn unused0(&mut self) -> Unused0W<'_, CtiintackSpec> {
        Unused0W::new(self, 2)
    }
    #[doc = "Bit 3 - N/A"]
    #[inline(always)]
    pub fn unused1(&mut self) -> Unused1W<'_, CtiintackSpec> {
        Unused1W::new(self, 3)
    }
    #[doc = "Bit 4 - N/A"]
    #[inline(always)]
    pub fn unused2(&mut self) -> Unused2W<'_, CtiintackSpec> {
        Unused2W::new(self, 4)
    }
    #[doc = "Bit 5 - N/A"]
    #[inline(always)]
    pub fn unused3(&mut self) -> Unused3W<'_, CtiintackSpec> {
        Unused3W::new(self, 5)
    }
    #[doc = "Bit 6 - N/A"]
    #[inline(always)]
    pub fn unused4(&mut self) -> Unused4W<'_, CtiintackSpec> {
        Unused4W::new(self, 6)
    }
    #[doc = "Bit 7 - N/A"]
    #[inline(always)]
    pub fn unused5(&mut self) -> Unused5W<'_, CtiintackSpec> {
        Unused5W::new(self, 7)
    }
}
#[doc = "CTI Interrupt Acknowledge register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctiintack::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtiintackSpec;
impl crate::RegisterSpec for CtiintackSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ctiintack::W`](W) writer structure"]
impl crate::Writable for CtiintackSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTIINTACK to value 0"]
impl crate::Resettable for CtiintackSpec {}
