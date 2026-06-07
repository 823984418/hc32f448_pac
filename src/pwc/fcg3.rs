#[doc = "Register `FCG3` reader"]
pub type R = crate::R<Fcg3Spec>;
#[doc = "Register `FCG3` writer"]
pub type W = crate::W<Fcg3Spec>;
#[doc = "Field `ADC1` reader - desc ADC1"]
pub type Adc1R = crate::BitReader;
#[doc = "Field `ADC1` writer - desc ADC1"]
pub type Adc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC2` reader - desc ADC2"]
pub type Adc2R = crate::BitReader;
#[doc = "Field `ADC2` writer - desc ADC2"]
pub type Adc2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC3` reader - desc ADC3"]
pub type Adc3R = crate::BitReader;
#[doc = "Field `ADC3` writer - desc ADC3"]
pub type Adc3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAC` reader - desc DAC"]
pub type DacR = crate::BitReader;
#[doc = "Field `DAC` writer - desc DAC"]
pub type DacW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP12` reader - desc CMP12"]
pub type Cmp12R = crate::BitReader;
#[doc = "Field `CMP12` writer - desc CMP12"]
pub type Cmp12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP34` reader - desc CMP34"]
pub type Cmp34R = crate::BitReader;
#[doc = "Field `CMP34` writer - desc CMP34"]
pub type Cmp34W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMC` reader - desc SMC"]
pub type SmcR = crate::BitReader;
#[doc = "Field `SMC` writer - desc SMC"]
pub type SmcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1` reader - desc USART1"]
pub type Usart1R = crate::BitReader;
#[doc = "Field `USART1` writer - desc USART1"]
pub type Usart1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART2` reader - desc USART2"]
pub type Usart2R = crate::BitReader;
#[doc = "Field `USART2` writer - desc USART2"]
pub type Usart2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART3` reader - desc USART3"]
pub type Usart3R = crate::BitReader;
#[doc = "Field `USART3` writer - desc USART3"]
pub type Usart3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART4` reader - desc USART4"]
pub type Usart4R = crate::BitReader;
#[doc = "Field `USART4` writer - desc USART4"]
pub type Usart4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART5` reader - desc USART5"]
pub type Usart5R = crate::BitReader;
#[doc = "Field `USART5` writer - desc USART5"]
pub type Usart5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART6` reader - desc USART6"]
pub type Usart6R = crate::BitReader;
#[doc = "Field `USART6` writer - desc USART6"]
pub type Usart6W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc ADC1"]
    #[inline(always)]
    pub fn adc1(&self) -> Adc1R {
        Adc1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc ADC2"]
    #[inline(always)]
    pub fn adc2(&self) -> Adc2R {
        Adc2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc ADC3"]
    #[inline(always)]
    pub fn adc3(&self) -> Adc3R {
        Adc3R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - desc DAC"]
    #[inline(always)]
    pub fn dac(&self) -> DacR {
        DacR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - desc CMP12"]
    #[inline(always)]
    pub fn cmp12(&self) -> Cmp12R {
        Cmp12R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc CMP34"]
    #[inline(always)]
    pub fn cmp34(&self) -> Cmp34R {
        Cmp34R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - desc SMC"]
    #[inline(always)]
    pub fn smc(&self) -> SmcR {
        SmcR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - desc USART1"]
    #[inline(always)]
    pub fn usart1(&self) -> Usart1R {
        Usart1R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - desc USART2"]
    #[inline(always)]
    pub fn usart2(&self) -> Usart2R {
        Usart2R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - desc USART3"]
    #[inline(always)]
    pub fn usart3(&self) -> Usart3R {
        Usart3R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - desc USART4"]
    #[inline(always)]
    pub fn usart4(&self) -> Usart4R {
        Usart4R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - desc USART5"]
    #[inline(always)]
    pub fn usart5(&self) -> Usart5R {
        Usart5R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - desc USART6"]
    #[inline(always)]
    pub fn usart6(&self) -> Usart6R {
        Usart6R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCG3")
            .field("adc1", &self.adc1())
            .field("adc2", &self.adc2())
            .field("adc3", &self.adc3())
            .field("dac", &self.dac())
            .field("cmp12", &self.cmp12())
            .field("cmp34", &self.cmp34())
            .field("smc", &self.smc())
            .field("usart1", &self.usart1())
            .field("usart2", &self.usart2())
            .field("usart3", &self.usart3())
            .field("usart4", &self.usart4())
            .field("usart5", &self.usart5())
            .field("usart6", &self.usart6())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - desc ADC1"]
    #[inline(always)]
    pub fn adc1(&mut self) -> Adc1W<'_, Fcg3Spec> {
        Adc1W::new(self, 0)
    }
    #[doc = "Bit 1 - desc ADC2"]
    #[inline(always)]
    pub fn adc2(&mut self) -> Adc2W<'_, Fcg3Spec> {
        Adc2W::new(self, 1)
    }
    #[doc = "Bit 2 - desc ADC3"]
    #[inline(always)]
    pub fn adc3(&mut self) -> Adc3W<'_, Fcg3Spec> {
        Adc3W::new(self, 2)
    }
    #[doc = "Bit 4 - desc DAC"]
    #[inline(always)]
    pub fn dac(&mut self) -> DacW<'_, Fcg3Spec> {
        DacW::new(self, 4)
    }
    #[doc = "Bit 8 - desc CMP12"]
    #[inline(always)]
    pub fn cmp12(&mut self) -> Cmp12W<'_, Fcg3Spec> {
        Cmp12W::new(self, 8)
    }
    #[doc = "Bit 9 - desc CMP34"]
    #[inline(always)]
    pub fn cmp34(&mut self) -> Cmp34W<'_, Fcg3Spec> {
        Cmp34W::new(self, 9)
    }
    #[doc = "Bit 16 - desc SMC"]
    #[inline(always)]
    pub fn smc(&mut self) -> SmcW<'_, Fcg3Spec> {
        SmcW::new(self, 16)
    }
    #[doc = "Bit 20 - desc USART1"]
    #[inline(always)]
    pub fn usart1(&mut self) -> Usart1W<'_, Fcg3Spec> {
        Usart1W::new(self, 20)
    }
    #[doc = "Bit 21 - desc USART2"]
    #[inline(always)]
    pub fn usart2(&mut self) -> Usart2W<'_, Fcg3Spec> {
        Usart2W::new(self, 21)
    }
    #[doc = "Bit 22 - desc USART3"]
    #[inline(always)]
    pub fn usart3(&mut self) -> Usart3W<'_, Fcg3Spec> {
        Usart3W::new(self, 22)
    }
    #[doc = "Bit 23 - desc USART4"]
    #[inline(always)]
    pub fn usart4(&mut self) -> Usart4W<'_, Fcg3Spec> {
        Usart4W::new(self, 23)
    }
    #[doc = "Bit 24 - desc USART5"]
    #[inline(always)]
    pub fn usart5(&mut self) -> Usart5W<'_, Fcg3Spec> {
        Usart5W::new(self, 24)
    }
    #[doc = "Bit 25 - desc USART6"]
    #[inline(always)]
    pub fn usart6(&mut self) -> Usart6W<'_, Fcg3Spec> {
        Usart6W::new(self, 25)
    }
}
#[doc = "desc FCG3\n\nYou can [`read`](crate::Reg::read) this register and get [`fcg3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcg3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fcg3Spec;
impl crate::RegisterSpec for Fcg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcg3::R`](R) reader structure"]
impl crate::Readable for Fcg3Spec {}
#[doc = "`write(|w| ..)` method takes [`fcg3::W`](W) writer structure"]
impl crate::Writable for Fcg3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FCG3 to value 0xffff_ffff"]
impl crate::Resettable for Fcg3Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
