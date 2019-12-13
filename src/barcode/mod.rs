    pub struct BarcodeProperties {

        pub input: String,
        pub check_digit: i32,
        pub human_text: String,

    }
    
    pub trait Barcode {
         //fn encode(&mut self, input: &String, check_digit: i32) -> String;
         fn encode(&mut self) -> String;
         fn get_human_text(&self) -> String;        
    }
    
    pub trait Barcode1D {
        //fn encode_1d(&mut self, input: &String, check_digit: i32) -> String;
        fn encode_1d(&mut self) -> String;
        fn get_human_text_1d(&self) -> String;        
    }
   
    impl<T> Barcode for T where T: Barcode1D {
/*
        fn encode(&mut self, input: &String, check_digit: i32) -> String
        {
            self.encode_1d(&input,check_digit)
        }
*/
        fn encode(&mut self) -> String
        {
            self.encode_1d()
        }
        fn get_human_text(&self) -> String
        {
            self.get_human_text_1d()
        }        
    }

    pub mod code39;
 
    pub mod industrial2of5;

    pub mod postnet;

 

