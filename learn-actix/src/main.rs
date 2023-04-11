use serde_json::Value;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use schema::base::*;
    let json = include_str!("../../example/gpu.json");

    let dev_container: DevContainer = serde_json::from_str(json)?;

    println!("{:#?}", dev_container);
    Ok(())
}



pub mod schema{
    // mod feature{
    //     use serde::{Deserialize, Serialize};
    //     schemafy::schemafy!(
    //         root: DevContainerFeature
    //         "../schema/devContainerFeature.schema.json"
    //     );
    // }
    pub mod base{
        use serde::{Deserialize, Serialize};
        schemafy::schemafy!(
            root: DevContainer // Optional name for the root type (if one exists)
            "../schema/devContainer.base.schema.json"
        );
    }
}
