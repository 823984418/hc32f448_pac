#[doc = "Register `PLU3_CR` reader"]
pub type R = crate::R<Plu3CrSpec>;
#[doc = "Register `PLU3_CR` writer"]
pub type W = crate::W<Plu3CrSpec>;
#[doc = "Field `PLMODE` reader - desc PLMODE"]
pub type PlmodeR = crate::FieldReader;
#[doc = "Field `PLMODE` writer - desc PLMODE"]
pub type PlmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PLINASEL` reader - desc PLINASEL"]
pub type PlinaselR = crate::FieldReader;
#[doc = "Field `PLINASEL` writer - desc PLINASEL"]
pub type PlinaselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PLINBSEL` reader - desc PLINBSEL"]
pub type PlinbselR = crate::FieldReader;
#[doc = "Field `PLINBSEL` writer - desc PLINBSEL"]
pub type PlinbselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PLINCSEL` reader - desc PLINCSEL"]
pub type PlincselR = crate::FieldReader;
#[doc = "Field `PLINCSEL` writer - desc PLINCSEL"]
pub type PlincselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PLINDSEL` reader - desc PLINDSEL"]
pub type PlindselR = crate::FieldReader;
#[doc = "Field `PLINDSEL` writer - desc PLINDSEL"]
pub type PlindselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - desc PLMODE"]
    #[inline(always)]
    pub fn plmode(&self) -> PlmodeR {
        PlmodeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - desc PLINASEL"]
    #[inline(always)]
    pub fn plinasel(&self) -> PlinaselR {
        PlinaselR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - desc PLINBSEL"]
    #[inline(always)]
    pub fn plinbsel(&self) -> PlinbselR {
        PlinbselR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - desc PLINCSEL"]
    #[inline(always)]
    pub fn plincsel(&self) -> PlincselR {
        PlincselR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - desc PLINDSEL"]
    #[inline(always)]
    pub fn plindsel(&self) -> PlindselR {
        PlindselR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLU3_CR")
            .field("plmode", &self.plmode())
            .field("plinasel", &self.plinasel())
            .field("plinbsel", &self.plinbsel())
            .field("plincsel", &self.plincsel())
            .field("plindsel", &self.plindsel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - desc PLMODE"]
    #[inline(always)]
    pub fn plmode(&mut self) -> PlmodeW<'_, Plu3CrSpec> {
        PlmodeW::new(self, 0)
    }
    #[doc = "Bits 8:9 - desc PLINASEL"]
    #[inline(always)]
    pub fn plinasel(&mut self) -> PlinaselW<'_, Plu3CrSpec> {
        PlinaselW::new(self, 8)
    }
    #[doc = "Bits 10:11 - desc PLINBSEL"]
    #[inline(always)]
    pub fn plinbsel(&mut self) -> PlinbselW<'_, Plu3CrSpec> {
        PlinbselW::new(self, 10)
    }
    #[doc = "Bits 12:13 - desc PLINCSEL"]
    #[inline(always)]
    pub fn plincsel(&mut self) -> PlincselW<'_, Plu3CrSpec> {
        PlincselW::new(self, 12)
    }
    #[doc = "Bits 14:15 - desc PLINDSEL"]
    #[inline(always)]
    pub fn plindsel(&mut self) -> PlindselW<'_, Plu3CrSpec> {
        PlindselW::new(self, 14)
    }
}
#[doc = "desc PLU3_CR\n\nYou can [`read`](crate::Reg::read) this register and get [`plu3_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plu3_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Plu3CrSpec;
impl crate::RegisterSpec for Plu3CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`plu3_cr::R`](R) reader structure"]
impl crate::Readable for Plu3CrSpec {}
#[doc = "`write(|w| ..)` method takes [`plu3_cr::W`](W) writer structure"]
impl crate::Writable for Plu3CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLU3_CR to value 0"]
impl crate::Resettable for Plu3CrSpec {}
