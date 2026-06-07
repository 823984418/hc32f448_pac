#[doc = "Register `INTSEL3` reader"]
pub type R = crate::R<Intsel3Spec>;
#[doc = "Register `INTSEL3` writer"]
pub type W = crate::W<Intsel3Spec>;
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
        f.debug_struct("INTSEL3")
            .field("intsel", &self.intsel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:8 - desc INTSEL"]
    #[inline(always)]
    pub fn intsel(&mut self) -> IntselW<'_, Intsel3Spec> {
        IntselW::new(self, 0)
    }
}
#[doc = "desc INTSEL3\n\nYou can [`read`](crate::Reg::read) this register and get [`intsel3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intsel3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Intsel3Spec;
impl crate::RegisterSpec for Intsel3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intsel3::R`](R) reader structure"]
impl crate::Readable for Intsel3Spec {}
#[doc = "`write(|w| ..)` method takes [`intsel3::W`](W) writer structure"]
impl crate::Writable for Intsel3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTSEL3 to value 0x01ff"]
impl crate::Resettable for Intsel3Spec {
    const RESET_VALUE: u32 = 0x01ff;
}
