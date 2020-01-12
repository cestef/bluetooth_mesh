use crate::address::UnicastAddress;
use crate::mesh::{IVIndex, IVUpdateFlag, KeyRefreshFlag, U24};

pub struct Flags(u8);
pub struct FSN(bool);
pub struct MD(u8);
pub struct Criteria(u8);
pub struct ReceiveDelay(u8);
pub struct PollTimeout(U24);
pub struct LPNCounter(u16);
pub enum RSSIFactor {
    Factor1 = 0b00,
    Factor2 = 0b01,
    Factor3 = 0b10,
    Factor4 = 0b11,
}
pub enum ReceiveWindowFactor {
    Window1 = 0b00,
    Window2 = 0b01,
    Window3 = 0b10,
    Window4 = 0b11,
}
pub enum MinQueueSizeLog {
    Prohibited = 0b000,
    N2 = 0b001,
    N4 = 0b010,
    N8 = 0b011,
    N16 = 0b100,
    N32 = 0b101,
    N64 = 0b110,
    N128 = 0b111,
}
pub struct FriendPoll {
    fsn: FSN,
}
pub struct FriendUpdate {
    key_refresh_flag: KeyRefreshFlag,
    iv_update_flag: IVUpdateFlag,
    iv_index: IVIndex,
    md: MD,
}
pub struct FriendRequest {
    criteria: Criteria,
    receive_delay: ReceiveDelay,
    poll_timeout: PollTimeout,
    previous_address: UnicastAddress,
    num_elements: u8,
    lpn_counter: LPNCounter,
}
pub struct FriendClear {
    address: UnicastAddress,
    counter: LPNCounter,
}
pub struct FriendClearConfirm {
    address: UnicastAddress,
    counter: LPNCounter,
}
