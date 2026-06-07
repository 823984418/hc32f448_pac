#[doc = "Register `INTSEL6` reader"]
pub type R = crate::R<Intsel6Spec>;
#[doc = "Register `INTSEL6` writer"]
pub type W = crate::W<Intsel6Spec>;
#[doc = "Field `INTSEL` reader - desc INTSEL"]
pub type IntselR = crate::FieldReader<u16>;
#[doc = "Field `INTSEL` writer - desc INTSEL"]
pub type IntselW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - desc INTSEL"]
    #[inline(always)]
    pub fn intsel(&self) -> IntselR {
        IntselR::new((self.bits & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTSEL6")
            .field("intsel", &self.intsel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:8 - desc INTSEL"]
    #[inline(always)]
    pub fn intsel(&mut self) -> IntselW<'_, Intsel6Spec> {
        IntselW::new(self, 0)
    }
}
#[doc = "desc INTSEL6\n\nYou can [`read`](crate::Reg::read) this register and get [`intsel6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intsel6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Intsel6Spec;
impl crate::RegisterSpec for Intsel6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intsel6::R`](R) reader structure"]
impl crate::Readable for Intsel6Spec {}
#[doc = "`write(|w| ..)` method takes [`intsel6::W`](W) writer structure"]
impl crate::Writable for Intsel6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTSEL6 to value 0x01ff"]
impl crate::Resettable for Intsel6Spec {
    const RESET_VALUE: u32 = 0x01ff;
}
