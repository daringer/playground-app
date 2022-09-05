use ctaphid_dispatch::app::{self as hid, Command as HidCommand, Message};
use ctaphid_dispatch::command::VendorCommand;
use trussed::{
    syscall, try_syscall, Bytes,
    Client as TrussedClient,
    types::{PathBuf, Location}
};


const TEST1: VendorCommand = VendorCommand::H75;
const TEST2: VendorCommand = VendorCommand::H76;


pub struct App<T>
where T: TrussedClient,
{
    trussed: T
}

impl<T> App<T>
where T: TrussedClient
{
    pub fn new(client: T) -> Self {
        Self { trussed: client }
    }

}

impl<T> hid::App for App<T>
where T: TrussedClient
{
    fn commands(&self) -> &'static [HidCommand] {
        &[
            HidCommand::Vendor(TEST1),
            HidCommand::Vendor(TEST2),
        ]
    }

    fn call(&mut self, command: HidCommand, input_data: &Message, response: &mut Message) -> hid::AppResult {
        match command {
            HidCommand::Vendor(TEST1) => {
                debug_now!("test1");

                // write some dummy data
                let data = Bytes::from_slice(b"abc-more-data.here").unwrap();
                syscall!(self.trussed.write_file(
                    Location::Internal,
                    PathBuf::from("test-filename"),
                    data,
                    None,
                ));

                // read some dummy data
                let result =
                    try_syscall!(self.trussed.read_file(Location::Internal, PathBuf::from("test-filename"),));

                if result.is_err() {
                    debug_now!("err loading: {:?}", result.err().unwrap());
                } else {
                    let data = result.unwrap().data;
                    debug_now!("data loaded: {:?}", data);
                }

            }
            HidCommand::Vendor(TEST2) => {
                debug_now!("test2");
            }
            _ => {
                return Err(hid::Error::InvalidCommand);
            }
        }
        Ok(())
    }
}