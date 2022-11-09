use crossbeam_channel::Receiver;
pub trait EventBus<IE, OE> {
    fn send_event(&self, event: IE) -> Result<(), anyhow::Error>;
    fn receive_events(&self) -> Receiver<OE>;
}
