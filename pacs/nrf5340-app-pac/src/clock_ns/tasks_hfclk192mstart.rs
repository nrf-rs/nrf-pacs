#[doc = "Register `TASKS_HFCLK192MSTART` writer"]
pub type W = crate::W<TasksHfclk192mstartSpec>;
#[doc = "Start HFCLK192M source as selected in HFCLK192MSRC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksHfclk192mstart {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksHfclk192mstart> for bool {
    #[inline(always)]
    fn from(variant: TasksHfclk192mstart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_HFCLK192MSTART` writer - Start HFCLK192M source as selected in HFCLK192MSRC"]
pub type TasksHfclk192mstartW<'a, REG> = crate::BitWriter<'a, REG, TasksHfclk192mstart>;
impl<'a, REG> TasksHfclk192mstartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksHfclk192mstart::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Start HFCLK192M source as selected in HFCLK192MSRC"]
    #[inline(always)]
    pub fn tasks_hfclk192mstart(&mut self) -> TasksHfclk192mstartW<'_, TasksHfclk192mstartSpec> {
        TasksHfclk192mstartW::new(self, 0)
    }
}
#[doc = "Start HFCLK192M source as selected in HFCLK192MSRC\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_hfclk192mstart::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksHfclk192mstartSpec;
impl crate::RegisterSpec for TasksHfclk192mstartSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_hfclk192mstart::W`](W) writer structure"]
impl crate::Writable for TasksHfclk192mstartSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_HFCLK192MSTART to value 0"]
impl crate::Resettable for TasksHfclk192mstartSpec {}
