 
        use super::BarcodeProperties;
        use super::Barcode1D;
        
        pub struct Code39 {
            pub barcode_properties: BarcodeProperties,
            pub code39_vec: Vec<char>,
        }
                            
        impl Code39{

            pub fn new(input: &String, check_digit:i32) -> Code39
            {
                Code39{
                    barcode_properties: BarcodeProperties {
                        input: input.clone(),
                        check_digit:check_digit,
                        human_text: "".to_string(),                
                    },
                    code39_vec : vec!['0','1','2','3','4','5','6','7','8','9',
                    'A','B','C','D','E','F','G','H','I','J',
                    'K','L','M','N','O','P','Q','R','S','T',
                    'U','V','W','X','Y','Z','-','.',' ','$',    
                    '/','+','%'],
                }
            }
    
            fn get_code39_value(&self, input_char:char) -> i32 {
                for x in 0..43
                {
                    if self.code39_vec[x] == input_char
                    {
                        return x as i32;
                    }
                }
    
                return -1;
            }
    
            fn get_code39_character(&self, input_decimal:i32) -> char
            {
                return self.code39_vec[input_decimal as usize];
            }
            
            fn generate_check_digit(&self, data:&String) -> String
            {
    
                let mut sum:i32 = 0;
                let result:i32;
                let mut str_result:String = "".to_string();
                for c in data.chars() { 
                    sum = sum + self.get_code39_value(c);
                }
    
                result = sum % 43;
                str_result.push(self.get_code39_character(result));
    
                return str_result;
    
            }
    
            fn filter_input(&self) -> String
            {
                let mut result = "".to_string();
                for c in self.barcode_properties.input.chars() { 
                    if self.get_code39_value(c) != -1
                    {
                        result.push(c);
                    }
    
                } 

                result.clone()
    
            }
    
        }
        impl Barcode1D for Code39 
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
                result.push('*');
                result.push_str(&filtered_data);
                result.push_str(&cd);            
                result.push('*');
                self.barcode_properties.human_text = result.clone();            
                return result;
            }
    
        }
        
    
    


 

