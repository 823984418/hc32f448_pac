#[doc = "Register `SRAM0_EIBIT1` reader"]
pub type R = crate::R<Sram0Eibit1Spec>;
#[doc = "Register `SRAM0_EIBIT1` writer"]
pub type W = crate::W<Sram0Eibit1Spec>;
#[doc = "Field `EIBIT` reader - desc EIBIT"]
pub type EibitR = crate::FieldReader;
#[doc = "Field `EIBIT` writer - desc EIBIT"]
pub type EibitW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - desc EIBIT"]
    #[inline(always)]
    pub fn eibit(&self) -> EibitR {
        EibitR::new((self.bits & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRAM0_EIBIT1")
            .field("eibit", &self.eibit())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:6 - desc EIBIT"]
    #[inline(always)]
    pub fn eibit(&mut self) -> EibitW<'_, Sram0Eibit1Spec> {
        EibitW::new(self, 0)
    }
}
#[doc = "desc SRAM0_EIBIT1\n\nYou can [`read`](crate::Reg::read) this register and get [`sram0_eibit1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram0_eibit1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sram0Eibit1Spec;
impl crate::RegisterSpec for Sram0Eibit1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sram0_eibit1::R`](R) reader structure"]
impl crate::Readable for Sram0Eibit1Spec {}
#[doc = "`write(|w| ..)` method takes [`sram0_eibit1::W`](W) writer structure"]
impl crate::Writable for Sram0Eibit1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SRAM0_EIBIT1 to value 0"]
impl crate::Resettable for Sram0Eibit1Spec {}
