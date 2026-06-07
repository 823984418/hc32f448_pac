#[doc = "Register `AWDCR` reader"]
pub type R = crate::R<AwdcrSpec>;
#[doc = "Register `AWDCR` writer"]
pub type W = crate::W<AwdcrSpec>;
#[doc = "Field `AWD0EN` reader - desc AWD0EN"]
pub type Awd0enR = crate::BitReader;
#[doc = "Field `AWD0EN` writer - desc AWD0EN"]
pub type Awd0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWD0IEN` reader - desc AWD0IEN"]
pub type Awd0ienR = crate::BitReader;
#[doc = "Field `AWD0IEN` writer - desc AWD0IEN"]
pub type Awd0ienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWD0MD` reader - desc AWD0MD"]
pub type Awd0mdR = crate::BitReader;
#[doc = "Field `AWD0MD` writer - desc AWD0MD"]
pub type Awd0mdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWD1EN` reader - desc AWD1EN"]
pub type Awd1enR = crate::BitReader;
#[doc = "Field `AWD1EN` writer - desc AWD1EN"]
pub type Awd1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWD1IEN` reader - desc AWD1IEN"]
pub type Awd1ienR = crate::BitReader;
#[doc = "Field `AWD1IEN` writer - desc AWD1IEN"]
pub type Awd1ienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWD1MD` reader - desc AWD1MD"]
pub type Awd1mdR = crate::BitReader;
#[doc = "Field `AWD1MD` writer - desc AWD1MD"]
pub type Awd1mdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWDCM` reader - desc AWDCM"]
pub type AwdcmR = crate::FieldReader;
#[doc = "Field `AWDCM` writer - desc AWDCM"]
pub type AwdcmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - desc AWD0EN"]
    #[inline(always)]
    pub fn awd0en(&self) -> Awd0enR {
        Awd0enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc AWD0IEN"]
    #[inline(always)]
    pub fn awd0ien(&self) -> Awd0ienR {
        Awd0ienR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc AWD0MD"]
    #[inline(always)]
    pub fn awd0md(&self) -> Awd0mdR {
        Awd0mdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - desc AWD1EN"]
    #[inline(always)]
    pub fn awd1en(&self) -> Awd1enR {
        Awd1enR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc AWD1IEN"]
    #[inline(always)]
    pub fn awd1ien(&self) -> Awd1ienR {
        Awd1ienR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc AWD1MD"]
    #[inline(always)]
    pub fn awd1md(&self) -> Awd1mdR {
        Awd1mdR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:9 - desc AWDCM"]
    #[inline(always)]
    pub fn awdcm(&self) -> AwdcmR {
        AwdcmR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AWDCR")
            .field("awd0en", &self.awd0en())
            .field("awd0ien", &self.awd0ien())
            .field("awd0md", &self.awd0md())
            .field("awd1en", &self.awd1en())
            .field("awd1ien", &self.awd1ien())
            .field("awd1md", &self.awd1md())
            .field("awdcm", &self.awdcm())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - desc AWD0EN"]
    #[inline(always)]
    pub fn awd0en(&mut self) -> Awd0enW<'_, AwdcrSpec> {
        Awd0enW::new(self, 0)
    }
    #[doc = "Bit 1 - desc AWD0IEN"]
    #[inline(always)]
    pub fn awd0ien(&mut self) -> Awd0ienW<'_, AwdcrSpec> {
        Awd0ienW::new(self, 1)
    }
    #[doc = "Bit 2 - desc AWD0MD"]
    #[inline(always)]
    pub fn awd0md(&mut self) -> Awd0mdW<'_, AwdcrSpec> {
        Awd0mdW::new(self, 2)
    }
    #[doc = "Bit 4 - desc AWD1EN"]
    #[inline(always)]
    pub fn awd1en(&mut self) -> Awd1enW<'_, AwdcrSpec> {
        Awd1enW::new(self, 4)
    }
    #[doc = "Bit 5 - desc AWD1IEN"]
    #[inline(always)]
    pub fn awd1ien(&mut self) -> Awd1ienW<'_, AwdcrSpec> {
        Awd1ienW::new(self, 5)
    }
    #[doc = "Bit 6 - desc AWD1MD"]
    #[inline(always)]
    pub fn awd1md(&mut self) -> Awd1mdW<'_, AwdcrSpec> {
        Awd1mdW::new(self, 6)
    }
    #[doc = "Bits 8:9 - desc AWDCM"]
    #[inline(always)]
    pub fn awdcm(&mut self) -> AwdcmW<'_, AwdcrSpec> {
        AwdcmW::new(self, 8)
    }
}
#[doc = "desc AWDCR\n\nYou can [`read`](crate::Reg::read) this register and get [`awdcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awdcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AwdcrSpec;
impl crate::RegisterSpec for AwdcrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`awdcr::R`](R) reader structure"]
impl crate::Readable for AwdcrSpec {}
#[doc = "`write(|w| ..)` method takes [`awdcr::W`](W) writer structure"]
impl crate::Writable for AwdcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AWDCR to value 0"]
impl crate::Resettable for AwdcrSpec {}
