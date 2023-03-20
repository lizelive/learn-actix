struct DockerCreation {
}




trait Mixin {
    fn id(&self) -> String;

    fn installs_after(&self) -> Vec<String>;

    fn install(&self) -> String;
}

