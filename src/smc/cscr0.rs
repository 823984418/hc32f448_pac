#[doc = "Register `CSCR0` reader"]
pub type R = crate::R<Cscr0Spec>;
#[doc = "Register `CSCR0` writer"]
pub type W = crate::W<Cscr0Spec>;
#[doc = "Field `ADDMSK0` reader - desc ADDMSK0"]
pub type Addmsk0R = crate::FieldReader;
#[doc = "Field `ADDMSK0` writer - desc ADDMSK0"]
pub type Addmsk0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - desc ADDMSK0"]
    #[inline(always)]
    pub fn addmsk0(&self) -> Addmsk0R {
        Addmsk0R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSCR0")
            .field("addmsk0", &self.addmsk0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - desc ADDMSK0"]
    #[inline(always)]
    pub fn addmsk0(&mut self) -> Addmsk0W<'_, Cscr0Spec> {
        Addmsk0W::new(self, 0)
    }
}
#[doc = "desc CSCR0\n\nYou can [`read`](crate::Reg::read) this register and get [`cscr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cscr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cscr0Spec;
impl crate::RegisterSpec for Cscr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cscr0::R`](R) reader structure"]
impl crate::Readable for Cscr0Spec {}
#[doc = "`write(|w| ..)` method takes [`cscr0::W`](W) writer structure"]
impl crate::Writable for Cscr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSCR0 to value 0xffff_ffff"]
impl crate::Resettable for Cscr0Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
