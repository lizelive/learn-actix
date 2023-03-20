fn main() {
    println!("Hello, world!");
}



mod schema{
    mod feature{
        use serde::{Deserialize, Serialize};
        schemafy::schemafy!(
            root: DevContainerFeature
            "schema/devContainerFeature.schema.json"
        );
    }
    mod base{
        use serde::{Deserialize, Serialize};
        schemafy::schemafy!(
            root: DevContainerBase // Optional name for the root type (if one exists)
            "schema/devContainer.base.schema.json"
        );
    }
}
