fn main() -> Result<(), Box<dyn std::error::Error>> {
    // use schema::base::*;
    let _json = include_str!("../../example/meta.json");

    // let dev_container: DevContainer = serde_json::from_str(json)?;

    let x = HashableValue::Null(());
    println!("{:#?}", x);
    
    Ok(())
}

use serde::{Deserialize, Serialize};

schemafy::schemafy!(
    root: HashableValue // Optional name for the root type (if one exists)
    "../schema/hashable-value.json"
);


// pub mod schema{
//     // mod feature{
//     //     use serde::{Deserialize, Serialize};
//     //     schemafy::schemafy!(
//     //         root: DevContainerFeature
//     //         "../schema/devContainerFeature.schema.json"
//     //     );
//     // }
//     pub mod base{
//         use serde::{Deserialize, Serialize};
//         schemafy::schemafy!(
//             root: DevContainer // Optional name for the root type (if one exists)
//             "../schema/devContainer.base.schema.json"
//         );
//     }
// }
