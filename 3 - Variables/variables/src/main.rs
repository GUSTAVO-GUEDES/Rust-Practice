fn main() {
//Declare-----------------------------------------------------------------------------------------------------
    //let to create a variable
    let x = 2;
//Type--------------------------------------------------------------------------------------------------------

    // ": type" in a variable to set the type 
    let y: u32 = 3;

    print!("Variable y = {}\n\n", y);

//Unused Variable---------------------------------------------------------------------------------------------

    // Use _variableName to no throw warnings in unused variables
    let _z: u32 = 4;

//Mutable-----------------------------------------------------------------------------------------------------
    
    // Variables are immutable by default
    // y = 6 --> This throws a error;

    //Use "mut" to a mutable variable
    let mut k: u32 = 5; // --> throws a warning because the value 5 is never used
    k = 6;

    print!("Variable k = {}\n\n", k);

    //Mutable Warn---------------------------------------------------------------------------------------------------------

    let mut l = 4; // --> throws a warning because the variable does not need to be mutable in this case
    print!("Variable l = {}\n\n", l);

//Redeclare--------------------------------------------------------------------------------------------------------

    //You can redeclare a variable 
    let u: u32 = 9;
    let u = "Hello";
    print!("Variable u = {}\n", u);

//Scope---------------------------------------------------------------------------------------------------------

    //Scopes !!!
    let q = 2;
    print!("\nVariable q1 = {}\n\n", q);
    {
        let q = 4;
        let t = q-2;
        print!("Variable q2 = {}\n", q);
        print!("Variable t = {}\n\n", t);
    }

    let q = q+1;
    print!("Variable q3 = {}\n\n", q);

//Consts---------------------------------------------------------------------------------------------------------

    //Use conventions
    const SECONDS_IN_MINUTE: u32 = 2;
    print!("Const SECONDS_IN_MINUTE = {}\n\n", SECONDS_IN_MINUTE);
}
