pub struct ApplicationIdentifier {
    application_name: String,
    application_number: u64,
}

impl ApplicationIdentifier {
    pub fn delimeter() -> String {
        const DELIMETER: &str = "_App";
        return String::from(DELIMETER);
    }

    pub fn env_var_name_application_id() -> String {
        const ENVVARNAME_APPLICATIONID: &str = "Fabric_ApplicationId";
        return String::from(ENVVARNAME_APPLICATIONID);
    }
}
