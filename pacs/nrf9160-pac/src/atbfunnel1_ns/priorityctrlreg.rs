#[doc = "Register `PRIORITYCTRLREG` reader"]
pub type R = crate::R<PriorityctrlregSpec>;
#[doc = "Register `PRIORITYCTRLREG` writer"]
pub type W = crate::W<PriorityctrlregSpec>;
#[doc = "Field `PRIPORT0` reader - Priority value of port number 0."]
pub type Priport0R = crate::FieldReader;
#[doc = "Field `PRIPORT0` writer - Priority value of port number 0."]
pub type Priport0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PRIPORT1` reader - Priority value of port number 1."]
pub type Priport1R = crate::FieldReader;
#[doc = "Field `PRIPORT1` writer - Priority value of port number 1."]
pub type Priport1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PRIPORT2` reader - Priority value of port number 2."]
pub type Priport2R = crate::FieldReader;
#[doc = "Field `PRIPORT2` writer - Priority value of port number 2."]
pub type Priport2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PRIPORT3` reader - Priority value of port number 3."]
pub type Priport3R = crate::FieldReader;
#[doc = "Field `PRIPORT3` writer - Priority value of port number 3."]
pub type Priport3W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PRIPORT4` reader - Priority value of port number 4."]
pub type Priport4R = crate::FieldReader;
#[doc = "Field `PRIPORT4` writer - Priority value of port number 4."]
pub type Priport4W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PRIPORT5` reader - Priority value of port number 5."]
pub type Priport5R = crate::FieldReader;
#[doc = "Field `PRIPORT5` writer - Priority value of port number 5."]
pub type Priport5W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PRIPORT6` reader - Priority value of port number 6."]
pub type Priport6R = crate::FieldReader;
#[doc = "Field `PRIPORT6` writer - Priority value of port number 6."]
pub type Priport6W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PRIPORT7` reader - Priority value of port number 7."]
pub type Priport7R = crate::FieldReader;
#[doc = "Field `PRIPORT7` writer - Priority value of port number 7."]
pub type Priport7W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Priority value of port number 0."]
    #[inline(always)]
    pub fn priport0(&self) -> Priport0R {
        Priport0R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Priority value of port number 1."]
    #[inline(always)]
    pub fn priport1(&self) -> Priport1R {
        Priport1R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - Priority value of port number 2."]
    #[inline(always)]
    pub fn priport2(&self) -> Priport2R {
        Priport2R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - Priority value of port number 3."]
    #[inline(always)]
    pub fn priport3(&self) -> Priport3R {
        Priport3R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Priority value of port number 4."]
    #[inline(always)]
    pub fn priport4(&self) -> Priport4R {
        Priport4R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - Priority value of port number 5."]
    #[inline(always)]
    pub fn priport5(&self) -> Priport5R {
        Priport5R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - Priority value of port number 6."]
    #[inline(always)]
    pub fn priport6(&self) -> Priport6R {
        Priport6R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - Priority value of port number 7."]
    #[inline(always)]
    pub fn priport7(&self) -> Priport7R {
        Priport7R::new(((self.bits >> 21) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Priority value of port number 0."]
    #[inline(always)]
    pub fn priport0(&mut self) -> Priport0W<'_, PriorityctrlregSpec> {
        Priport0W::new(self, 0)
    }
    #[doc = "Bits 3:5 - Priority value of port number 1."]
    #[inline(always)]
    pub fn priport1(&mut self) -> Priport1W<'_, PriorityctrlregSpec> {
        Priport1W::new(self, 3)
    }
    #[doc = "Bits 6:8 - Priority value of port number 2."]
    #[inline(always)]
    pub fn priport2(&mut self) -> Priport2W<'_, PriorityctrlregSpec> {
        Priport2W::new(self, 6)
    }
    #[doc = "Bits 9:11 - Priority value of port number 3."]
    #[inline(always)]
    pub fn priport3(&mut self) -> Priport3W<'_, PriorityctrlregSpec> {
        Priport3W::new(self, 9)
    }
    #[doc = "Bits 12:14 - Priority value of port number 4."]
    #[inline(always)]
    pub fn priport4(&mut self) -> Priport4W<'_, PriorityctrlregSpec> {
        Priport4W::new(self, 12)
    }
    #[doc = "Bits 15:17 - Priority value of port number 5."]
    #[inline(always)]
    pub fn priport5(&mut self) -> Priport5W<'_, PriorityctrlregSpec> {
        Priport5W::new(self, 15)
    }
    #[doc = "Bits 18:20 - Priority value of port number 6."]
    #[inline(always)]
    pub fn priport6(&mut self) -> Priport6W<'_, PriorityctrlregSpec> {
        Priport6W::new(self, 18)
    }
    #[doc = "Bits 21:23 - Priority value of port number 7."]
    #[inline(always)]
    pub fn priport7(&mut self) -> Priport7W<'_, PriorityctrlregSpec> {
        Priport7W::new(self, 21)
    }
}
#[doc = "The Priority_Ctrl_Reg register defines the order in which inputs are selected. Each 3-bit field is a priority for each particular slave interface.\n\nYou can [`read`](crate::Reg::read) this register and get [`priorityctrlreg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priorityctrlreg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PriorityctrlregSpec;
impl crate::RegisterSpec for PriorityctrlregSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`priorityctrlreg::R`](R) reader structure"]
impl crate::Readable for PriorityctrlregSpec {}
#[doc = "`write(|w| ..)` method takes [`priorityctrlreg::W`](W) writer structure"]
impl crate::Writable for PriorityctrlregSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIORITYCTRLREG to value 0"]
impl crate::Resettable for PriorityctrlregSpec {}
