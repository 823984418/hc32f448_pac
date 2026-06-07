#[doc = "Register `ADC3_TRGSEL0` reader"]
pub type R = crate::R<Adc3Trgsel0Spec>;
#[doc = "Register `ADC3_TRGSEL0` writer"]
pub type W = crate::W<Adc3Trgsel0Spec>;
#[doc = "Field `TRGSEL` reader - desc TRGSEL"]
pub type TrgselR = crate::FieldReader<u16>;
#[doc = "Field `TRGSEL` writer - desc TRGSEL"]
pub type TrgselW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `PLCHSEL` reader - desc PLCHSEL"]
pub type PlchselR = crate::FieldReader;
#[doc = "Field `PLCHSEL` writer - desc PLCHSEL"]
pub type PlchselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `COMEN` reader - desc COMEN"]
pub type ComenR = crate::FieldReader;
#[doc = "Field `COMEN` writer - desc COMEN"]
pub type ComenW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:8 - desc TRGSEL"]
    #[inline(always)]
    pub fn trgsel(&self) -> TrgselR {
        TrgselR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:18 - desc PLCHSEL"]
    #[inline(always)]
    pub fn plchsel(&self) -> PlchselR {
        PlchselR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 30:31 - desc COMEN"]
    #[inline(always)]
    pub fn comen(&self) -> ComenR {
        ComenR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC3_TRGSEL0")
            .field("trgsel", &self.trgsel())
            .field("plchsel", &self.plchsel())
            .field("comen", &self.comen())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:8 - desc TRGSEL"]
    #[inline(always)]
    pub fn trgsel(&mut self) -> TrgselW<'_, Adc3Trgsel0Spec> {
        TrgselW::new(self, 0)
    }
    #[doc = "Bits 16:18 - desc PLCHSEL"]
    #[inline(always)]
    pub fn plchsel(&mut self) -> PlchselW<'_, Adc3Trgsel0Spec> {
        PlchselW::new(self, 16)
    }
    #[doc = "Bits 30:31 - desc COMEN"]
    #[inline(always)]
    pub fn comen(&mut self) -> ComenW<'_, Adc3Trgsel0Spec> {
        ComenW::new(self, 30)
    }
}
#[doc = "desc ADC3_TRGSEL0\n\nYou can [`read`](crate::Reg::read) this register and get [`adc3_trgsel0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc3_trgsel0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc3Trgsel0Spec;
impl crate::RegisterSpec for Adc3Trgsel0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc3_trgsel0::R`](R) reader structure"]
impl crate::Readable for Adc3Trgsel0Spec {}
#[doc = "`write(|w| ..)` method takes [`adc3_trgsel0::W`](W) writer structure"]
impl crate::Writable for Adc3Trgsel0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADC3_TRGSEL0 to value 0x01ff"]
impl crate::Resettable for Adc3Trgsel0Spec {
    const RESET_VALUE: u32 = 0x01ff;
}
