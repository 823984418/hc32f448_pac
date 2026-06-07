#[doc = "Register `STCR1` writer"]
pub type W = crate::W<Stcr1Spec>;
#[doc = "Field `LPWOR` writer - desc LPWOR"]
pub type LpworW<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<Stcr1Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 2 - desc LPWOR"]
    #[inline(always)]
    pub fn lpwor(&mut self) -> LpworW<'_, Stcr1Spec> {
        LpworW::new(self, 2)
    }
}
#[doc = "desc STCR1\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stcr1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Stcr1Spec;
impl crate::RegisterSpec for Stcr1Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`stcr1::W`](W) writer structure"]
impl crate::Writable for Stcr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STCR1 to value 0"]
impl crate::Resettable for Stcr1Spec {}
