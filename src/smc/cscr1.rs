#[doc = "Register `CSCR1` reader"]
pub type R = crate::R<Cscr1Spec>;
#[doc = "Register `CSCR1` writer"]
pub type W = crate::W<Cscr1Spec>;
#[doc = "Field `ADDMAT0` reader - desc ADDMAT0"]
pub type Addmat0R = crate::FieldReader;
#[doc = "Field `ADDMAT0` writer - desc ADDMAT0"]
pub type Addmat0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - desc ADDMAT0"]
    #[inline(always)]
    pub fn addmat0(&self) -> Addmat0R {
        Addmat0R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSCR1")
            .field("addmat0", &self.addmat0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - desc ADDMAT0"]
    #[inline(always)]
    pub fn addmat0(&mut self) -> Addmat0W<'_, Cscr1Spec> {
        Addmat0W::new(self, 0)
    }
}
#[doc = "desc CSCR1\n\nYou can [`read`](crate::Reg::read) this register and get [`cscr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cscr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cscr1Spec;
impl crate::RegisterSpec for Cscr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cscr1::R`](R) reader structure"]
impl crate::Readable for Cscr1Spec {}
#[doc = "`write(|w| ..)` method takes [`cscr1::W`](W) writer structure"]
impl crate::Writable for Cscr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSCR1 to value 0"]
impl crate::Resettable for Cscr1Spec {}
