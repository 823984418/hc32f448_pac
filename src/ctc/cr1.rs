#[doc = "Register `CR1` reader"]
pub type R = crate::R<Cr1Spec>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<Cr1Spec>;
#[doc = "Field `REFPSC` reader - desc REFPSC"]
pub type RefpscR = crate::FieldReader;
#[doc = "Field `REFPSC` writer - desc REFPSC"]
pub type RefpscW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `REFCKS` reader - desc REFCKS"]
pub type RefcksR = crate::FieldReader;
#[doc = "Field `REFCKS` writer - desc REFCKS"]
pub type RefcksW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ERRIE` reader - desc ERRIE"]
pub type ErrieR = crate::BitReader;
#[doc = "Field `ERRIE` writer - desc ERRIE"]
pub type ErrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCEN` reader - desc CTCEN"]
pub type CtcenR = crate::BitReader;
#[doc = "Field `CTCEN` writer - desc CTCEN"]
pub type CtcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRCPSC` reader - desc HRCPSC"]
pub type HrcpscR = crate::FieldReader;
#[doc = "Field `HRCPSC` writer - desc HRCPSC"]
pub type HrcpscW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `REFEDG` reader - desc REFEDG"]
pub type RefedgR = crate::FieldReader;
#[doc = "Field `REFEDG` writer - desc REFEDG"]
pub type RefedgW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TRMVAL` reader - desc TRMVAL"]
pub type TrmvalR = crate::FieldReader;
#[doc = "Field `TRMVAL` writer - desc TRMVAL"]
pub type TrmvalW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:2 - desc REFPSC"]
    #[inline(always)]
    pub fn refpsc(&self) -> RefpscR {
        RefpscR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:5 - desc REFCKS"]
    #[inline(always)]
    pub fn refcks(&self) -> RefcksR {
        RefcksR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - desc ERRIE"]
    #[inline(always)]
    pub fn errie(&self) -> ErrieR {
        ErrieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc CTCEN"]
    #[inline(always)]
    pub fn ctcen(&self) -> CtcenR {
        CtcenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - desc HRCPSC"]
    #[inline(always)]
    pub fn hrcpsc(&self) -> HrcpscR {
        HrcpscR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:13 - desc REFEDG"]
    #[inline(always)]
    pub fn refedg(&self) -> RefedgR {
        RefedgR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:21 - desc TRMVAL"]
    #[inline(always)]
    pub fn trmval(&self) -> TrmvalR {
        TrmvalR::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR1")
            .field("refpsc", &self.refpsc())
            .field("refcks", &self.refcks())
            .field("errie", &self.errie())
            .field("ctcen", &self.ctcen())
            .field("hrcpsc", &self.hrcpsc())
            .field("refedg", &self.refedg())
            .field("trmval", &self.trmval())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - desc REFPSC"]
    #[inline(always)]
    pub fn refpsc(&mut self) -> RefpscW<'_, Cr1Spec> {
        RefpscW::new(self, 0)
    }
    #[doc = "Bits 4:5 - desc REFCKS"]
    #[inline(always)]
    pub fn refcks(&mut self) -> RefcksW<'_, Cr1Spec> {
        RefcksW::new(self, 4)
    }
    #[doc = "Bit 6 - desc ERRIE"]
    #[inline(always)]
    pub fn errie(&mut self) -> ErrieW<'_, Cr1Spec> {
        ErrieW::new(self, 6)
    }
    #[doc = "Bit 7 - desc CTCEN"]
    #[inline(always)]
    pub fn ctcen(&mut self) -> CtcenW<'_, Cr1Spec> {
        CtcenW::new(self, 7)
    }
    #[doc = "Bits 8:10 - desc HRCPSC"]
    #[inline(always)]
    pub fn hrcpsc(&mut self) -> HrcpscW<'_, Cr1Spec> {
        HrcpscW::new(self, 8)
    }
    #[doc = "Bits 12:13 - desc REFEDG"]
    #[inline(always)]
    pub fn refedg(&mut self) -> RefedgW<'_, Cr1Spec> {
        RefedgW::new(self, 12)
    }
    #[doc = "Bits 16:21 - desc TRMVAL"]
    #[inline(always)]
    pub fn trmval(&mut self) -> TrmvalW<'_, Cr1Spec> {
        TrmvalW::new(self, 16)
    }
}
#[doc = "desc CR1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr1Spec;
impl crate::RegisterSpec for Cr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for Cr1Spec {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for Cr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for Cr1Spec {}
