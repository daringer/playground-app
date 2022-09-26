use ctaphid_dispatch::app::{self as hid, Command as HidCommand, Message};
use ctaphid_dispatch::command::VendorCommand;
use trussed::{
    syscall, try_syscall, Bytes,
    Client as TrussedClient,
    types::{PathBuf, Location}
};


const TEST1: VendorCommand = VendorCommand::H75;
const TEST2: VendorCommand = VendorCommand::H76;
const TEST3: VendorCommand = VendorCommand::H77;
const TEST4: VendorCommand = VendorCommand::H78;
const TEST5: VendorCommand = VendorCommand::H79;

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
            HidCommand::Vendor(TEST3),
            HidCommand::Vendor(TEST4),
            HidCommand::Vendor(TEST5),
        ]
    }

    fn call(&mut self, command: HidCommand, input_data: &Message, response: &mut Message) -> hid::AppResult {
        match command {
            HidCommand::Vendor(TEST1) => {
                debug_now!("test1 - write test data");

                // write some dummy data
                let data = Bytes::from_slice(b"abc-more-data.here").unwrap();
                debug_now!("writing: {:?}", data);
                syscall!(self.trussed.write_file(
                    Location::Internal,
                    PathBuf::from("test-filename"),
                    data,
                    None,
                ));
            }
            HidCommand::Vendor(TEST2) => {
                debug_now!("test2 - reading test data");
                // read some dummy data
                let result =
                    try_syscall!(self.trussed.read_file(Location::Internal, PathBuf::from("test-filename"),));

                if result.is_err() {
                    debug_now!("err loading: {:?}", result.err().unwrap());
                } else {
                    let data = result.unwrap().data;
                    debug_now!("reading: {:?}", data);
                }
            }
            HidCommand::Vendor(TEST3) => {
                debug_now!("test3 - setting pin in ClientContext to 123456");
                let data = Bytes::from_slice(b"123456").unwrap();
                syscall!(self.trussed.set_client_context_pin(data));
            }
            HidCommand::Vendor(TEST4) => {
                debug_now!("test4 - change pin to 123456");
                let data = Bytes::from_slice(b"123456").unwrap();
                let res = try_syscall!(self.trussed.change_pin(data.clone()));

                if res.is_err() {
                    debug_now!("err changing pin: {:?}", res.err().unwrap());
                } else {
                    debug_now!("changed pin to: {:?}", data);
                }
            }
            HidCommand::Vendor(TEST5) => {
                debug_now!("test5 - change pin back to 1234");
                let data = Bytes::from_slice(b"1234").unwrap();
                let res = try_syscall!(self.trussed.change_pin(data.clone()));

                if res.is_err() {
                    debug_now!("err changing pin: {:?}", res.err().unwrap());
                } else {
                    debug_now!("changed pin to: {:?}", data);
                }
            }
            _ => {
                return Err(hid::Error::InvalidCommand);
            }
        }
        Ok(())
    }
}