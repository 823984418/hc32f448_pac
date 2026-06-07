#[doc = "Register `ISCLRR` writer"]
pub type W = crate::W<IsclrrSpec>;
#[doc = "Field `CLREOCAF` writer - desc CLREOCAF"]
pub type ClreocafW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLREOCBF` writer - desc CLREOCBF"]
pub type ClreocbfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRSASTPDF` writer - desc CLRSASTPDF"]
pub type ClrsastpdfW<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<IsclrrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - desc CLREOCAF"]
    #[inline(always)]
    pub fn clreocaf(&mut self) -> ClreocafW<'_, IsclrrSpec> {
        ClreocafW::new(self, 0)
    }
    #[doc = "Bit 1 - desc CLREOCBF"]
    #[inline(always)]
    pub fn clreocbf(&mut self) -> ClreocbfW<'_, IsclrrSpec> {
        ClreocbfW::new(self, 1)
    }
    #[doc = "Bit 4 - desc CLRSASTPDF"]
    #[inline(always)]
    pub fn clrsastpdf(&mut self) -> ClrsastpdfW<'_, IsclrrSpec> {
        ClrsastpdfW::new(self, 4)
    }
}
#[doc = "desc ISCLRR\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isclrr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsclrrSpec;
impl crate::RegisterSpec for IsclrrSpec {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [`isclrr::W`](W) writer structure"]
impl crate::Writable for IsclrrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ISCLRR to value 0"]
impl crate::Resettable for IsclrrSpec {}
