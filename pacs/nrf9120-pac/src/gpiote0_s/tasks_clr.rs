#[doc = "Register `TASKS_CLR[%s]` writer"]
pub type W = crate::W<TasksClrSpec>;
#[doc = "Task for writing to pin specified in CONFIG\\[n\\].PSEL. Action on pin is to set it low.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksClr {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksClr> for bool {
    #[inline(always)]
    fn from(variant: TasksClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_CLR` writer - Task for writing to pin specified in CONFIG\\[n\\].PSEL. Action on pin is to set it low."]
pub type TasksClrW<'a, REG> = crate::BitWriter<'a, REG, TasksClr>;
impl<'a, REG> TasksClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksClr::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Task for writing to pin specified in CONFIG\\[n\\].PSEL. Action on pin is to set it low."]
    #[inline(always)]
    #[must_use]
    pub fn tasks_clr(&mut self) -> TasksClrW<TasksClrSpec> {
        TasksClrW::new(self, 0)
    }
}
#[doc = "Description collection: Task for writing to pin specified in CONFIG\\[n\\].PSEL. Action on pin is to set it low.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tasks_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksClrSpec;
impl crate::RegisterSpec for TasksClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_clr::W`](W) writer structure"]
impl crate::Writable for TasksClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TASKS_CLR[%s]
to value 0"]
impl crate::Resettable for TasksClrSpec {
    const RESET_VALUE: u32 = 0;
}
