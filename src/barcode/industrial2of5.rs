 
        use super::BarcodeProperties;
        use super::Barcode1D;
        
        pub struct Industrial2of5 {
            pub barcode_properties: BarcodeProperties,
        }
                            
        impl Industrial2of5{

            pub fn new(input: &String, check_digit:i32) -> Industrial2of5
            {
                Industrial2of5{
                    barcode_properties: BarcodeProperties {
                        input: input.clone(),
                        check_digit:check_digit,
                        human_text: "".to_string(),                
                    },
                }
            }
            fn generate_check_digit(&self, data:&String) -> String
            {
    
                let mut sum:i32 = 0;
                let mut str_result:String = "".to_string();
                let mut toggle = 1;
                for c in data.chars().rev() { 
                    let barcode_value:i32 = c as i32 - 48; 
                    if toggle == 1
                    {
                        sum += barcode_value *3;
                        toggle=0;
                    }
                    else
                    {
                        sum += barcode_value;
                        toggle = 1;    
                    }
                }
                if sum % 10 == 0
                {
                    str_result = "0".to_string();
                }
                else
                {
                    let cd_val = 10 - (sum % 10) + '0' as i32;
                    let c:char = cd_val as u8 as char;
                    str_result = c.to_string();

                }
    
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
        impl Barcode1D for Industrial2of5 
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
                if self.barcode_properties.check_digit == 1
                {
                    if filtered_length > 254
                    {
                        filtered_data.truncate(254);
                    }
                    cd = self.generate_check_digit(&filtered_data);
    
                }
                else
                {
                    if filtered_length > 255
                    {
                        filtered_data.truncate(255);
                    }
                }
                result.push('{');
                result.push_str(&filtered_data);
                result.push_str(&cd);            
                result.push('}');
                self.barcode_properties.human_text = result.clone();            
                return result;
            }
    
        }
        
    
    


 

