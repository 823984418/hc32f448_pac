#[doc = "Register `CMPAR4` reader"]
pub type R = crate::R<Cmpar4Spec>;
#[doc = "Register `CMPAR4` writer"]
pub type W = crate::W<Cmpar4Spec>;
#[doc = "Field `CMP` reader - desc CMP"]
pub type CmpR = crate::FieldReader<u32>;
#[doc = "Field `CMP` writer - desc CMP"]
pub type CmpW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - desc CMP"]
    #[inline(always)]
    pub fn cmp(&self) -> CmpR {
        CmpR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMPAR4").field("cmp", &self.cmp()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - desc CMP"]
    #[inline(always)]
    pub fn cmp(&mut self) -> CmpW<'_, Cmpar4Spec> {
        CmpW::new(self, 0)
    }
}
#[doc = "desc CMPAR4\n\nYou can [`read`](crate::Reg::read) this register and get [`cmpar4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpar4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cmpar4Spec;
impl crate::RegisterSpec for Cmpar4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmpar4::R`](R) reader structure"]
impl crate::Readable for Cmpar4Spec {}
#[doc = "`write(|w| ..)` method takes [`cmpar4::W`](W) writer structure"]
impl crate::Writable for Cmpar4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMPAR4 to value 0xffff_ffff"]
impl crate::Resettable for Cmpar4Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
