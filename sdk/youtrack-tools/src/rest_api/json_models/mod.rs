pub mod issue;
pub mod user;
pub mod project;

mod active_exp {
    use std::ops::{Deref, DerefMut};
    use std::sync::{Mutex, Arc};
    use std::rc::Rc;
    use std::cell::RefCell;
    use std::borrow::BorrowMut;

    #[derive(Clone, Debug, Default)]
    pub struct UserDto {
        pub name: String,
        pub age: u8,
        pub id: u16,
    }

    #[derive(Debug)]
    pub struct Wrapper<DTO> {
        origin_dto: Rc<DTO>,
        inner_dto: Rc<DTO>,
    }

    impl<DTO> Wrapper<DTO> {
        pub fn new(origin: DTO) -> Self {
            let origin = Rc::new(origin);
            let rc = Rc::clone(&origin);
            Wrapper { inner_dto: rc, origin_dto: origin }
        }

        pub fn save(&mut self) {
            self.origin_dto = self.inner_dto.clone()
        }
    }


    impl<DTO> DerefMut for Wrapper<DTO>
        where DTO: Clone {
        fn deref_mut(&mut self) -> &mut Self::Target {
            Rc::make_mut(&mut self.inner_dto)
        }
    }

    impl<DTO> Deref for Wrapper<DTO> {
        type Target = DTO;

        fn deref(&self) -> &Self::Target {
            &*self.origin_dto
        }
    }
}

#[cfg(test)]
mod rr {
    mod rr1 {
        use std::sync::Arc;
        use std::ops;
        use std::ops::Add;

        trait Representation {
            fn print();
        }

        struct CustomType {
            pub name: String,
            pub age: u8,
        }

        impl CustomType {
            pub fn hello(&self) {
                println!("Hello {:?}", self.name)
            }
        }


        impl ops::Add<CustomType> for CustomType {
            type Output = CustomType;

            fn add(self, rhs: CustomType) -> Self::Output {
                let age = self.age + rhs.age;
                // let name = concat!(self.name, rhs.name);
                let name = {
                    let mut result = "".to_string();
                    result.push_str(self.name.as_str());
                    result.push_str(" ");
                    result.push_str(rhs.name.as_str());
                    result
                };
                CustomType { name, age }
            }
        }

        #[test]
        #[ignore]
        fn some() {
            let sergey = CustomType { name: "Sergey".to_string(), age: 10 };
            let murzin = CustomType { name: "Murzin".to_string(), age: 18 };
            let sergey_arc = Arc::new(sergey);
            let murzin_arc = Arc::new(murzin);

            // let res = sergey_arc.clone() + murzin_arc.clone();
        }
    }

    mod rr2 {
        use crate::rest_api::json_models::active_exp::{UserDto, Wrapper};
        use std::ops::Deref;

        #[test]
        fn test2() {
            let dto = UserDto { name: "Alesha".into(), age: 10, id: 1 };
            let mut wrapper = Wrapper::new(dto);
            
            wrapper.age = 100;
            println!("read age: {:?}", wrapper.age);
            println!("lol #1 {:?}", wrapper);
            wrapper.save();
            wrapper.age = 101;
            println!("read age: {:?}", wrapper.age);
            println!("lol #2 {:?}", wrapper);
        }
    }
}