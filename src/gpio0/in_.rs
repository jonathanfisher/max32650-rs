#[doc = "Reader of register IN"]
pub type R = crate::R<u32, super::IN>;
#[doc = "Reader of field `GPIO_IN`"]
pub type GPIO_IN_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_in(&self) -> GPIO_IN_R {
        GPIO_IN_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
