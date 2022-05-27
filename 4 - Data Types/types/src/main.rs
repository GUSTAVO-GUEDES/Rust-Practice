fn main() {
//Integer-----------------------------------------------------------------------------------------------\
    
    //Signed Integer
        // i8 -> Range: -2^7 to 2^7 -1 = 0 - 255
        // i16
        // i32 -> default type to integer variables
        // i64
        // i128
        let _a: i32 = 1;
    
    //Unsigned Integer -> Not allow negative numbers
        // u8 ->  Range: 0 to 2^8 -1 = -128 - 127
        // u16
        // u32
        // u64
        // u128
        let _b: u32 = 1;

//Float-----------------------------------------------------------------------------------------------
    
    //f32
    //f64 -> default type to float variables
    let _c: f32 = 1.0;

//Boolean-----------------------------------------------------------------------------------------------
    
    let _d: bool = false;

//Character-----------------------------------------------------------------------------------------------
    
    let _e: char = 'a';

//Tuple-----------------------------------------------------------------------------------------------
    
    let mut _tup: (i32, bool, char) = (1, true, 'z');
    let mut tup2: (i8, bool, char) = (1, true, 'z');

    // _tup = _tup2; -> Throws a error because the _tup uses i32 and _tup2 uses i8

    //Format tuple
        print!("{}\n", tup2.0);

    //Edit tuple
        tup2.0 = 10;
        print!("{}\n", tup2.0);

        //Or
        tup2 = (5, false, 'd');
        print!("{} {} {}\n", tup2.0, tup2.1, tup2.2);

//Array----------------------------------------------------------------------------------------------- 
   
    //Cannot initializa with empty array
    let arr: [u8; 5] = [1,2,3,4,5];

    let _f: u8 = arr[0];
}
