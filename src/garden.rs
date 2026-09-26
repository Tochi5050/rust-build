

pub mod vegetable;

use crate::garden::vegetable::Asparagus;

pub fn garden_veg () -> String{
  
  let asparagus: Asparagus = Asparagus{
    carrot: "carrot".to_string(),
    orange: "orange".to_string()
  };

  return asparagus.orange;
}