
#[derive(Debug)]
pub struct WoewDbus {
    pub sender: String,
}

impl arg::AppendAll for WoewDbus{
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.sender, i);
    }
}

impl arg::ReadAll for WoewDbus {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(WoewDbus {
            sender: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for WoewDbus {
    const NAME: &'static str = "HelloHappened";
    const INTERFACE: &'static str = "com.example.dbustest";
}

