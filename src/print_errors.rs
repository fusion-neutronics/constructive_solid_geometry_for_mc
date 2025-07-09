fn main() {
    println!("Starting error check...");
    #[cfg(feature = "python")]
    {
        use std::fmt::Debug;
        use std::collections::HashMap;
        
        struct Test;
        
        impl Test {
            fn check() {
                use pyo3::prelude::*;
                
                #[pyclass]
                struct TestPyClass {
                    value: i32,
                }
                
                #[pymethods]
                impl TestPyClass {
                    #[new]
                    fn new() -> Self {
                        TestPyClass { value: 42 }
                    }
                }
                
                println!("Python features are enabled");
            }
        }
        
        Test::check();
    }
    
    println!("Done!");
}
