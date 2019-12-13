 
        use super::BarcodeProperties;
        use super::Barcode1D;
        use std::char;
        
        pub struct POSTNET {
            pub barcode_properties: BarcodeProperties,
        }
                            
        impl POSTNET{

            pub fn new(input: &String) -> POSTNET
            {
                POSTNET{
                    barcode_properties: BarcodeProperties {
                        input: input.clone(),
                        check_digit:0,
                        human_text: "".to_string(),                
                    },
                }
            }
    
            fn get_postnet_value(&self, input_char:char) -> i32 {
                return input_char as i32 - 48;
            }
    
            fn get_postnet_character(&self, input_decimal:i32) -> char
            {
                let input_u32 = input_decimal as u32 + 48;
                //println!("u32 {}",input_u32 );
                //println!("char #{}#",char::from_u32(input_u32).unwrap());
                return char::from_u32(input_u32).unwrap();
            }
            
            fn generate_check_digit(&self, data:&String) -> String
            {
    
                let mut sum:i32 = 0;
                let mut result:i32;
                let mut str_result:String = "".to_string();
                for c in data.chars() { 
                    sum = sum + self.get_postnet_value(c);
                }
    
                result = sum % 10;
                if result !=0 
                {
                    result = (10 - result);
                }
                //println!("{}",self.get_postnet_character(result));

                str_result.push(self.get_postnet_character(result));
    
                return str_result;
    
            }
    
            fn filter_input(&self) -> String
            {
                let mut result = "".to_string();
                for c in self.barcode_properties.input.chars() { 
                    if c <= '9' && c >='0'
                    {
                        result.push(c);
                    }
    
                } 
                result.clone()
    
            }
    
        }
        impl Barcode1D for POSTNET 
        {
    
            fn get_human_text_1d(&self) -> String{
                self.barcode_properties.human_text.clone()
            }
    
    
            fn encode_1d(&mut self) -> String
            {
                let mut cd="".to_string();
                let mut result="".to_string();
                let mut filtered_data=self.filter_input();
                let filtered_length = filtered_data.len();
                if filtered_length > 11
                {
                    filtered_data.truncate(11);
                }

                cd = self.generate_check_digit(&filtered_data);
                result.push('{');
                result.push_str(&filtered_data);
                result.push_str(&cd);            
                result.push('}');
                self.barcode_properties.human_text = result.clone();            
                return result;
            }
    
        }


 

