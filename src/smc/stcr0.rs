#[doc = "Register `STCR0` writer"]
pub type W = crate::W<Stcr0Spec>;
#[doc = "Field `LPWIR` writer - desc LPWIR"]
pub type LpwirW<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<Stcr0Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 2 - desc LPWIR"]
    #[inline(always)]
    pub fn lpwir(&mut self) -> LpwirW<'_, Stcr0Spec> {
        LpwirW::new(self, 2)
    }
}
#[doc = "desc STCR0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stcr0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Stcr0Spec;
impl crate::RegisterSpec for Stcr0Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`stcr0::W`](W) writer structure"]
impl crate::Writable for Stcr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STCR0 to value 0"]
impl crate::Resettable for Stcr0Spec {}
