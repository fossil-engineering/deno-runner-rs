use deno_runner::{Builder, Runtime};
use std::collections::HashMap;

#[test]
fn test_import_from_external_lib() {
    // Using re-exported tokio runtime
    let rt = Runtime::new().unwrap();
    let out = rt.block_on(async {
        let custom_code = r#"
            import { isTrue } from "https://deno.land/x/isx@1.0.0-beta.24/mod.ts";

            isTrue(1)
        "#;

        let runner = Builder::new().build();

        let vars = HashMap::from([("a", 1), ]);
        let result = runner.run(custom_code, Some(vars)).await.unwrap();

        result
    });

    assert_eq!(out, "true");
}
