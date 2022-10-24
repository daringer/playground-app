use ctaphid_dispatch::app::{self as hid, Command as HidCommand, Message};
use ctaphid_dispatch::command::VendorCommand;
use trussed::{
    syscall, try_syscall, Bytes,
    Client as TrussedClient,
    types::{PathBuf, Location, AuthContextID},
};


const TEST1: VendorCommand = VendorCommand::H75;
const TEST2: VendorCommand = VendorCommand::H76;
const TEST3: VendorCommand = VendorCommand::H77;
const TEST4: VendorCommand = VendorCommand::H78;
const TEST5: VendorCommand = VendorCommand::H79;
const TEST6: VendorCommand = VendorCommand::H7A;

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
            HidCommand::Vendor(TEST6),
        ]
    }

    fn call(&mut self, command: HidCommand, input_data: &Message, response: &mut Message) -> hid::AppResult {
        match command {
            HidCommand::Vendor(TEST1) => {
                debug_now!("test1 - pin management");

                // admin pin checks
                debug_now!("------------- admin pin management tests:");
                debug_now!("ADMIN - checking with '123456'");
                let res = syscall!(self.trussed.check_auth_context(AuthContextID::Admin, b"123456"));
                match res.authorized {
                    true => debug_now!("ADMIN - check: success! retries left: {:?}", res.retries_left),
                    false => debug_now!("ADMIN - check: fail! retries left: {:?}", res.retries_left),
                };

                debug_now!("ADMIN - checking with '1234'");
                let res = syscall!(self.trussed.check_auth_context(AuthContextID::Admin, b"1234"));
                match syscall!(self.trussed.check_auth_context(AuthContextID::Admin, b"1234")).authorized {
                    true => debug_now!("ADMIN - check: success! retries left: {:?}", res.retries_left),
                    false => debug_now!("ADMIN - check: fail! retries left: {:?}", res.retries_left),
                };

                debug_now!("ADMIN - set auth context to pin: 123456");
                syscall!(self.trussed.set_auth_context(AuthContextID::Admin, b"123456"));

                match try_syscall!(self.trussed.write_auth_context(AuthContextID::Admin, b"aaaa")) {
                    Ok(_) => debug_now!("ADMIN - changed/wrote auth context state! (to 'aaaa')"),
                    Err(e) => debug_now!("ADMIN - error writing auth context state: {:?}", e),
                };

                debug_now!("ADMIN - checking ADMIN with '123456'");
                let res = syscall!(self.trussed.check_auth_context(AuthContextID::Admin, b"123456"));
                match res.authorized {
                    true => debug_now!("ADMIN - check: success! retries left: {:?}", res.retries_left),
                    false => debug_now!("ADMIN - check: fail! retries left: {:?}", res.retries_left),
                };

                match try_syscall!(self.trussed.write_auth_context(AuthContextID::Admin, b"123456")) {
                    Ok(_) => debug_now!("ADMIN - changed/wrote auth context state! (to '123456')"),
                    Err(e) => debug_now!("ADMIN - error writing auth context state: {:?}", e),
                };

                // user pin checks
                debug_now!("------------- user pin management tests:");
                debug_now!("USER - checking USER with '123456'");
                let res = syscall!(self.trussed.check_auth_context(AuthContextID::User, b"123456"));
                match res.authorized {
                    true => debug_now!("USER - check: success! retries left: {:?}", res.retries_left),
                    false => debug_now!("USER - check: fail! retries left: {:?}", res.retries_left),
                };

                debug_now!("USER - checking with '1234'");
                let res = syscall!(self.trussed.check_auth_context(AuthContextID::User, b"1234"));
                match res.authorized {
                    true => debug_now!("USER - check: success! retries left: {:?}", res.retries_left),
                    false => debug_now!("USER - check: fail! retries left: {:?}", res.retries_left),
                };

                debug_now!("USER - set auth context to pin: 1234");
                syscall!(self.trussed.set_auth_context(AuthContextID::User, b"1234"));

                match try_syscall!(self.trussed.write_auth_context(AuthContextID::User, b"aaaa")) {
                    Ok(_) => debug_now!("USER - changed/wrote auth context state! (to 'aaaa')"),
                    Err(e) => debug_now!("USER - error writing auth context state: {:?}", e),
                };

                debug_now!("USER - checking with '1234'");
                let res = syscall!(self.trussed.check_auth_context(AuthContextID::User, b"1234"));
                match res.authorized {
                    true => debug_now!("USER - check: success! retries left: {:?}", res.retries_left),
                    false => debug_now!("USER - check: fail! retries left: {:?}", res.retries_left),
                };

                match try_syscall!(self.trussed.write_auth_context(AuthContextID::User, b"1234")) {
                    Ok(_) => debug_now!("USER - changed/wrote auth context state! (to '1234')"),
                    Err(e) => debug_now!("USER - error writing auth context state: {:?}", e),
                };


            }
            HidCommand::Vendor(TEST2) => {
                debug_now!("test2 - reading test data");
                // read some dummy data
                /*let result =
                    try_syscall!(self.trussed.read_file(Location::Internal, PathBuf::from("test-filename"),));

                if result.is_err() {
                    debug_now!("err loading: {:?}", result.err().unwrap());
                } else {
                    let data = result.unwrap().data;
                    debug_now!("reading: {:?}", data);
                }*/
            }
            HidCommand::Vendor(TEST3) => {
                debug_now!("test3 - setting pin in ClientContext to 123456");
                /*let data = Bytes::from_slice(b"123456").unwrap();
                syscall!(self.trussed.set_client_context_pin(data));*/
            }
            HidCommand::Vendor(TEST4) => {
                debug_now!("test4 - change pin to 123456");
                /*let data = Bytes::from_slice(b"123456").unwrap();
                let res = try_syscall!(self.trussed.change_pin(data.clone()));

                if res.is_err() {
                    debug_now!("err changing pin: {:?}", res.err().unwrap());
                } else {
                    debug_now!("changed pin to: {:?}", data);
                }*/
            }
            HidCommand::Vendor(TEST5) => {
                debug_now!("test5 - change pin back to 1234");
                /*let data = Bytes::from_slice(b"1234").unwrap();
                let res = try_syscall!(self.trussed.change_pin(data.clone()));

                if res.is_err() {
                    debug_now!("err changing pin: {:?}", res.err().unwrap());
                } else {
                    debug_now!("changed pin to: {:?}", data);
                }*/
            }

            HidCommand::Vendor(TEST6) => {
                debug_now!("test6 - reset pin to 123456");
                /*let data = Bytes::from_slice(b"123456").unwrap();
                let res = try_syscall!(self.trussed.reset_pin(data.clone()));

                if res.is_err() {
                    debug_now!("err resetting pin: {:?}", res.err().unwrap());
                } else {
                    debug_now!("reset pin to: {:?}", data);
                }*/
            }

            _ => {
                return Err(hid::Error::InvalidCommand);
            }
        }
        Ok(())
    }
}