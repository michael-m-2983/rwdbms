pub trait Server {
    fn update(&self) -> ();
    /**
    * TODO: This fn needs the repo object
    * TODO: static impl needs to: 1. Pull from git 2. Switch to the branch name 3. Copy from repo
    * to the static directory
    */
    fn get_name(&self) -> String;
}

pub struct StaticServer {
    name: String
}
impl Server for StaticServer {
    fn update(&self) -> () {
        println!("Updating {}", self.name);
    }

    fn get_name(&self) -> String {
        return self.name.clone();
    }
}

impl StaticServer {
    pub fn create(name: &str) -> StaticServer {
        StaticServer {
            name: name.to_owned()
        }
    }
}
