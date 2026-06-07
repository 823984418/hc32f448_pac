#[doc = "Register `EIRQCR1` reader"]
pub type R = crate::R<Eirqcr1Spec>;
#[doc = "Register `EIRQCR1` writer"]
pub type W = crate::W<Eirqcr1Spec>;
#[doc = "Field `EIRQTRG` reader - desc EIRQTRG"]
pub type EirqtrgR = crate::FieldReader;
#[doc = "Field `EIRQTRG` writer - desc EIRQTRG"]
pub type EirqtrgW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EISMPCLK` reader - desc EISMPCLK"]
pub type EismpclkR = crate::FieldReader;
#[doc = "Field `EISMPCLK` writer - desc EISMPCLK"]
pub type EismpclkW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EFEN` reader - desc EFEN"]
pub type EfenR = crate::BitReader;
#[doc = "Field `EFEN` writer - desc EFEN"]
pub type EfenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOCSEL` reader - desc NOCSEL"]
pub type NocselR = crate::FieldReader;
#[doc = "Field `NOCSEL` writer - desc NOCSEL"]
pub type NocselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NOCEN` reader - desc NOCEN"]
pub type NocenR = crate::BitReader;
#[doc = "Field `NOCEN` writer - desc NOCEN"]
pub type NocenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - desc EIRQTRG"]
    #[inline(always)]
    pub fn eirqtrg(&self) -> EirqtrgR {
        EirqtrgR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - desc EISMPCLK"]
    #[inline(always)]
    pub fn eismpclk(&self) -> EismpclkR {
        EismpclkR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - desc EFEN"]
    #[inline(always)]
    pub fn efen(&self) -> EfenR {
        EfenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 12:13 - desc NOCSEL"]
    #[inline(always)]
    pub fn nocsel(&self) -> NocselR {
        NocselR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 15 - desc NOCEN"]
    #[inline(always)]
    pub fn nocen(&self) -> NocenR {
        NocenR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EIRQCR1")
            .field("eirqtrg", &self.eirqtrg())
            .field("eismpclk", &self.eismpclk())
            .field("efen", &self.efen())
            .field("nocsel", &self.nocsel())
            .field("nocen", &self.nocen())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - desc EIRQTRG"]
    #[inline(always)]
    pub fn eirqtrg(&mut self) -> EirqtrgW<'_, Eirqcr1Spec> {
        EirqtrgW::new(self, 0)
    }
    #[doc = "Bits 4:5 - desc EISMPCLK"]
    #[inline(always)]
    pub fn eismpclk(&mut self) -> EismpclkW<'_, Eirqcr1Spec> {
        EismpclkW::new(self, 4)
    }
    #[doc = "Bit 7 - desc EFEN"]
    #[inline(always)]
    pub fn efen(&mut self) -> EfenW<'_, Eirqcr1Spec> {
        EfenW::new(self, 7)
    }
    #[doc = "Bits 12:13 - desc NOCSEL"]
    #[inline(always)]
    pub fn nocsel(&mut self) -> NocselW<'_, Eirqcr1Spec> {
        NocselW::new(self, 12)
    }
    #[doc = "Bit 15 - desc NOCEN"]
    #[inline(always)]
    pub fn nocen(&mut self) -> NocenW<'_, Eirqcr1Spec> {
        NocenW::new(self, 15)
    }
}
#[doc = "desc EIRQCR1\n\nYou can [`read`](crate::Reg::read) this register and get [`eirqcr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eirqcr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Eirqcr1Spec;
impl crate::RegisterSpec for Eirqcr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eirqcr1::R`](R) reader structure"]
impl crate::Readable for Eirqcr1Spec {}
#[doc = "`write(|w| ..)` method takes [`eirqcr1::W`](W) writer structure"]
impl crate::Writable for Eirqcr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EIRQCR1 to value 0"]
impl crate::Resettable for Eirqcr1Spec {}
