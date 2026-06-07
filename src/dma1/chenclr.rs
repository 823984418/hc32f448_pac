#[doc = "Register `CHENCLR` writer"]
pub type W = crate::W<ChenclrSpec>;
#[doc = "Field `CHENCLR` writer - desc CHENCLR"]
pub type ChenclrW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl core::fmt::Debug for crate::generic::Reg<ChenclrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:5 - desc CHENCLR"]
    #[inline(always)]
    pub fn chenclr(&mut self) -> ChenclrW<'_, ChenclrSpec> {
        ChenclrW::new(self, 0)
    }
}
#[doc = "desc CHENCLR\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chenclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChenclrSpec;
impl crate::RegisterSpec for ChenclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`chenclr::W`](W) writer structure"]
impl crate::Writable for ChenclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHENCLR to value 0"]
impl crate::Resettable for ChenclrSpec {}
